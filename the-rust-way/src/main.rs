use std::env;

mod utils;

use solana_sdk::{
    derivation_path::DerivationPath,
    signature::{keypair_from_seed, keypair_from_seed_and_derivation_path},
    signer::Signer,
};

fn main() {
    let cmd = clap::Command::new("the-rust-way")
        .subcommand_required(true)
        .subcommand(clap::Command::new("derive-keys-please"))
        .subcommand(
            clap::Command::new("list-assets-please").arg(
                clap::Arg::new("ownerAddress")
                    .value_parser(clap::builder::NonEmptyStringValueParser::new()),
            ),
        );

    let matches = cmd.get_matches();
    let _matches = match matches.subcommand() {
        Some(("derive-keys-please", _matches)) => derive_keys_please(),
        Some(("list-assets-please", matches)) => {
            let owner_address: &String = matches
                .get_one("ownerAddress")
                .expect("ownerAddress is required");

            list_assets_please(&owner_address)
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}

fn derive_keys_please() {
    const DEFAULT_SEED_PHRASE: &str =
        "method bronze music hero response market impact sound bone magic unfair salad";
    const DEFAULT_PASS_PHRASE: &str = "";

    let seed_phrase = match env::var_os("TEST_SEED_PHRASE") {
        Some(val) => val.into_string().unwrap(),
        None => DEFAULT_SEED_PHRASE.to_string(),
    };

    let pass_phrase = match env::var_os("TEST_PASS_PHRASE") {
        Some(val) => val.into_string().unwrap(),
        None => DEFAULT_PASS_PHRASE.to_string(),
    };

    let mnemonic = utils::parse_mnemonic(&seed_phrase).unwrap();
    let seed = bip39::Seed::new(&mnemonic, &pass_phrase);
    let root = keypair_from_seed(seed.as_bytes()).unwrap();

    let mut paths: Vec<String> = vec![];
    paths.push("m/44'/501'".to_string());
    for i in 0..10 {
        paths.push(format!("m/44'/501'/{}'", i));
        paths.push(format!("m/44'/501'/{}'/0'", i));
        for j in 0..10 {
            paths.push(format!("m/44'/501'/{}'/0'/{}'", i, j));
        }
    }

    let pairs = paths.iter().map(|path| {
        let derivation_path = DerivationPath::from_absolute_path_str(&path)
            .unwrap_or_else(|e| panic!("invalid derivation path {}: {}", path, e));

        let keypair = keypair_from_seed_and_derivation_path(seed.as_bytes(), Some(derivation_path));

        (path, keypair.unwrap().pubkey())
    });

    println!("{:20} {}", "path", "pubkey");
    println!("{:20} {}", "", root.pubkey());
    for (path, pubkey) in pairs {
        println!("{:20} {}", path, pubkey);
    }
}

fn list_assets_please(owner_address: &str) {
    println!("owner_address {}", owner_address);
}
