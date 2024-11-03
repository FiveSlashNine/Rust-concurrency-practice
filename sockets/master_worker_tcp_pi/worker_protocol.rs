pub struct WorkerProtocol{
    num_workers: i32,
}

impl WorkerProtocol {
    pub fn new(num_workers: i32) -> Self{
        Self {
            num_workers: num_workers,
        }
    }

    pub fn compute(&self, the_input: &str) -> String {
        let splited: Vec<&str> = the_input.split_whitespace().collect();
        let range: i32 = splited[0].parse().unwrap();
        let id: i32 = splited[1].parse().unwrap();

        println!("Worker {} calculates {}", id, range);
        let block = range / self.num_workers;
        let start = id * block;
        let mut stop = start + block;
        if id == self.num_workers - 1 {
            stop = range;
        }
        println!("Worker {} sums from {} to {}", id, start, stop);

        let step = 1.0 / range as f64;
        let mut sum: f64 = 0.0;

        for i in start..stop{
            let x: f64 = (i as f64 + 0.5) * step;
            sum += 4.0 / (1.0 + x * x); 
        }

        // let pi_sum = Arc::new(Mutex::new(0.0));
        // let mut threads = Vec::new();

        // let thread_block = block / self.num_workers;

        // for i in 0..self.num_workers {
        //     let from = i * thread_block + start;
        //     let mut to = i * thread_block + thread_block + start;
        //     if i == self.num_workers - 1 {
        //         to = block + start
        //     }

        //     let pi_sum = Arc::clone(&pi_sum);
        //     let thread = thread::spawn(move || {
        //         let mut temp_sum: f64 = 0.0;
        //         for i in from..to{
        //             let x: f64 = (i as f64 + 0.5) * step;
        //             temp_sum += 4.0 / (1.0 + x * x); 
        //         }
                
        //         let mut pi_sum = pi_sum.lock().unwrap();
        //         *pi_sum+=temp_sum;
        //     });
        //     threads.push(thread);
        // }

        // for thread in threads {
        //     thread.join().unwrap();
        // }

        // let pi_sum = pi_sum.lock().unwrap();
        // let sum = *pi_sum;

        println!("Worker {} result {}", id, sum);

        return sum.to_string();
    }
}
