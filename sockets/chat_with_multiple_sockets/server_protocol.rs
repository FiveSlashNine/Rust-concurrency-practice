pub struct ServerProtocol;

impl ServerProtocol {
    pub fn process_request(&self, the_input: &str) -> String {
        println!("Received message from client: {}", the_input);
        println!("Send message to client: {}", the_input);
        
        return the_input.to_string(); 
    }
}
