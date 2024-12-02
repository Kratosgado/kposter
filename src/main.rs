use headless_chrome::{Browser, LaunchOptions};
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

fn launch_browser() -> Result<Browser, Box<dyn std::error::Error>> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(true)
            .build()
            .unwrap(),
    );
    Ok(browser)
}

async fn login_twitter(username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    // launch browser
    let browser = launch_browser()?;
    let tab = browser.new_tab()?;

    // open x login page
    tab.navigate_to("https://web.facebook.com/login.php/?_rdc=1&_rdr")?
        .await?;
    tab.wait_until_navigated()?;
}
