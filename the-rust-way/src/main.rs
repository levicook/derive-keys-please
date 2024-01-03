use std::env;

use solana_sdk::{
    derivation_path::DerivationPath, signature::keypair_from_seed_and_derivation_path,
    signer::Signer,
};

fn main() {
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

    let mnemonic = parse_mnemonic(&seed_phrase).unwrap();
    let seed = bip39::Seed::new(&mnemonic, &pass_phrase);

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

    println!("{:20} {}", "derivation path", "pubkey");
    for (path, pubkey) in pairs {
        println!("{:20} {}", path, pubkey);
    }
}

fn parse_mnemonic(phrase: &str) -> Result<bip39::Mnemonic, String> {
    for language in [
        bip39::Language::English,
        bip39::Language::ChineseSimplified,
        bip39::Language::ChineseTraditional,
        bip39::Language::Japanese,
        bip39::Language::Spanish,
        bip39::Language::Korean,
        bip39::Language::French,
        bip39::Language::Italian,
    ] {
        if let Ok(mnemonic) = bip39::Mnemonic::from_phrase(phrase, language) {
            return Ok(mnemonic);
        }
    }
    Err("Invalid mnemonic".to_string())
}
