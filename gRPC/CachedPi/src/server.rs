mod cached_pi;

use cached_pi::CachedPi;
use tonic::{transport::Server, Request, Response, Status};
use calcpi::calc_pi_server::{CalcPi, CalcPiServer};
use calcpi::{CalcPiRequest, CalcPiResponse};
use std::sync::Arc;

pub mod calcpi {
    tonic::include_proto!("calcpi");
}

pub struct MyCalcPi {
    cache: Arc<CachedPi>,
}

#[tonic::async_trait]
impl CalcPi for MyCalcPi {
    async fn calc_pi(&self, request: Request<CalcPiRequest>,) -> Result<Response<CalcPiResponse>, Status> {
        let num = request.into_inner().num;

        if let Ok(num_steps) = num.parse() {
            if self.cache.in_cache(&num) {
                if let Some(pi) = self.cache.get_pi(&num) {
                    return Ok(Response::new(CalcPiResponse {
                        result: pi,
                    }));
                }
            }

            let result = self.cache.calc_pi(num_steps);
            self.cache.put(num.clone(), result.clone());

            return Ok(Response::new(CalcPiResponse { result }));
        } else {
            return Ok(Response::new(CalcPiResponse {result: "Couldn't parse the number of steps".to_string(), }));
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:1234".parse().unwrap();
    let calc_pi = MyCalcPi {
        cache: Arc::new(CachedPi::new()),
    };

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(CalcPiServer::new(calc_pi))
        .serve(addr)
        .await?;

    Ok(())
}
