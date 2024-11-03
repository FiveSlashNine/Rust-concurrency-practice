use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::f64::consts::PI;

fn recursive_threads(from: i64, to: i64, limit: i64, step: f64) -> f64 {
    let workload = to - from;
    if workload <= limit {
        let mut sum: f64 = 0.0;
        for i in from..to {
            let x: f64 = (i as f64 + 0.5) * step;
            sum += 4.0 / (1.0 + x * x);
        }
        return sum;
    } else {
        let mid = from + workload / 2;
        let left_sum = Arc::new(Mutex::new(0.0));

        let left_thread = {
            let left_sum = Arc::clone(&left_sum); 
            thread::spawn(move || {
                let sum = recursive_threads(from, mid, limit, step);
                let mut left_sum = left_sum.lock().unwrap();
                *left_sum = sum;
            })
        };

        let right_sum = recursive_threads(mid, to, limit, step);
        left_thread.join().unwrap();

        let left_sum = left_sum.lock().unwrap();

        return *left_sum + right_sum;
    }
}

fn main() {
    let num_steps = 100000000;
    let limit = 100000000/4;

    let start = Instant::now();
    let step: f64 = 1.0 / num_steps as f64;

    let sum = recursive_threads(0, num_steps, limit, step);

    let pi: f64 = sum * step;
    let endtime = start.elapsed().as_secs_f64();

    println!("Recursive program results with {} steps", num_steps);
    println!("computed pi = {}", pi);
    println!("difference between estimated pi and Math.PI = {}", (pi - PI).abs());
    println!("time to compute = {} seconds", endtime);
}
