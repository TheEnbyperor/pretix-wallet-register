use wasm_bindgen::prelude::*;

mod asn1 {
    include!("../asn1_gen/vas_data.rs");
    include!("../asn1_gen/pretix_wallet.rs");
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct WalletClient {
    pk_db: uic_ticket::sig::PublicKeyDB,
    http_client: reqwest::Client,
    event_callback: js_sys::Function,
    registration: Option<ClientRegistration>,
    vas_registration: Option<ClientVASRegistration>,
}

#[derive(Debug)]
struct ClientRegistration {
    url: reqwest::Url,
    registration: PretixDeviceInfo,
}

#[derive(Debug)]
struct ClientVASRegistration {
    url: reqwest::Url,
    registration: VASDeviceInfo,
}

#[derive(Debug)]
#[wasm_bindgen]
pub enum State {
    Initialising,
    NotConfigured,
    Ready,
}

#[derive(Debug, Default, Copy, Clone)]
#[wasm_bindgen]
pub enum ScanAction {
    #[default]
    Registered,
    Wallet,
}

#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct ScanResult {
    action: ScanAction,
    wallet_pan: String,
    wallet_public_pan: String,
    wallet_balance: String,
    wallet_currency: String,
    wallet_customer: Option<String>,
}

#[wasm_bindgen]
impl ScanResult {
    #[wasm_bindgen(getter)]
    pub fn action(&self) -> ScanAction {
        self.action
    }

    #[wasm_bindgen(getter)]
    pub fn wallet_pan(&self) -> JsValue {
        (&self.wallet_pan).into()
    }

    #[wasm_bindgen(getter)]
    pub fn wallet_public_pan(&self) -> JsValue {
        (&self.wallet_public_pan).into()
    }

    #[wasm_bindgen(getter)]
    pub fn wallet_balance(&self) -> JsValue {
        (&self.wallet_balance).into()
    }

    #[wasm_bindgen(getter)]
    pub fn wallet_currency(&self) -> JsValue {
        (&self.wallet_currency).into()
    }

    #[wasm_bindgen(getter)]
    pub fn wallet_customer(&self) -> JsValue {
        self.wallet_customer.as_ref().into()
    }
}

#[derive(Debug, serde::Deserialize)]
struct PretixDeviceQR {
    handshake_version: u16,
    url: String,
    token: String,
}

#[derive(Debug, serde::Serialize)]
struct PretixDeviceInitialize {
    token: String,
    hardware_brand: &'static str,
    hardware_model: String,
    software_brand: &'static str,
    software_version: &'static str,
}

#[derive(Debug, serde::Serialize)]
struct PretixDeviceUpdate {
    hardware_brand: &'static str,
    hardware_model: String,
    software_brand: &'static str,
    software_version: &'static str,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct PretixDeviceInfo {
    organizer: String,
    device_id: usize,
    unique_serial: String,
    api_token: String,
}

#[derive(Debug, serde::Deserialize)]
struct PretixKeyData {
    keys: Vec<PretixKey>
}

#[derive(Debug, serde::Deserialize)]
struct PretixKey {
    security_provider: String,
    key_id: String,
    public_key: String,
}

#[derive(Debug, serde::Deserialize)]
struct PretixWalletData {
    pan: String,
    public_pan: String,
    balance: String,
    currency: String,
    customer: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct PretixWalletCharge {
    amount: String,
    #[serde(skip_serializing_if="Option::is_none")]
    descriptor: Option<String>
}

#[derive(Debug, serde::Serialize)]
struct VASDeviceInitialize<'a> {
    token: &'a str,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct VASDeviceInfo {
    unique_serial: String,
    api_token: String,
}

#[derive(Debug, serde::Deserialize)]
struct VASWebsocketToken {
    token: String,
}

#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct VASConnection {
    url: String,
    token: String,
}

#[wasm_bindgen]
impl VASConnection {
    #[wasm_bindgen(getter)]
    pub fn url(&self) -> String {
        self.url.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn token(&self) -> String {
        self.token.clone()
    }
}

#[wasm_bindgen]
impl WalletClient {
    pub fn new(event_callback: js_sys::Function) -> Self {
        console_error_panic_hook::set_once();
        wasm_logger::init(wasm_logger::Config::default());

        Self {
            pk_db: uic_ticket::sig::PublicKeyDB::new(),
            http_client: reqwest::Client::new(),
            event_callback,
            registration: None,
            vas_registration: None,
        }
    }

