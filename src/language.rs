#[derive(Debug)]
pub enum Language {
    English,
    ChineseSimplified,
    ChineseTraditional,
    Czech,
    French,
    Italian,
    Japanese,
    Korean,
    Portuguese,
    Spanish,
}

impl Language {
    pub fn from_u8(language_code: u8) -> Result<Self, anyhow::Error> {
        Ok(match language_code {
            1 => Language::English,
            2 => Language::ChineseSimplified,
            3 => Language::ChineseTraditional,
            4 => Language::Czech,
            5 => Language::French,
            6 => Language::Italian,
            7 => Language::Japanese,
            8 => Language::Korean,
            9 => Language::Portuguese,
            10 => Language::Spanish,
            _ => return Err(anyhow::anyhow!("Unknown lang")),
        })
    }
}

#[derive(Debug, Clone)]
pub enum WordlistWrapper {
    English(coins_bip39::English),
    ChineseSimplified(coins_bip39::ChineseSimplified),
    ChineseTraditional(coins_bip39::ChineseTraditional),
    Czech(coins_bip39::Czech),
    French(coins_bip39::French),
    Italian(coins_bip39::Italian),
    Japanese(coins_bip39::Japanese),
    Korean(coins_bip39::Korean),
    Portuguese(coins_bip39::Portuguese),
    Spanish(coins_bip39::Spanish),
}

impl WordlistWrapper {
    pub fn new(lang: u8) -> Result<WordlistWrapper, anyhow::Error> {
        let language = Language::from_u8(lang)?;
        Ok(language.gen_wordlist_wrapper())
    }
}

impl Language {
    pub fn gen_wordlist_wrapper(self) -> WordlistWrapper {
        match self {
            Language::English => WordlistWrapper::English(coins_bip39::English),
            Language::ChineseSimplified => {
                WordlistWrapper::ChineseSimplified(coins_bip39::ChineseSimplified)
            }
            Language::ChineseTraditional => {
                WordlistWrapper::ChineseTraditional(coins_bip39::ChineseTraditional)
            }
            Language::Czech => WordlistWrapper::Czech(coins_bip39::Czech),
            Language::French => WordlistWrapper::French(coins_bip39::French),
            Language::Italian => WordlistWrapper::Italian(coins_bip39::Italian),
            Language::Japanese => WordlistWrapper::Japanese(coins_bip39::Japanese),
            Language::Korean => WordlistWrapper::Korean(coins_bip39::Korean),
            Language::Portuguese => WordlistWrapper::Portuguese(coins_bip39::Portuguese),
            Language::Spanish => WordlistWrapper::Spanish(coins_bip39::Spanish),
        }
    }
}
