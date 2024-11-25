use kposter::auth::x::login::{login, LoginDto};

use kposter::create::post::post;

#[tokio::main]
async fn main() {
    login(LoginDto {
        username: "user".to_string(),
        password: "what".to_string(),
    });
    post("hello".to_string()).await;
}
