#![feature(strict_overflow_ops)]
use std::sync::{
    atomic::{AtomicBool, AtomicIsize, Ordering},
    Arc,
};

pub mod address;
pub mod config;
pub mod constant;
pub mod handle;
pub mod language;
pub mod timer;
pub mod write;
pub mod xpriv;

pub fn run(
    path: Option<String>,
    language: language::Language,
    reverse_index: bool,
    phrase: &str,
    password: &str,
    eth_match_length: usize,
    trx_match_length: usize,
) -> Result<(), anyhow::Error> {
    let (key, _) = xpriv::phrase_to_master_key(language, &phrase, password)?;
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);
    let generated_count = Arc::new(AtomicIsize::new(0));
    let generated_count_clone = Arc::clone(&generated_count);

    // 创建一个通道，用于发送地址写入任务
    let (tx, rx) = std::sync::mpsc::channel::<write::AddressRecord>();
    let tx = Arc::new(tx);

    // let rotation_interval = std::time::Duration::from_secs(rotation_interval_secs); // 1 hour
    // 启动一个线程来处理写入文件任务
    let writer_handle = write::start_writer_thread(path.clone(), rx);

    // 启动计时器线程，每秒输出生成的地址数
    let timer_handle = timer::start_timer_thread(running_clone, generated_count_clone);

    // 启动多个线程来生成地址
    let eth_handles = handle::process(
        path.clone(),
        key.clone(),
        address::eth::EthereumAddressGenerator,
        running.clone(),
        generated_count.clone(),
        tx.clone(),
        eth_match_length,
        reverse_index,
    )?;
    let tron_handles = handle::process(
        path,
        key.clone(),
        address::trx::TronAddressGenerator,
        running.clone(),
        generated_count.clone(),
        tx.clone(),
        trx_match_length,
        reverse_index,
    )?;

    tracing::info!("111");
    for handle in eth_handles.into_iter().chain(tron_handles) {
        handle.join().expect("Thread failed");
    }

    tracing::info!("aaa");
    // running.store(false, Ordering::SeqCst);
    // drop(tx); // 关闭发送端以终止接收线程
    writer_handle.join().expect("Writer thread failed");
    tracing::info!("bbb");
    timer_handle.join().expect("Timer thread failed");
    tracing::info!("ccc");
    Ok(())
}
