pub(crate) fn parse_mnemonic(phrase: &str) -> Result<bip39::Mnemonic, String> {
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
