use aws_sdk_dynamodb::model::AttributeValue;

use super::init::DynamoDBClient;
use crate::constants::KEY_MAP;

pub async fn insert_user(email: &String, secret: &String) -> Option<()> {
    
    let email_av = AttributeValue::S(email.to_owned());
    let secret_av = AttributeValue::S(secret.to_owned());

    let client = &DynamoDBClient.to_owned();

    let table_name: String = KEY_MAP
        .get(&"auth-table".to_string())
        .unwrap_or(&"rpc-table".to_string())
        .to_owned();

    let request = client
        .put_item()
        .table_name(table_name)
        .item("email", email_av)
        .item("value", secret_av);

    match request.send().await {
        Ok(output) => {
            if output.attributes.is_none() {
                println!("insertion succesfull");
                return Some(());
            } else {
                println!("value already present");
                return None;
            }
        }
        Err(_) => {
            println!(" Insertion  invalid");
            return None;
        }
    };
}
