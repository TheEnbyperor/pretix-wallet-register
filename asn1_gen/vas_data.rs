#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod vas_data {
    extern crate alloc;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DataType {
        #[rasn(
            size("0..=4"),
            from(
                "\u{30}", "\u{31}", "\u{32}", "\u{33}", "\u{34}", "\u{35}", "\u{36}", "\u{37}",
                "\u{38}", "\u{39}", "\u{41}", "\u{42}", "\u{43}", "\u{44}", "\u{45}", "\u{46}",
                "\u{47}", "\u{48}", "\u{49}", "\u{4a}", "\u{4b}", "\u{4c}", "\u{4d}", "\u{4e}",
                "\u{4f}", "\u{50}", "\u{51}", "\u{52}", "\u{53}", "\u{54}", "\u{55}", "\u{56}",
                "\u{57}", "\u{58}", "\u{59}", "\u{5a}"
            ),
            identifier = "dataFormat"
        )]
        pub data_format: Ia5String,
        #[rasn(size("0..=64"))]
        pub data: OctetString,
    }
    impl DataType {
        pub fn new(data_format: Ia5String, data: OctetString) -> Self {
            Self { data_format, data }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VASData {
        #[rasn(size("0..=16"))]
        pub data: SequenceOf<DataType>,
    }
    impl VASData {
        pub fn new(data: SequenceOf<DataType>) -> Self {
            Self { data }
        }
    }
}
