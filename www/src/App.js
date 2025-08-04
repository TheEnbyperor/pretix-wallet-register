import BarkoderSDK from "barkoder-wasm";
import React, {useEffect, useState} from "react";
import Spinner from 'react-bootstrap/Spinner';
import * as pretixWallet from "pretix-wallet";
import Error from './Error';
import Notice from './Notice';
import "./App.css";

const BARKODER_LICENSE = "0MHXR8cuvoJT62F-vUCcqMQR74K0988ixUjSf_DnucZlrv_DJTneGfAh1avJBr72P0VecEQGK5JHDH0FmfI_Lp8PdEdFGLDlQzT_axGBusQQWRt4-vYYaAyxrCvqtGWZIVN6jhCiyvQ7fndQ7oDAwhdpufGp1KH2tYFeNfif84DE8anuMEXfTOGUjN3jfEu1";

export default function App() {
    const [isInitialized, setIsInitialized] = useState(false);
    const [isProcessing, setIsProcessing] = useState(false);
    const [error, setError] = useState(null);
    const [notice, setNotice] = useState(null);
    const [clientState, setClientState] = useState(null);
    const [barkoder, setBarkoder] = useState(null);
    const [client, setClient] = useState(null);

    useEffect(() => {
        const initializeBarkoder = async () => {
            const Barkoder = await BarkoderSDK.initialize(BARKODER_LICENSE);
            Barkoder.setEnabledDecoders(
                Barkoder.constants.Decoders.QR,
                Barkoder.constants.Decoders.Aztec,
            );
            Barkoder.setScannerTimeout(0);
            Barkoder.setCloseEnabled(false);
            Barkoder.setCameraPickerEnabled(true)
            Barkoder.setCameraResolution(Barkoder.constants.CameraResolution.FHD);
            Barkoder.setDecodingSpeed(Barkoder.constants.DecodingSpeed.Normal);
            Barkoder.setContinuous(true);
            Barkoder.setDuplicatesDelayMs(1000);
            setBarkoder(Barkoder);
            setIsInitialized(true);
        };
        initializeBarkoder()
            .then(_ => {
            })
            .catch(err => {
                setError(err.toString());
            });
    }, []);

    useEffect(() => {
        const initializeClient = async () => {
            const client = pretixWallet.WalletClient.new((state) => {
                setClientState(state);
            });
            setClient(client);
            await client.setup();
        };
        initializeClient()
            .then(_ => {
            })
            .catch(err => {
                setClientState(pretixWallet.State.NotConfigured);
                setError(err.toString());
            });
    }, []);

    useEffect(() => {
        if (barkoder && client) {
            barkoder.startScanner((result) => {
                if (result.error) {
                    setError(`Barkoder error: ${result.error.message}`);
                } else {
                    setIsProcessing(true);
                    client.barcode_scanned(result.binaryData).then((res) => {
                        setIsProcessing(false);
                        if (res.action === pretixWallet.ScanAction.Registered) {
                            setNotice({
                                title: "Registered",
                                msg: "This device has been successfully registered with Pretix"
                            })
                        } else if (res.action === pretixWallet.ScanAction.Wallet) {
                            let balance = new Intl.NumberFormat(undefined, {
                                style: "currency",
                                currency: res.wallet_currency
                            }).format(res.wallet_balance);
                            setNotice({
                                title: `Wallet ${res.wallet_pan}`,
                                msg: `Balance: ${balance}`
                            })
                        } else {
                            barkoder.setPauseDecoding(false);
                        }
                    }).catch(err => {
                        setIsProcessing(false);
                        setError(err.toString());
                    });
                }
            });
        }
    }, [barkoder, clientState]);

    const clearError = () => {
        setError(null);
        barkoder.setPauseDecoding(false);
    }

    const clearNotice = () => {
        setNotice(null);
        barkoder.setPauseDecoding(false);
    }

    return <div id="app">
        <div className="status-bar">
            {!isInitialized ? <h1>Loading...</h1> : (
                clientState === pretixWallet.State.Initialising ? <h1>Initialising...</h1> :
                    clientState === pretixWallet.State.NotConfigured ? <>
                        <h1>Not configured</h1>
                        <p>Scan a setup barcode to link to Pretix</p>
                    </> :
                        clientState === pretixWallet.State.Ready ? <>
                            <h1>Ready</h1>
                            <p>Scan a ticket barcode</p>
                        </> : null
            )}
        </div>
        {(!isInitialized || isProcessing) ? <div className="loading-spinner">
            <Spinner animation="border" role="status">
                <span className="visually-hidden">Loading...</span>
            </Spinner>
        </div> : null}
        <div className="barkoder">
            <div id="barkoder-container"></div>
        </div>
        <Error error={error} onClose={clearError}/>
        <Notice notice={!!notice ? notice.msg : null} title={!!notice ? notice.title : null} onClose={clearNotice}/>
    </div>
}