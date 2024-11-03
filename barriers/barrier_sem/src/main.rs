use std::env;
use std::thread;
use std::sync::Arc;
use std::time::Duration;
use rand::Rng;
use std_semaphore::Semaphore;
use std::sync::atomic::{AtomicI32, Ordering};

struct CyclicBarrier {
    arrived: AtomicI32,
    total_threads: i32,
    waiting: Semaphore,
    leaving: Semaphore,
    mutex: Semaphore,
}
impl CyclicBarrier {
    fn new(total: i32) -> Self {
        Self {
            arrived: AtomicI32::new(0),
            total_threads: total,
            waiting: Semaphore::new(0),
            leaving: Semaphore::new(1),
            mutex: Semaphore::new(1),
        }
    }

    fn barrier(&self, thread_id: i32) {
        self.mutex.acquire();
        self.arrived.fetch_add(1, Ordering::Relaxed); 

        println!("waiting {}", thread_id);

        if self.arrived.load(Ordering::Relaxed) == self.total_threads {
            self.waiting.release();
            self.leaving.acquire();
        }

        self.mutex.release();
        self.waiting.acquire();
        self.waiting.release();

        println!("leaving {}", thread_id);

        self.mutex.acquire();
        self.arrived.fetch_sub(1, Ordering::Relaxed);

        if self.arrived.load(Ordering::Relaxed) == 0 {
            self.waiting.acquire();
            self.leaving.release();
        }

        self.mutex.release();
        self.leaving.acquire();
        self.leaving.release();
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
                barrier_clone.barrier(i as i32);
                thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(0..1000)));
                println!("Thread{} reached barrier", i);
                barrier_clone.barrier(i as i32);
                println!("Thread{} passed barrier", i);
                barrier_clone.barrier(i as i32);
            }
        });
        test_threads.push(thread);
    }

    for thread in test_threads {
        thread.join().unwrap(); 
    }
}
