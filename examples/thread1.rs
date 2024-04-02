use std::{sync::mpsc, thread, time::Duration};

use anyhow::{anyhow, Result};

const NUM_PRODUCERS: usize = 4;
#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        std::thread::spawn(move || producer(i, tx));
    }
    drop(tx);
    let consumer = std::thread::spawn(move || {
        for msg in rx {
            println!("Received: {:?}", msg);
        }
        println!("All messages received");
        42
    });
    let secret = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    print!("The secret is: {}", secret);
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        // random exit the producer
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exit", idx);
            break;
        }
    }
    // more things to do
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
