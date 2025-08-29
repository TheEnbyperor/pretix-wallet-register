use std::path::PathBuf;
use rasn_compiler::OutputMode;
use rasn_compiler::prelude::*;

fn main() {
    println!("cargo:rerun-if-changed=asn1/vasData.asn");
    println!("cargo:rerun-if-changed=asn1/pretixWallet.asn");

    Compiler::<RasnBackend, _>::new()
        .add_asn_by_path(PathBuf::from("asn1/vasData.asn"))
        .set_output_mode(OutputMode::SingleFile(PathBuf::from("./asn1_gen/vas_data.rs")))
        .compile().unwrap();

    Compiler::<RasnBackend, _>::new()
        .add_asn_by_path(PathBuf::from("asn1/pretixWallet.asn"))
        .set_output_mode(OutputMode::SingleFile(PathBuf::from("./asn1_gen/pretix_wallet.rs")))
        .compile().unwrap();
}