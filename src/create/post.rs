use crate::constants::x::xurl;
use reqwest::Client;

pub async fn post(text: String) {
    let client = Client::new();
    let res = client.post(xurl).body(text).send().await;
    println!("{:?}", res);
}
