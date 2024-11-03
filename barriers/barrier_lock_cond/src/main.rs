use std::env;
use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;
use rand::Rng;
use std::sync::atomic::{AtomicI32, AtomicBool, Ordering};

struct CyclicBarrier {
    arrived: AtomicI32, 
    total_threads: i32,
    waiting: AtomicBool,
    leaving: AtomicBool,
    lock: Mutex<bool>,
    c_wait: Condvar, 
    c_leave: Condvar,
}

impl CyclicBarrier {
    fn new(total: i32) -> Self {
        Self {
            arrived: AtomicI32::new(0),
            total_threads: total,
            waiting: AtomicBool::new(false),
            leaving: AtomicBool::new(true),
            lock: Mutex::new(false),
            c_wait: Condvar::new(),
            c_leave: Condvar::new(),
        }
    }

    fn barrier(&self) {
        let mut waiting = self.lock.lock().unwrap();
        self.arrived.fetch_add(1, Ordering::Relaxed); 

        if self.arrived.load(Ordering::Relaxed) == self.total_threads {
            self.waiting.store(true, Ordering::Relaxed);
            self.leaving.store(false, Ordering::Relaxed);
        }

        while !self.waiting.load(Ordering::Relaxed) {
            waiting = self.c_wait.wait(waiting).unwrap();
        }
        self.c_wait.notify_all();
        drop(waiting); 

        let mut leaving = self.lock.lock().unwrap();
        self.arrived.fetch_sub(1, Ordering::Relaxed);

        if self.arrived.load(Ordering::Relaxed) == 0 {
            self.waiting.store(false, Ordering::Relaxed);
            self.leaving.store(true, Ordering::Relaxed);
        }

        while !self.leaving.load(Ordering::Relaxed) {
            leaving = self.c_leave.wait(leaving).unwrap();
        }
        self.c_leave.notify_all();
        drop(leaving);
    }
}

fn main() {
     let args: Vec<String> = env::args().collect();

     let num_threads = match args.get(1) {
        Some(arg) => match arg.parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Integer argument expected");
                return;
            }
        },
        None => {
            println!("Usage: barrierMain <number of threads>");
            return;
        }
    };

    let mut test_threads = vec![]; 
    let barrier = Arc::new(CyclicBarrier::new(num_threads as i32));

    for i in 0..num_threads {
        let barrier_clone = barrier.clone(); 
        let thread = thread::spawn(move || {
            loop { 
                println!("Thread{} started", i);
                barrier_clone.barrier();
                thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(0..1000)));
                println!("Thread{} reached barrier", i);
                barrier_clone.barrier();
                println!("Thread{} passed barrier", i);
                barrier_clone.barrier();
            }
        });
        test_threads.push(thread);
    }

    for thread in test_threads {
        thread.join().unwrap(); 
    }
}
