use html_parser::Dom;
use reqwest::get;
#[tokio::main]
pub async fn dom(url: &str) -> Result<Dom, Box<dyn std::error::Error>> {
    let html = get(url).await?.text().await?;
    let dom = Dom::parse(&html)?;
    Ok(dom)
}
