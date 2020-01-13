use std::env;
use std::fs;
use std::io::prelude::*;
use std::time;
use std::thread;
use std::sync::{Arc, Mutex};
use rand::Rng;

#[derive(Debug)]
struct Accumlator {
    sum: i64,
}

fn accumlate(accum: Arc<Mutex<Accumlator>>, filename: &str) {
    let mut file = fs::File::open(filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let num: i64 = contents.trim().parse().unwrap();

    let mut rng = rand::thread_rng();
    let mut accum = accum.lock().unwrap();
    thread::sleep(time::Duration::from_millis(rng.gen_range(0, 20)));
    let cur = accum.sum;
    thread::sleep(time::Duration::from_millis(rng.gen_range(0, 20)));
    accum.sum = cur + num;
}

fn main() {
    let filenames = env::args().into_iter().skip(1);
    let accum = Arc::new(Mutex::new(Accumlator { sum: 0 }));
    let mut handlers = Vec::new();
    for filename in filenames {
        let cloned = accum.clone();
        let handler = thread::spawn(move || {
            accumlate(cloned, &filename);
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.join().unwrap();
    }
    let accum = accum.lock().unwrap();
    println!("The sum of the files is {}.", accum.sum);
}
