use std::sync::Mutex;

pub struct Pi {
    sum: Mutex<f64>,
    step: f64,
    number_of_steps: i32,
}

impl Pi {
    pub fn new(init: i32) -> Self {
        Pi {
            number_of_steps: init,
            sum: Mutex::new(0.0),
            step: 1.0 / init as f64,
        }
    }

    pub fn add_to(&self, to_add: f64) {
        let mut sum = self.sum.lock().unwrap();
        *sum += to_add;
    }

    pub fn print_pi(&self) {
        let sum = self.sum.lock().unwrap();
        println!("Pi = {}", self.step * *sum);
    }

    pub fn print_init(&self) -> String {
       return self.number_of_steps.to_string();
    }
}

