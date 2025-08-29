#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod pretix_wallet {
    extern crate alloc;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PretixWallet {
        #[rasn(size("10..=19"), from("\u{30}..=\u{39}"))]
        pub pan: Ia5String,
        #[rasn(
            value("0.."),
            from(
                "\u{2d}",
                "\u{2e}",
                "\u{30}..=\u{39}",
                "\u{41}..=\u{5a}",
                "\u{61}..=\u{7a}"
            )
        )]
        pub issuer: Ia5String,
    }
    impl PretixWallet {
        pub fn new(pan: Ia5String, issuer: Ia5String) -> Self {
            Self { pan, issuer }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VASPretixWallet {
        #[rasn(size("10..=19"), from("\u{30}..=\u{39}"))]
        pub pan: Ia5String,
    }
    impl VASPretixWallet {
        pub fn new(pan: Ia5String) -> Self {
            Self { pan }
        }
    }
}
