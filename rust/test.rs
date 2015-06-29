extern crate time;

use std::thread;
use time::PreciseTime;

fn main () {
    
    let start = PreciseTime::now();
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut _x = 0;
            for _ in (0..5_000_001) {
                _x += 1
            }
        })
    }).collect();

    for h in handles {
        h.join().ok().expect("Could not join a thread!");
    }
    
    let end = PreciseTime::now();

    println!("Dur: {}", start.to(end));
}
