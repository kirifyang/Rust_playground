use chrono::Utc;
use futures_util::stream::StreamExt;
use klickhouse::*;

#[derive(Row, Debug, Default)]
pub struct UserData {
    id: Uuid,
    username: String,
    created_at: DateTime,
}

#[tokio::main]
async fn main() {
    // env_logger::Builder()::new()
    //   .parse_env(env_logger::Env::default().default_filter_or("info")).init();
}
