use clap::Parser as _;
use crack_addr_seeker::config;

// cargo run -- --phrase 'fan swamp loop mesh enact tennis priority artefact canal hour skull joy' --password '123' --max-file-size 104857600 --rotation-interval-secs 3600 --language english --eth-match-length 1 --tron-match-length 1 --reverse-index
fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    // 从命令行参数中读取 `phrase` 和 `password`，如果没有提供则使用默认值
    let cli = config::Cli::parse();

    let phrase = cli.phrase;
    let password = cli.password;
    // let max_file_size = cli.max_file_size;
    // let rotation_interval_secs = cli.rotation_interval_secs;
    let language = cli.language;
    let reverse_index = cli.reverse_index;
    let eth_match_length = cli.eth_match_length;
    let trx_match_length = cli.tron_match_length;
    crack_addr_seeker::run(
        None,
        language,
        reverse_index,
        &phrase,
        &password,
        eth_match_length,
        trx_match_length,
    )?;
    Ok(())
}
