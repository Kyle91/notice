use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct ServerChanMessage {
    title: String,
    desp: String,
}

pub async fn send_notification(
    domain: Option<&str>,
    sendkey: &str,
    title: &str,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // 处理域名：去掉协议前缀
    let base_url = domain
        .filter(|d| !d.is_empty())
        .map(|d| d.trim_start_matches("https://").trim_start_matches("http://"))
        .unwrap_or("sctapi.ftqq.com");

    // 根据域名选择路径格式
    let url = if base_url.contains("push.ft07.com") {
        format!("https://{}/send/{}.send", base_url, sendkey)
    } else {
        format!("https://{}/{}.send", base_url, sendkey)
    };

    let message = ServerChanMessage {
        title: title.to_string(),
        desp: content.to_string(),
    };

    let response = client.post(&url)
        .json(&message)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Server酱请求失败: {}", response.status()).into());
    }

    Ok(())
}

pub async fn test_connection(domain: Option<&str>, sendkey: &str) -> Result<bool, Box<dyn std::error::Error>> {
    send_notification(domain, sendkey, "叮咚 测试", "这是一条测试消息，说明 Server酱 配置成功！").await?;
    Ok(true)
}
