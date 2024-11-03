use tonic::{transport::Server, Request, Response, Status};

pub mod calculator {
    tonic::include_proto!("calculator");
}

#[derive(Debug, Default)]
pub struct Calc {}

#[tonic::async_trait]
impl calculator::calculator_server::Calculator for Calc {
    async fn calc_result(&self, request: Request<calculator::CalcRequest>,) -> Result<Response<calculator::CalcResponse>, Status> {
        let req = request.into_inner();

        let result = match req.operator.as_str() {
            "+" => (req.first_num + req.second_num).to_string(),
            "-" => (req.first_num - req.second_num).to_string(),
            "*" => (req.first_num * req.second_num).to_string(),
            "/" => {
                if req.second_num == 0.0 {
                    "Division by zero".to_string()
                } else {
                    (req.first_num / req.second_num).to_string()
                }
            },
            _ => "Invalid operator".to_string(),
        };

        let response = calculator::CalcResponse {
            result: result.to_string(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:1234".parse().unwrap();
    let calculator = Calc::default();

    println!("CalculatorServer listening on {}", addr);

    Server::builder()
        .add_service(calculator::calculator_server::CalculatorServer::new(calculator))
        .serve(addr)
        .await?;

    Ok(())
}
