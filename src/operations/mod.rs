pub const T: i32 = 1;

pub mod db_insertion;
pub mod init;
pub mod file_upload;

pub use db_insertion::insert_user;
pub use init::DynamoDBClient;
