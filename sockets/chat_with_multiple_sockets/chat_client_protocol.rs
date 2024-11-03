use std::io;

pub struct ChatClintProtocol {
    user: io::Stdin,
}

impl ChatClintProtocol {
    pub fn new() -> Self {
        Self {
            user: io::stdin(),
        }
    }

    pub fn send_message(&mut self) -> String {
        println!("Send message, CLOSE for exit:");
        let mut the_output = String::new();
        self.user.read_line(&mut the_output).expect("Failed to read line");
        return the_output.trim().to_string();
    }

    pub fn receive_message(&self, the_input: &str) {
        println!("Received message: {}", the_input);
        println!("Send a reply, CLOSE for exit:");
    }
}
