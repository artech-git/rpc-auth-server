// use aws_sdk_dynamodb::model::AttributeValue;
// use bytes::BufMut;
// use bytes::BytesMut;
// use futures::Stream;
// use pb::{EchoRequest, EchoResponse};
// use std::pin::Pin;
// use std::time::Duration;
// use tokio::fs::File;
// use tokio::io::AsyncWriteExt;
// use tokio::sync::mpsc;
// use tokio_stream::wrappers::ReceiverStream;
// use tokio_stream::StreamExt;
// use tonic::Request;

// use tonic::Response;
// use tonic::Status;
// use tonic::Streaming;

// use self::pb::InfoReponse;
// use self::pb::UserCredential;

// pub mod pb {
//     // tonic::include_proto!("grpc.examples.echo");
//     tonic::include_proto!("user.authentication");
// }

// type SendResult<T> = Result<Response<T>, Status>;
// type ResponseStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send>>;

// /*
// Implement the auth server for the suer

// */
// // type InfoResponse<T> = Result<Response<T>, Status>;
// // type ResponseStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send>>;

// #[derive(Default)]
// pub struct AuthServer;

// #[tonic::async_trait]
// impl pb::auth_server::Auth for AuthServer {
//     // type ServerStreamingEchoStream = ResponseStream;

//     async fn register_user(&self, request: Request<UserCredential>) -> EchoResult<InfoReponse> {
//         let body = request.into_inner();
//         let message = body.uid;

//         println!(">> INFO: {:?}", body.field1);

//         match insert_user(&body.email, &body.passwd).await {
//             Some(_) => Ok(Response::new(InfoReponse {
//                 message: "User insertion successfull".to_string(),
//             })),
//             None => Err(Status::aborted("insertion failed")),
//         }
//     }

//     async fn verify_user(&self, request: Request<UserCredential>) -> EchoResult<InfoReponse> {
//         Ok(Response::new(InfoReponse {
//             message: "user verified".to_string(),
//         }))
//     }

//     async fn upload_file(&self, req: Request<Streaming<pb::FileInfo>>) -> EchoResult<InfoReponse> {
//         let mut in_stream = req.into_inner();

//         // let (tx, rx) = mpsc::channel(128);

//         // this spawn here is required if you want to handle connection error.
//         // If we just map `in_stream` and write it back as `out_stream` the `out_stream`
//         // will be drooped when connection error occurs and error will never be propagated
//         // to mapped version of `in_stream`.

//         let mut file = File::create("foo.txt").await.unwrap();

//         let mut buf = tokio::io::BufWriter::new(file);

//         let mut file_bytes = BytesMut::with_capacity(1024);

//         tokio::spawn(async move {
//             while let Some(result) = in_stream.next().await {
//                 match result {
//                     Ok(v) => {
//                         println!("file bytes: {:?}", v.filebytes);
//                         // let mut s = v.filebytes.iter().map(|i| i.parse::<char>().unwrap()).collect::<String>();
//                         file_bytes.put(v.filebytes.as_slice());
//                         // let mut s = std::str::from_utf8(v.filebytes.as_slice()).unwrap();

//                         // println!("string: {:?}", s);
//                         // let mut v_buf = std::io::Cursor::new(v.filebytes.as_slice());.

//                         let a = buf.write_buf(&mut file_bytes).await.unwrap();

//                         // buf.
//                     }
//                     Err(err) => {
//                         if let Some(io_err) = match_for_io_error(&err) {
//                             if io_err.kind() == std::io::ErrorKind::BrokenPipe {
//                                 // here you can handle special case when client
//                                 // disconnected in unexpected way
//                                 eprintln!("\tclient disconnected: broken pipe");
//                                 break;
//                             }
//                         }
//                     }
//                 }
//             }
//             buf.flush().await;
//             println!("\tstream ended");
//         });

//         // echo just write the same data that was received
//         let out_stream = InfoReponse {
//             message: "file recived".to_string(),
//         };

//         Ok(Response::new(out_stream))
//     }
// }
