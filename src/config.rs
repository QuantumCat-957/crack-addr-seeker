use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(
        long,
        default_value = "fan swamp loop mesh enact tennis priority artefact canal hour skull joy"
    )]
    pub(crate) phrase: String,
    #[arg(long, default_value = "")]
    pub(crate) password: String,
    #[arg(long, default_value_t = 104857600)]
    pub(crate) max_file_size: u64,
    #[arg(long, default_value_t = 3600)]
    pub(crate) rotation_interval_secs: u64,
}
