use std::{
    sync::{
        atomic::{AtomicBool, AtomicIsize, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

pub fn start_timer_thread(
    running: Arc<AtomicBool>,
    generated_count: Arc<AtomicIsize>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let start = Instant::now();
        let mut last_count = 0;
        while running.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(1));
            let elapsed = start.elapsed().as_secs();
            if elapsed > 0 {
                let count = generated_count.load(Ordering::SeqCst);
                let per_sec = count - last_count;
                last_count = count;
                tracing::info!("Addresses generated per second: {}", per_sec);
            }
        }
    })
}
