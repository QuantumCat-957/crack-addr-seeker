use clap::Parser as _;
use crack_addr_seeker::config;

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    // 从命令行参数中读取 `phrase` 和 `password`，如果没有提供则使用默认值
    let cli = config::Cli::parse();

    let phrase = cli.phrase;
    let password = cli.password;
    // let max_file_size = cli.max_file_size;
    // let rotation_interval_secs = cli.rotation_interval_secs;
    let language = cli.language;
    let eth_match_length = cli.eth_match_length;
    let trx_match_length = cli.tron_match_length;

    crack_addr_seeker::run(
        language,
        &phrase,
        &password,
        eth_match_length,
        trx_match_length,
    )?;
    Ok(())
}
