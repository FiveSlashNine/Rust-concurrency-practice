use std::collections::HashMap;
use std::sync::Mutex;

pub struct CachedPi {
    cached_pi: Mutex<HashMap<String, String>>,
}

impl CachedPi {
    pub fn new() -> Self {
        CachedPi {
            cached_pi: Mutex::new(HashMap::new()),
        }
    }

    pub fn put(&self, iterations: String, pi: String) {
        let mut cached_pi = self.cached_pi.lock().unwrap();
        cached_pi.insert(iterations, pi);
    }

    pub fn in_cache(&self, iterations: &str) -> bool {
        let cached_pi = self.cached_pi.lock().unwrap();
        cached_pi.contains_key(iterations)
    }

    pub fn get_pi(&self, iterations: &str) -> Option<String> {
        let cached_pi = self.cached_pi.lock().unwrap();
        cached_pi.get(iterations).cloned()
    }

    pub fn calc_pi(&self, num_steps: i64) -> String {
        let mut sum: f64 = 0.0;

        let step: f64 = 1.0 / num_steps as f64;
        for i in 0..num_steps {
            let x = (0.5 + i as f64) * step;
            sum += 4.0/(1.0+x*x);
        }
        
        return (sum*step).to_string();
    }
}

