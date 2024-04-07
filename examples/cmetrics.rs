use std::{thread, time::Duration};

use concurrency::CmapMetrics;
use rand::Rng;

const N: usize = 2;
const M: usize = 4;
fn main() {
    let metrics = CmapMetrics::new();
    println!("metrics: {:?}", metrics);

    for idx in 0..N {
        task_worker(idx, metrics.clone());
    }

    for _ in 0..M {
        request_worker(metrics.clone());
    }

    loop {
        thread::sleep(Duration::from_secs(1));
        println!("metrics: {:?}", metrics);
    }
}

fn task_worker(idx: usize, metrics: CmapMetrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metrics.inc(format!("call.thread.worker.{}", idx)).unwrap();
    });
}

fn request_worker(metrics: CmapMetrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
        let page = rng.gen_range(1..100);
        metrics.inc(format!("req.page.{}", page)).unwrap();
    });
}
