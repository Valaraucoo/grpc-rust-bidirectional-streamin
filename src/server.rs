use std::pin::Pin;

use futures::Stream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use calculator::{
    calculator_server::{Calculator, CalculatorServer},
    CalculatorRequest, CalculatorResponse,
};

pub mod calculator {
    tonic::include_proto!("calculator");
}

#[derive(Debug, Default)]
pub struct Service {}

#[tonic::async_trait]
impl Calculator for Service {
    type MultiplyStream =
        Pin<Box<dyn Stream<Item = Result<CalculatorResponse, Status>> + Send + 'static>>;

    async fn multiply(
        &self,
        request: Request<tonic::Streaming<CalculatorRequest>>,
    ) -> Result<Response<Self::MultiplyStream>, Status> {
        let mut stream = request.into_inner();

        println!("Started streaming");

        let output = async_stream::try_stream! {
            while let Some(request) = stream.message().await? {
                println!("Got request: {:?}", request);
                let result = request.value * 2;
                println!("Sending response: {:?}", result);
                yield CalculatorResponse { result }.clone();
            }
        };
        println!("Finished streaming");
        Ok(Response::new(Box::pin(output) as Self::MultiplyStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();

    println!("Starting server on {} 🚀", address);

    let service = Service::default();
    Server::builder()
        .add_service(CalculatorServer::new(service))
        .serve(address)
        .await?;

    Ok(())
}
