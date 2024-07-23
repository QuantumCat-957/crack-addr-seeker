use std::fmt::{self};

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
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

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let language_str = match self {
            Language::English => "english",
            Language::ChineseSimplified => "chinese-simplified",
            Language::ChineseTraditional => "chinese-traditional",
            Language::Czech => "czech",
            Language::French => "french",
            Language::Italian => "italian",
            Language::Japanese => "japanese",
            Language::Korean => "korean",
            Language::Portuguese => "portuguese",
            Language::Spanish => "spanish",
        };
        write!(f, "{}", language_str)
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
    pub fn new(language: Language) -> Result<WordlistWrapper, anyhow::Error> {
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
