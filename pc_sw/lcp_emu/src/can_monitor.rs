use std::{sync::{atomic::{AtomicU32, Ordering}, Arc}, time::Duration};

use futures_util::StreamExt;
use tokio::runtime::Runtime;



pub struct CanMonitor {
    bps_b: Arc<AtomicU32>,
    bps_c: Arc<AtomicU32>,
}

impl CanMonitor {
    pub fn new(can_b_name: &'static str, can_c_name: &'static str, rt: Arc<Runtime>) -> Self {

        let bps_c = Arc::new(AtomicU32::new(0));
        let bps_b = Arc::new(AtomicU32::new(0));

        let bps_c_c = bps_c.clone();
        let bps_b_c = bps_b.clone();

        rt.clone().spawn(async move {
            let mut b = loop {
                match tokio_socketcan::CANSocket::open(can_b_name) {
                    Ok(s) => break s,
                    Err(_) => {
                        continue;
                    }
                }
            };
            let mut c = loop {
                match tokio_socketcan::CANSocket::open(can_c_name) {
                    Ok(s) => break s,
                    Err(_) => {
                        continue;
                    }
                }
            };
            c.filter_accept_all().unwrap();
            b.filter_accept_all().unwrap();

            rt.spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_millis(1000));
                let mut total = 0;
                loop {
                    //tokio::time::sleep(Duration::from_millis(10)).await;
                    tokio::select! {
                        _ = interval.tick() => {
                            bps_c_c.store(total as u32, Ordering::Relaxed);
                            total = 0;
                        },
                        Some(Ok(f)) = c.next() => {
                            total += f.data().len()*8 + 8 + 11;
                        }
                    };
                }
            });

            rt.spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_millis(1000));
                let mut total = 0;
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            bps_b_c.store(total as u32, Ordering::Relaxed);
                            total = 0;
                        },
                        Some(Ok(f)) = b.next() => {
                            total += f.data().len()*8 + 8 + 11;
    
                        }
                    };
                }
            });
        });

        Self {
            bps_b,
            bps_c,
        }
    }
}

impl CanMonitor {
    pub fn data_rate_c(&self) -> u32 {
        self.bps_c.load(Ordering::Relaxed)
    }

    pub fn data_rate_b(&self) -> u32 {
        self.bps_b.load(Ordering::Relaxed)
    }
}