    pub async fn setup(&mut self) -> Result<(), JsValue> {
        self.event_callback.call1(&JsValue::NULL, &JsValue::from(State::Initialising))?;

        let window = web_sys::window().expect("no global `window` exists");
        let ls = window.local_storage()?.ok_or("Local Storage support is required")?;

        match (ls.get_item("pretix_wallet_url")?, ls.get_item("pretix_wallet_registration")?) {
            (Some(url), Some(registration)) => {
                let url = match reqwest::Url::parse(&url) {
                    Ok(url) => url,
                    Err(e) => {
                        log::warn!("Failed to parse Pretix instance URL: {}", e);
                        self.event_callback.call1(&JsValue::NULL, &JsValue::from(State::NotConfigured))?;
                        return Ok(());
                    }
                };
                let registration: PretixDeviceInfo = match serde_json::from_str(&registration) {
                    Ok(registration) => registration,
                    Err(e) => {
                        log::warn!("Failed to parse Pretix registration: {}", e);
                        self.event_callback.call1(&JsValue::NULL, &JsValue::from(State::NotConfigured))?;
                        return Ok(());
                    }
                };

                let res = self.http_client.post(url.join("/api/v1/device/update").unwrap())
                    .header("Authorization", format!("Device {}", registration.api_token))
                    .json(&PretixDeviceUpdate {
                        hardware_brand: "Web",
                        hardware_model: window.navigator().user_agent()?,
                        software_brand: env!("CARGO_PKG_NAME"),
                        software_version: env!("CARGO_PKG_VERSION"),
                    })
                    .send()
                    .await
                    .map_err(|e| JsValue::from(format!("Failed to communicate with Pretix: {}", e)))?;
                if res.status() == reqwest::StatusCode::UNAUTHORIZED {
                    log::info!("Pretix access revoked");
                    self.event_callback.call1(&JsValue::NULL, &JsValue::from(State::NotConfigured))?;
                    return Ok(())
                }
                res.error_for_status()
                    .map_err(|e| JsValue::from(format!("Failed to communicate with Pretix: {}", e)))?;

                self.registration = Some(ClientRegistration {
                    url,
                    registration,
                });
                self.load_keys().await?;
                self.event_callback.call1(&JsValue::NULL, &JsValue::from(State::Ready))?;
            },
            _ => {
                self.event_callback.call1(&JsValue::NULL, &JsValue::from(State::NotConfigured))?;
            }
        }

        self.read_vas_registration(&ls)?;
        Ok(())
    }

    fn read_vas_registration(&mut self, ls: &web_sys::Storage) -> Result<(), JsValue> {
        match (ls.get_item("vas_url")?, ls.get_item("vas_registration")?) {
            (Some(url), Some(registration)) => {
                let url = match reqwest::Url::parse(&url) {
                    Ok(url) => url,
                    Err(e) => {
                        log::warn!("Failed to parse Tappybara instance URL: {}", e);
                        return Ok(());
                    }
                };
                let registration: VASDeviceInfo = match serde_json::from_str(&registration) {
                    Ok(registration) => registration,
                    Err(e) => {
                        log::warn!("Failed to parse Tappybara registration: {}", e);
                        return Ok(());
                    }
                };
                self.vas_registration = Some(ClientVASRegistration {
                    url,
                    registration,
                });
            },
            _ => {}
        }
        Ok(())
    }

    async fn load_keys(&mut self) -> Result<(), JsValue> {
        let registration = self.registration.as_ref().unwrap();
        let keys: PretixKeyData = self.http_client.get(registration.url.join(&format!(
            "/api/v1/organizers/{}/uic_keys/",
            registration.registration.organizer,
        )).unwrap())
            .header("Authorization", format!("Device {}", registration.registration.api_token))
            .send()
            .await
            .map_err(|e| JsValue::from(format!("Failed to communicate with Pretix: {}", e)))?
            .error_for_status()
            .map_err(|e| JsValue::from(format!("Failed to public keys from Pretix: {}", e)))?
            .json().await
            .map_err(|e| JsValue::from(format!("Failed to parse response from Pretix: {}", e)))?;

        for key in keys.keys {
            if let Err(err) = self.pk_db.load_key_pem(&key.security_provider, &key.key_id, &key.public_key) {
                return Err(JsValue::from(format!("Failed to load public key: {}", err)));
            }
        }

        Ok(())
    }

