use clap::Parser;

use crate::language::Language;

#[derive(Parser, Debug)]
#[command(name = "Address Generator")]
#[command(author = "wenjing")]
#[command(about = "Generates Ethereum and TRON addresses.")]
#[command(version = "1.0", long_about = None)]
pub struct Cli {
    #[clap(
        long,
        default_value = "fan swamp loop mesh enact tennis priority artefact canal hour skull joy"
    )]
    pub phrase: String,
    #[clap(long, default_value_t = Language::English)]
    pub language: Language,
    #[clap(long, default_value = "")]
    pub password: String,
    #[clap(long, default_value_t = 104857600)]
    pub max_file_size: u64,
    #[clap(long, default_value_t = 3600)]
    pub rotation_interval_secs: u64,
    #[clap(long, default_value_t = 6)]
    pub eth_match_length: usize,
    #[clap(long, default_value_t = 4)]
    pub tron_match_length: usize,
    #[clap(long, default_value_t = false)]
    pub reverse_index: bool,
}
