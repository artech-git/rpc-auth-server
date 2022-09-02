use aws_sdk_dynamodb::model::AttributeValue;
use bytes::BufMut;
use bytes::BytesMut;
use futures::Stream;
use uuid::Uuid;

use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
// use serde_bytes::Serialize;
use std::pin::Pin;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::Request;

use tonic::Response;
use tonic::Status;
use tonic::Streaming;

use image::EncodableLayout;
use std::error::Error;


use crate::constants::KEY_MAP;
use crate::db::DynamoDBClient;

use crate::pb::{
     ProfileOperation, UserCredential, UserDetails, UserInfo, UserProfile, UserStatus, UserSessionDetails
};

pub mod pb {
    tonic::include_proto!("auth.user.authentication");
}

type SendResult<T> = Result<Response<T>, Status>;
type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

/*
Implement the auth server for the suer

*/
// type InfoResponse<T> = Result<Response<T>, Status>;
// type ResponseStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send>>;

#[derive(Default)]
pub struct AuthenticationServer;


use crate::validations::headers::AuthenticateUser;
use crate::validations::headers::HeaderValidation;

#[tonic::async_trait]
impl pb::authentication_server::Authentication for AuthenticationServer {

    async fn register_user(&self, mut request: Request<UserInfo>) -> SendResult<UserStatus> {
        
        if let Err(e) = (request.validate_headers()).await {
                let src: String = e.source().unwrap().to_string();
                return Err( Status::cancelled(src) );
        }

        return Ok(Response::new(UserStatus{ status: "accept".to_string(), msg: "some msg".to_string()}));
    }
    
    async fn login_user(&self, mut request: Request<UserCredential>) -> SendResult<UserSessionDetails> {
        Ok(Response::new(UserSessionDetails {
            uid: "fwaeifn8fh238rhsdfje".to_string(),
            session_token: "helfoij28fh38rjd83hf".to_string(),
        }))
    }

    async fn get_user_info(&self, mut request: Request<UserProfile>) -> SendResult<UserDetails> {

        
        if let Err(e) = (request.validate_headers()).await {
            let src: String = e.source().unwrap().to_string();
            return Err( Status::cancelled(src) );
        }
        
        let body = request.into_inner();
        
        // match body.section {
        //     ProfileOperation::ProfileInfo => {}
        //     ProfileOperation::CartItems =>{}
        //     ProfileOperation::Settings => {}
        //     ProfileOperation::Wishlist => {}
        //     ProfileOperation::Notifications => {}
        //     ProfileOperation::Orders => {}
        // }
        return Ok( Response::new(
            UserDetails{ status: 
                "get_user_info".to_string(), 
                msg: "working".to_string()}
            ));
    }
    
}