    pub async fn barcode_scanned(&mut self, data: &[u8]) -> Result<ScanResult, JsValue> {
        log::debug!("barcode scanned: {:X?}", data);

        if let Ok(r) = serde_json::from_slice::<PretixDeviceQR>(data) {
            return self.link_device(r).await;
        }

        let Some(registration) = &self.registration else {
            return Err("Not registered".into())
        };

        match uic_ticket::Ticket::parse(data) {
            Ok(t) => {
                log::debug!("UIC ticket: {:02X?}", t);
                if let Err(err) = self.pk_db.verify_ticket(&t) {
                    return match err {
                        uic_ticket::sig::VerificationError::UnknownIssuer => Err("Unknown ticket issuer".into()),
                        uic_ticket::sig::VerificationError::UnknownKey => Err("Unknown signing key".into()),
                        uic_ticket::sig::VerificationError::UnsupportedTicketType => Err("Unsupported ticket type".into()),
                        uic_ticket::sig::VerificationError::InvalidSignature => Err("Invalid/corrupt ticket signature".into()),
                    };
                }

                let wallet = match match t {
                    uic_ticket::Ticket::UicTlbTicket(t) => t.records.records.into_iter().find_map(|r| match r {
                        uic_ticket::tlb_records::Record::PretixWallet(w) => Some(w),
                        _ => None,
                    }),
                    uic_ticket::Ticket::UicDosipasTicket(t) => t.records.into_iter().find_map(|r| match r {
                        uic_ticket::dosipas::Record::PretixWallet(w) => Some(w),
                        _ => None,
                    }),
                    _ => return Err("Unsupported ticket type".into())
                } {
                    Some(w) => w,
                    None => return Err("Ticket does not contain a wallet".into())
                };

                log::debug!("Ticket wallet: {:02X?}", wallet);
                self.get_wallet(registration, &wallet.pan.to_string()).await
            },
            Err(uic_ticket::Error::UnknownBarcodeType) => {
                Err("Unknown barcode type".into())
            }
            Err(e) => {
                Err(format!("Invalid barcode: {}", e).into())
            }
        }
    }

    async fn link_device(&mut self, device: PretixDeviceQR) -> Result<ScanResult, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");

        if device.handshake_version != 1 {
            return Err("Unsupported Pretix link code version".into());
        }
        let base_url = if let Ok(url) = reqwest::Url::parse(&device.url) {
            url
        } else {
            return Err("Invalid Pretix instance URL".into());
        };
        let registration_data: PretixDeviceInfo = self.http_client.post(base_url.join("/api/v1/device/initialize").unwrap())
            .json(&PretixDeviceInitialize {
                token: device.token,
                hardware_brand: "Web",
                hardware_model: window.navigator().user_agent()?,
                software_brand: env!("CARGO_PKG_NAME"),
                software_version: env!("CARGO_PKG_VERSION"),
            })
            .send()
            .await
            .map_err(|e| JsValue::from(format!("Failed to communicate with Pretix: {}", e)))?
            .error_for_status()
            .map_err(|e| JsValue::from(format!("Failed to register with Pretix: {}", e)))?
            .json().await
            .map_err(|e| JsValue::from(format!("Failed to parse response from Pretix: {}", e)))?;

        let ls = window.local_storage()?.ok_or("Local Storage support is required")?;
        ls.set_item("pretix_wallet_url", base_url.as_str())?;
        ls.set_item("pretix_wallet_registration", &serde_json::to_string(&registration_data).unwrap())?;

        self.registration = Some(ClientRegistration {
            url: base_url,
            registration: registration_data,
        });
        self.load_keys().await?;
        self.event_callback.call1(&JsValue::NULL, &JsValue::from(State::Ready))?;
        Ok(ScanResult {
            action: ScanAction::Registered,
            ..Default::default()
        })
    }

    pub async fn link_vas_device(&mut self, server_url: &str, token: &str) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");

        let base_url = if let Ok(url) = reqwest::Url::parse(&server_url) {
            url
        } else {
            return Err("Invalid Tappybara instance URL".into());
        };
        let registration_data: VASDeviceInfo = self.http_client.post(base_url.join("/device/initialize").unwrap())
            .json(&VASDeviceInitialize {
                token,
            })
            .send()
            .await
            .map_err(|e| JsValue::from(format!("Failed to communicate with Tappybara: {}", e)))?
            .error_for_status()
            .map_err(|e| JsValue::from(format!("Failed to register with Tappybara: {}", e)))?
            .json().await
            .map_err(|e| JsValue::from(format!("Failed to parse response from Tappybara: {}", e)))?;

