use std::io;

pub struct ClientProtocol {
    user: io::Stdin,
}

impl ClientProtocol {
    pub fn new() -> Self {
        Self {
            user: io::stdin(),
        }
    }

    pub fn prepare_request(&mut self) -> String {
        println!("Enter operation to send to server:");
        let mut the_output = String::new();
        self.user.read_line(&mut the_output).expect("Failed to read line");
        return the_output.trim().to_string();
    }

    pub fn process_reply(&self, the_input: &str) {
        println!("Message received from server: {}", the_input);
    }
}
