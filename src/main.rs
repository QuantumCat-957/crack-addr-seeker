mod address;
mod config;
mod constant;
mod handle;
mod language;
mod timer;
mod write;
mod xpriv;

use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    mpsc::{self},
    Arc,
};

use clap::Parser as _;

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    // 从命令行参数中读取 `phrase` 和 `password`，如果没有提供则使用默认值
    let cli = config::Cli::parse();

    let phrase = cli.phrase;
    let bip39_pw = cli.password;
    let max_file_size = cli.max_file_size;
    let rotation_interval_secs = cli.rotation_interval_secs;
    let language = cli.language;
    let match_length = cli.match_length;

    let (key, _) = xpriv::phrase_to_master_key(language, &phrase, &bip39_pw)?;
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);
    let generated_count = Arc::new(AtomicUsize::new(0));
    let generated_count_clone = Arc::clone(&generated_count);

    // 创建一个通道，用于发送地址写入任务
    let (tx, rx) = mpsc::channel::<write::AddressRecord>();
    let tx = Arc::new(tx);

    let rotation_interval = std::time::Duration::from_secs(rotation_interval_secs); // 1 hour
                                                                                    // 启动一个线程来处理写入文件任务
    let writer_handle = write::start_writer_thread(rx, max_file_size as u64, rotation_interval);

    // 启动计时器线程，每秒输出生成的地址数
    let timer_handle = timer::start_timer_thread(running_clone, generated_count_clone);

    // 启动多个线程来生成地址
    let eth_handles = handle::process(
        key.clone(),
        address::eth::EthereumAddressGenerator,
        running.clone(),
        generated_count.clone(),
        tx.clone(),
        match_length,
    )?;
    let tron_handles = handle::process(
        key.clone(),
        address::trx::TronAddressGenerator,
        running.clone(),
        generated_count.clone(),
        tx.clone(),
        match_length,
    )?;

    for handle in eth_handles.into_iter().chain(tron_handles) {
        handle.join().expect("Thread failed");
    }

    running.store(false, Ordering::SeqCst);
    drop(tx); // 关闭发送端以终止接收线程
    writer_handle.join().expect("Writer thread failed");
    timer_handle.join().expect("Timer thread failed");

    Ok(())
}
