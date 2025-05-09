use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub mod services {
    tonic::include_proto!("services");
}

use services::{
    payment_service_server::{PaymentService, PaymentServiceServer},
    transaction_service_server::{TransactionService, TransactionServiceServer},
    chat_service_server::{ChatService, ChatServiceServer},
    PaymentRequest, PaymentResponse, TransactionRequest, TransactionResponse, ChatMessage,
};

#[derive(Default)]
pub struct MyPaymentService {}

#[tonic::async_trait]
impl PaymentService for MyPaymentService {
    async fn process_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Received payment request: {:?}", request);
        Ok(Response::new(PaymentResponse { success: true }))
    }
}

#[derive(Default)]
pub struct MyTransactionService {}

#[tonic::async_trait]
impl TransactionService for MyTransactionService {
    type GetTransactionHistoryStream = ReceiverStream<Result<TransactionResponse, Status>>;

    async fn get_transaction_history(
        &self,
        _request: Request<TransactionRequest>,
    ) -> Result<Response<Self::GetTransactionHistoryStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        
        tokio::spawn(async move {
            for i in 0..30 {
                tx.send(Ok(TransactionResponse {
                    transaction_id: format!("trans_{}", i),
                    status: "Completed".to_string(),
                    amount: 100.0,
                    timestamp: "2022-01-01T12:00:00Z".to_string(),
                })).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[derive(Default)]
pub struct MyChatService {}

#[tonic::async_trait]
impl ChatService for MyChatService {
    type ChatStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn chat(
        &self,
        request: Request<tonic::Streaming<ChatMessage>>,
    ) -> Result<Response<Self::ChatStream>, Status> {
        let (tx, rx) = mpsc::channel(10);
        let mut stream = request.into_inner();

        tokio::spawn(async move {
            while let Some(message) = stream.message().await.unwrap() {
                let reply = ChatMessage {
                    user_id: message.user_id.clone(),
                    message: format!("CS: Received '{}'", message.message),
                };
                tx.send(Ok(reply)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let payment_service = MyPaymentService::default();
    let transaction_service = MyTransactionService::default();
    let chat_service = MyChatService::default();

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .add_service(TransactionServiceServer::new(transaction_service))
        .add_service(ChatServiceServer::new(chat_service))
        .serve(addr)
        .await?;

    Ok(())
}