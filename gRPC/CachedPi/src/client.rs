use calcpi::calc_pi_client::CalcPiClient;
use calcpi::CalcPiRequest;
use std::io::{self};

pub mod calcpi {
    tonic::include_proto!("calcpi");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CalcPiClient::connect("http://127.0.0.1:1234").await?;

    let stdin = io::stdin();
    
    loop {
        println!("Type an amount: ");
        
        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let input = input.trim();
        
        if input == "-1" {
            break;
        }
        
        let request = tonic::Request::new(CalcPiRequest {
            num: input.to_string(),
        });

        let response = client.calc_pi(request).await?;
        println!("pi = {}", response.into_inner().result);
    }
    
    Ok(())
}
