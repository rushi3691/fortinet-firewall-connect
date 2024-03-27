use reqwest::header::{HeaderMap, HeaderValue, CONNECTION, USER_AGENT};
use scraper::{Html, Selector};

const FORTI_URL: &str = "http://10.250.209.251:1000/login?05";

pub async fn extract_magic() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0"));
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));

    let resp = client.get(FORTI_URL).headers(headers).send().await?;
    println!("Status: {}", resp.status());
    let body = resp.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse(r#"input[name="magic"]"#).or(Err("Selector not found"))?;
    let magic = document.select(&selector).next().ok_or("Magic not found")?;
    let magic = magic
        .value()
        .attr("value")
        .ok_or("Magic not found")?
        .to_string();
    println!("Magic: {:?}", magic);
    Ok(magic)
}

pub async fn login(
    username: &String,
    password: &String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Logging in!");

    let magic = extract_magic().await?;
    println!("magic {}", magic);

    let params = [
        ("4Tredir", FORTI_URL),
        ("magic", &magic),
        ("username", &username),
        ("password", &password),
    ];

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0"));
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));

    let resp = client
        .post(FORTI_URL)
        .headers(headers)
        .form(&params)
        .send()
        .await?;
    let body = resp.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("script").or(Err("Selector not found"))?;
    let keep_alive_url = document
        .select(&selector)
        .next()
        .ok_or("Keep alive url not found")?
        .inner_html()
        .split("\"")
        .nth(1)
        .ok_or("Keep alive url not found")?
        .to_string();
    println!("{}", keep_alive_url);

    // let session_id = keep_alive_url.split('?').collect::<Vec<&str>>()[1];
    let session_id = keep_alive_url.split("?").nth(1).ok_or("Session id not found")?;
    println!("Logged in with sid: {}", session_id);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_extract_magic() {
        let magic = extract_magic().await;
        assert!(magic.is_ok());
        assert!(magic.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_login_should_not_panic() {
        let username = "2003126".to_owned();
        let password = "80779433".to_owned();
        assert!(login(&username, &password).await.is_ok());
    }

    #[tokio::test]
    async fn test_login_should_panic() {
        let username = "2003126".to_owned();
        let password = "89fsdf".to_owned();
        assert!(login(&username, &password).await.is_err());
    }
}
