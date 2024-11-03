use std::sync::Arc;
use pi::Pi;

pub struct MasterProtocol{
    pi: Arc<Pi>,
    my_id: i32
}

impl MasterProtocol {
    pub fn new(pi: Arc<Pi>, id: i32) -> Self{
        Self {
            pi: pi,
            my_id: id
        }
    }

    pub fn prepare_request(&self) -> String {
        return self.pi.print_init() + " " + &self.my_id.to_string();
    } 
    
    pub fn process_reply(&self, the_input: &str) {
       self.pi.add_to(the_input.parse().unwrap());
    }

}
