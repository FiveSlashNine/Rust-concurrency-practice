use std::thread;
use std::sync::{Arc, atomic::{Ordering, AtomicI32}};
use std::time::Duration;
use rand::Rng;
use std_semaphore::Semaphore;

struct Buffer {
    contents: Vec<AtomicI32>,
    size: usize,
    front: AtomicI32,
    back: AtomicI32,
    counter: AtomicI32,
    mutex: Semaphore,
    buffer_full: Semaphore,
    buffer_empty: Semaphore,
}

impl Buffer {
    fn new(s: usize) -> Self {
        let mut contents = Vec::new();
        for _ in 0..s {
            contents.push(AtomicI32::new(0));
        }        
        Self {
            contents: contents,
            size: s,
            front: AtomicI32::new(0),
            back: AtomicI32::new((s-1)as i32),
            counter: AtomicI32::new(0),
            mutex: Semaphore::new(1), 
            buffer_full: Semaphore::new(0),
            buffer_empty: Semaphore::new(s as isize),
        }
    }

    fn put(&self, data: i32) {
        self.buffer_empty.acquire();
        self.mutex.acquire();

        let value = self.back.load(Ordering::Relaxed)+1;
        self.back.store(value % self.size as i32, Ordering::Relaxed); 
        self.contents[self.back.load(Ordering::Relaxed) as usize].store(data, Ordering::Relaxed);

        self.counter.fetch_add(1, Ordering::Relaxed); 

        println!("Prod {:?} No {} Loc {} Count = {}", thread::current().id(), data, self.back.load(Ordering::Relaxed), self.counter.load(Ordering::Relaxed));
        if self.counter.load(Ordering::Acquire) == self.size as i32 {
            println!("The buffer is full");
        }
        self.mutex.release();
        self.buffer_full.release();
    }

    fn get(&self) -> i32 {
        self.buffer_full.acquire();
        self.mutex.acquire();
        
        let data = self.contents[self.front.load(Ordering::Relaxed) as usize].load(Ordering::Relaxed);
        println!("  Cons {:?} No {} Loc {} Count = {}", thread::current().id(), data, self.front.load(Ordering::Relaxed), self.counter.load(Ordering::Relaxed) - 1);
        
        let value = self.front.load(Ordering::Relaxed)+1;
        self.front.store(value % self.size as i32, Ordering::Relaxed); 
        
        self.counter.fetch_sub(1, Ordering::Relaxed);
        
        if self.counter.load(Ordering::Acquire) == 0 {
            println!("The buffer is empty");
        }
        self.mutex.release();
        self.buffer_empty.release();
        data
    }
}

fn main() {
    let buffer_size = 5;
    let no_iterations = 20;
    let producer_delay = 1;
    let consumer_delay = 100;
    let no_prods = 3;
    let no_cons = 2;
    let mut prod = Vec::with_capacity(no_prods);
    let mut cons = Vec::with_capacity(no_cons);
   
    let buffer = Arc::new(Buffer::new(buffer_size));

    for _ in 0..no_prods {
        let buffer = Arc::clone(&buffer);
        let producer = thread::spawn(move || {
            for i in 0..no_iterations {
                buffer.put(i);
                thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(0..producer_delay)));
            }
        });
        prod.push(producer);
    }

    for _ in 0..no_cons {
        let buffer = Arc::clone(&buffer);
        let consumer = thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(0..consumer_delay)));
                let _ = buffer.get();
            }
        });
        cons.push(consumer);
    }

    for producer in prod {
        producer.join().unwrap();
    }

    for consumer in cons {
        consumer.join().unwrap();
    }
}
