use std::{error::Error, sync::mpsc, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let num = match std::env::args().skip(1).next().map(|v| v.parse::<usize>()) {
        Some(Ok(num)) => num,
        None => 10000,
        Some(Err(e)) => return Err(e.into()),
    };
    println!("Threads: {num}");
    let (tx, rx) = mpsc::channel();
    let start = Instant::now();
    for _ in 0..num {
        let tx = tx.clone();
        std::thread::spawn(move || {
            tx.send(()).unwrap();
        });
    }
    drop(tx);
    for _ in 0..num {
        rx.recv().unwrap();
    }
    println!(
        "Elapsed: {:?} {:.2}/sec",
        start.elapsed(),
        (num as f64) / start.elapsed().as_secs_f64()
    );
    Ok(())
}
