use std::time::Instant;
use std::thread;
use std::sync::Mutex;

// seq -> time in ms = 887
// par -> time in ms = 912 
// cyclic -> time in ms = 577
// master worker -> time in ms = 495

const NUM_THREADS:usize = 10;
const SIZE:usize = 100000000;
static mut PRIME: [bool; 100000001] = [true; 100000001];
static mut TASKS_ASSIGNED: Mutex<i32> = Mutex::new(-1);

fn get_task(limit: &usize) -> i32 {
    unsafe{ 
        let mut tasks = TASKS_ASSIGNED.lock().unwrap();
        *tasks+=1;

        if *tasks <= *limit as i32{
            *tasks
        } else {
            -1
        }
    }
}

fn master_worker_lock(limit:usize){ 
    let mut threads = vec![];

    for _ in 2..NUM_THREADS+2 {
        let thread = thread::spawn(move|| {
            let mut element = get_task(&limit);
            while element >=0 as i32 {
                unsafe{
                    if PRIME[element as usize] {
                        for i in (element*element..=SIZE as i32).step_by(element as usize){
                            PRIME[i as usize] = false;
                        }
                    }
                }
                element = get_task(&limit);
            }
        });
        threads.push(thread)
    }

    for thread in threads {
        thread.join().expect("Thread panicked");
    }
}

fn cyclic(limit:usize){
    let mut threads = vec![];
    
    for i in 2..NUM_THREADS+2 {
        let thread = thread::spawn(move|| {
            sieve_with_step(i, limit);
        });
        threads.push(thread)
    }

    for thread in threads {
        thread.join().expect("Thread panicked");
    }
}

fn par(limit:usize){
    let mut threads = vec![];
    
    let block = SIZE / NUM_THREADS;
    for i in 0..NUM_THREADS {
        let from = i * block;
        let mut to = i * block + block;
        if i == (NUM_THREADS - 1) {
            to = limit;
        }

        let thread = thread::spawn(move|| {
            sieve(from, to);
        });
        threads.push(thread)
    }

    for thread in threads {
        thread.join().expect("Thread panicked");
    }
}

fn sieve(from: usize, to:usize) {
    for p in from..=to {
        unsafe{
            if PRIME[p as usize] {
                for i in (p*p..SIZE).step_by(p as usize) {
                    PRIME[i as usize] = false;
                }
            }
        }
    }
}

fn sieve_with_step(from: usize, to:usize) {
    for p in (from..=to).step_by(NUM_THREADS)  {
        unsafe{
            if PRIME[p as usize] {
                for i in (p*p..SIZE).step_by(p) {
                    PRIME[i as usize] = false;
                }
            }
        }
    }
}

fn main() {
    unsafe{
        PRIME[0] = false;
        PRIME[1] = false; 
    }
    
    let start = Instant::now();
    
    let limit: usize = ((SIZE as f64).sqrt() + 1.0) as usize;
   
    //sieve(2, limit);
    //par(limit);
    //cyclic(limit);
    master_worker_lock(limit);

    let endtime = start.elapsed().as_millis();

    unsafe { 
        let count = PRIME.iter().filter(|&&x| x == true).count(); 
        println!("number of PRIMEs {}", count);
    }

    println!("time in ms = {}", endtime);
}
