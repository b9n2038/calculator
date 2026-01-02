use proto::calculator_client::CalculatorClient;
use std::error::Error;

pub mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50051";
    let mut client = CalculatorClient::connect(url).await?;

    let body = proto::CalculationRequest { a: 10, b: 2 };
    let request = tonic::Request::new(body);

    let response = client.divide(request).await?;
    // let response = client.add(request).await?;

    println!("Response {:?}", response.get_ref().result);

    Ok(())
}
