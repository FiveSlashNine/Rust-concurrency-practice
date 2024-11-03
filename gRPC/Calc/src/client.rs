use calculator::CalcRequest;

pub mod calculator {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = calculator::calculator_client::CalculatorClient::connect("http://127.0.0.1:1234").await?;

    let requests = vec![
        CalcRequest {
            first_num: 0.2,
            second_num: 0.1,
            operator: "+".to_string(),
        },
        CalcRequest {
            first_num: 2.0,
            second_num: 5.0,
            operator: "-".to_string(),
        },
        CalcRequest {
            first_num: 12.0,
            second_num: 7.0,
            operator: "*".to_string(),
        },
        CalcRequest {
            first_num: 1.0,
            second_num: 0.0,
            operator: "/".to_string(),
        },
    ];

    for req in requests {
        let response = client.calc_result(req).await?;
        println!("Result: {}", response.into_inner().result);
    }

    Ok(())
}
