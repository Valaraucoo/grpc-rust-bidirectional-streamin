use std::error::Error;
use tonic::Request;

use calculator::{calculator_client::CalculatorClient, CalculatorRequest};

pub mod calculator {
    tonic::include_proto!("calculator");
}

async fn multiply(
    client: &mut CalculatorClient<tonic::transport::Channel>,
) -> Result<(), Box<dyn Error>> {
    let outbound = async_stream::stream! {
        for val in 0..10 {
            yield CalculatorRequest { value: val };
        }
    };

    let response = client.multiply(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    while let Some(val) = inbound.message().await? {
        println!("result = {:?}", val.result);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "http://[::1]:8080".to_string();
    let mut client = CalculatorClient::connect(addr).await?;

    multiply(&mut client).await?;

    Ok(())
}
