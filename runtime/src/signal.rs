use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;
use std::thread;
use log::{info, error};

// 全局退出标志
static GLOBAL_EXIT_FLAG: AtomicBool = AtomicBool::new(false);
static INIT: Once = Once::new();

// 获取全局退出标志的引用
pub fn should_exit() -> bool {
    GLOBAL_EXIT_FLAG.load(Ordering::SeqCst)
}

// 初始化信号处理
pub fn init_signal_handler() {
    INIT.call_once(|| {
        let mut signals = match Signals::new(&[SIGINT, SIGTERM]) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to create signal handler: {}", e);
                return;
            }
        };

        thread::spawn(move || {
            for sig in signals.forever() {
                match sig {
                    SIGINT => {
                        info!("Received SIGINT, initiating graceful shutdown...");
                        GLOBAL_EXIT_FLAG.store(true, Ordering::SeqCst);
                    }
                    SIGTERM => {
                        info!("Received SIGTERM, initiating graceful shutdown...");
                        GLOBAL_EXIT_FLAG.store(true, Ordering::SeqCst);
                    }
                    _ => unreachable!(),
                }
            }
        });
    });
} 