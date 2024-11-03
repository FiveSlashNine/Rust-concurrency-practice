pub struct ServerProtocol;

impl ServerProtocol {
    pub fn process_request(&self, the_input: &str) -> String {
        println!("Received message from client: {}", the_input);
        let result: String;

        let split_input: Vec<&str> = the_input.splitn(3, ' ').collect();

        if split_input[0] == "!" {
            result = "!".to_string();
        } else if split_input.len() == 3 {
            if split_input[1].parse::<f64>().is_ok() && split_input[2].parse::<f64>().is_ok() {
                let first_num = split_input[1].parse::<f64>().unwrap();
                let second_num = split_input[2].parse::<f64>().unwrap();
                result = self.calc_result(first_num, second_num, split_input[0]);
            } else {
                result = "E Numbers aren't valid".to_string();
            }
        } else {
            result = "E Number of arguments is wrong".to_string();
        }
        println!("Send message to client: {}", result);
        result
    }

    fn calc_result(&self, first_num: f64, second_num: f64, operator: &str) -> String {
        let result: String;
        match operator {
            "+" => result = format!("R {}", first_num + second_num),
            "-" => result = format!("R {}", first_num - second_num),
            "*" => result = format!("R {}", first_num * second_num),
            "/" => {
                if second_num != 0.0 {
                    result = format!("R {}", first_num / second_num);
                } else {
                    if first_num == 0.0 {
                        result = "E Undefined".to_string();
                    } else {
                        result = "R Infinity".to_string();
                    }
                }
            }
            _ => result = "E Operator isn't valid (+,-,*,!)".to_string(),
        }
        result
    }
}
