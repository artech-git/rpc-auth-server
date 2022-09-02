use std::collections::HashSet;

use tonic::metadata::Entry;
use tonic::{Status, async_trait, Request, metadata::KeyAndMutValueRef};
use super::errors::{AuthError, TokenError, HeaderError};

use crate::pb::{
    ProfileOperation, UserCredential, UserDetails, UserInfo, UserProfile, UserStatus,
};
use std::marker::Send;
use std::marker::Sync;

pub mod pb {
    tonic::include_proto!("auth.user.authentication");
}
use http::header::{self, HeaderName};

fn get_valid_headers() -> HashSet<HeaderName> {

    let VALID_HEADERS: HashSet<HeaderName> = [
    header::ACCEPT,
    header::ACCEPT_CHARSET,
    header::ACCEPT_ENCODING,
    header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
    header::ACCESS_CONTROL_ALLOW_ORIGIN,
    header::ACCESS_CONTROL_ALLOW_HEADERS,
    header::AUTHORIZATION,
    header::CONTENT_DISPOSITION,	
    header::CONTENT_ENCODING,	
    header::CONTENT_LANGUAGE,	
    header::CONTENT_LENGTH,	
    header::CONTENT_LOCATION,
    header::CONTENT_TYPE
    ].into();
    
    VALID_HEADERS
}
    
#[async_trait]
pub trait AuthenticateUser {
    async fn validate_headers(&mut self) -> Result<(), HeaderError>;
    // async fn generate_token(&self) -> Result<String, TokenError>;
}

#[async_trait]
pub trait HeaderValidation {
    async fn check_headers(&mut self) -> Result<(), HeaderError>; 
}

#[async_trait]
impl<T: Sync + Send> AuthenticateUser for Request<T> {

    async fn validate_headers(&mut self) -> Result<(), HeaderError> {

        let mut headers = self.metadata_mut();


        println!(" headers ->  {:#?}", headers);

        for header in get_valid_headers().iter() {
            match headers.entry(header.as_str()) {
                Ok(b) => {
                    match b {
                        Entry::Occupied(a) => {
                            let field_string = a.get().to_str().unwrap(); 

                            for j in field_string.chars() {
                                if j.is_alphanumeric() {
                                    continue;
                                }
                                else {
                                    return Err(HeaderError::Unknown);
                                }
                            }
                            return Ok(());
                        }
                        Entry::Vacant(b) => {
                            return Err(HeaderError::FIELD_MISSING(header.to_string()));
                        }
                    }
                }
                Err(e) => {
                    return Err(HeaderError::MissingHeader{expected: header.to_string()});
                }
            }
        }
        
        Ok(())
    }

}

#[async_trait]

impl<T: Sync + Send> HeaderValidation for Request<T> {

    async fn check_headers(&mut self) -> Result<(), HeaderError> {
        
        let mut header = self.metadata_mut();

        for I in get_valid_headers().iter() {
            
            match header.entry((*I).as_str()) {
                Ok(v) => {
                    continue;
                }
                Err(e) => {
                    return Err(HeaderError::FIELD_MISSING(I.to_string()));
                } 
            };
        }        
        
        Ok(())
    }
}


fn generate_secret() -> String {
    let rand_str = rand::Rng::sample_iter(rand::thread_rng(), &rand::distributions::Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();

    rand_str
}

