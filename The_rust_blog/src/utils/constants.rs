use std::env;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref SECRET: String = set_secret_key();
    pub static ref MAX_FILE_SIZE: u64 = set_max_file_size();
}

fn set_address() -> String{
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap_or("127.0.0.1".to_string())
}

fn set_port() -> u16{
    dotenv::dotenv().ok();
    env::var("PORT")
    .unwrap_or("8080".to_owned())
    .parse::<u16>()
    .expect("Cannot parse port number")
}

fn set_database_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").expect("DB not found")
}

fn set_secret_key() -> String {
    dotenv::dotenv().ok();
    env::var("SECRET").unwrap_or("SECRET".to_string())
}

fn set_max_file_size() -> u64{
    dotenv::dotenv().ok();
    env::var("MAX_FILE_SIZE")
    .unwrap_or("10485760".to_owned())
    .parse::<u64>()
    .expect("Can't parse the file")
}

