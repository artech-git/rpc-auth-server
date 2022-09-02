use regex::Regex;

use super::errors::BodyError;

use crate::pb::{
     ProfileOperation, UserCredential, UserDetails, UserInfo, UserProfile, UserStatus,
};

pub mod pb {
    tonic::include_proto!("auth.user.authentication");
}
pub trait ContentValidation {
    fn field_validity(&self) -> Result<(), BodyError>;
}


impl ContentValidation for UserInfo {
    fn field_validity(&self) -> Result<(), BodyError> {
        let v1 = self.name.is_ascii();
        let v2 = check_email(&self.email);
        // let v3 = 
        todo!()
    
    }
}

fn check_email(email: &String) -> bool {
    
    if email.is_empty() {
        return false;
    }

    lazy_static! {//todo evaluate the email constrain too
        static ref RE: Regex = Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)").unwrap();
    }

    RE.is_match(email.as_str())

}