        let ls = window.local_storage()?.ok_or("Local Storage support is required")?;
        ls.set_item("vas_url", base_url.as_str())?;
        ls.set_item("vas_registration", &serde_json::to_string(&registration_data).unwrap())?;

        self.vas_registration = Some(ClientVASRegistration {
            url: base_url,
            registration: registration_data,
        });
        Ok(())
    }

    pub async fn charge_wallet(&mut self, pan: String, amount: String, descriptor: Option<String>) -> Result<(), JsValue> {
        let registration = match &self.registration {
            Some(r) => r,
            None => return Err("Not registered".into())
        };

        let res = self.http_client.post(registration.url.join(&format!(
            "/api/v1/organizers/{}/wallets/{}/charge/",
            registration.registration.organizer,
            pan,
        )).unwrap())
            .header("Authorization", format!("Device {}", registration.registration.api_token))
            .json(&PretixWalletCharge {
                amount,
                descriptor,
            })
            .send()
            .await
            .map_err(|e| JsValue::from(format!("Failed to communicate with Pretix: {}", e)))?;
        if res.status() == reqwest::StatusCode::PAYMENT_REQUIRED {
            return Err("Insufficient balance on wallet".into())
        }
        res.error_for_status()
            .map_err(|e| JsValue::from(format!("Failed to charge wallet: {}", e)))?;

        Ok(())
    }

    pub async fn get_vas_connection(&self) -> Result<Option<VASConnection>, JsValue> {
        let Some(reg) = &self.vas_registration else {
            return Ok(None);
        };

        let res: VASWebsocketToken = self.http_client.get(reg.url.join("/device/websocket_token").unwrap())
            .header("Authorization", format!("Device {}", reg.registration.api_token))
            .send()
            .await
            .map_err(|e| JsValue::from(format!("Failed to communicate with Tappybara: {}", e)))?
            .error_for_status()
            .map_err(|e| JsValue::from(format!("Failed to register with Tappybara: {}", e)))?
            .json().await
            .map_err(|e| JsValue::from(format!("Failed to parse response from Tappybara: {}", e)))?;

        Ok(Some(VASConnection {
            url: reg.url.join("/device/ws").unwrap().to_string(),
            token: res.token,
        }))
    }

    pub async fn tap_redemption(&self, data: &str) -> Result<ScanResult, JsValue> {
        let Ok(data) = base85::decode(data) else {
            return Err("Invalid tap".into())
        };

        let Some(registration) = &self.registration else {
            return Err("Not registered".into())
        };

        let data = rasn::uper::decode::<asn1::vas_data::VASData>(&data)
            .map_err(|e| JsValue::from(format!("Invalid tap: {}", e)))?;

        log::debug!("Tap data: {:02X?}", data);

        let Some(wallet_record) = data.data.into_iter().filter(|d| d.data_format.to_string() == "W").next() else {
            return Err("Tap does not contain a wallet".into());
        };

        let wallet = rasn::uper::decode::<asn1::pretix_wallet::VASPretixWallet>(&wallet_record.data)
            .map_err(|e| JsValue::from(format!("Invalid tap: {}", e)))?;

        log::debug!("Tap wallet: {:02X?}", wallet);
        self.get_wallet(registration, &wallet.pan.to_string()).await
    }

    async fn get_wallet(&self, registration: &ClientRegistration, pan: &str) -> Result<ScanResult, JsValue> {
        let wallet_data: PretixWalletData = self.http_client.get(registration.url.join(&format!(
            "/api/v1/organizers/{}/wallets/{}/",
            registration.registration.organizer,
            pan,
        )).unwrap())
            .header("Authorization", format!("Device {}", registration.registration.api_token))
            .send()
            .await
            .map_err(|e| JsValue::from(format!("Failed to communicate with Pretix: {}", e)))?
            .error_for_status()
            .map_err(|e| JsValue::from(format!("Failed to retrieve wallet from Pretix: {}", e)))?
            .json().await
            .map_err(|e| JsValue::from(format!("Failed to parse response from Pretix: {}", e)))?;

        log::debug!("Wallet data: {:02X?}", wallet_data);

        Ok(ScanResult {
            action: ScanAction::Wallet,
            wallet_pan: wallet_data.pan,
            wallet_public_pan: wallet_data.public_pan,
            wallet_balance: wallet_data.balance,
            wallet_currency: wallet_data.currency,
            wallet_customer: wallet_data.customer,
        })
    }
}