use serde::{Deserialize, Serialize};

use crate::config::{self, PlatformType};

#[derive(Serialize, Deserialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

pub async fn translate(
    text: String,
    target_lang: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let (api_url, api_key, model_name, platform) = {
        let config = config::CONFIG.get().unwrap().lock().unwrap();
        (
            config.api_url.clone(),
            config.api_key.clone(),
            config.model_name.clone(),
            config.platform,
        )
    };
    let mut promot = "Treat next line as plain text input and translate it into {{to}} output translation ONLY. If translation is unnecessary (e.g. proper nouns, codes, etc.), return the original text. NO explanations. NO notes. Input:
 {{text}}".replace("{{text}}", &text).replace("{{to}}", target_lang);

    let content: String = match platform {
        PlatformType::OLLama => {
            promot.push_str(" /no_think");
            promot
        }
        PlatformType::DeepSeek => promot,
        PlatformType::ChatGPT => promot,
    };
    println!("content: {:?}", content);
    let chat_req = ChatRequest {
        model: model_name.to_string(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content,
        }],
        stream: Some(false),
    };

    let res = config::REQUEST_CLIENT
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&chat_req)
        .send()
        .await?;

    if res.status().is_success() {
        let response: serde_json::Value = res.json().await?;
        match platform {
            PlatformType::OLLama => {
                if let Some(message) = response["message"]["content"].as_str() {
                    Ok(message.to_string())
                } else {
                    Err("Failed to parse response".into())
                }
            }
            PlatformType::DeepSeek => {
                if let Some(message) = response["choices"][0]["message"]["content"].as_str() {
                    Ok(message.to_string())
                } else {
                    Err("Failed to parse response".into())
                }
            }
            PlatformType::ChatGPT => {
                if let Some(message) = response["choices"][0]["message"]["content"].as_str() {
                    Ok(message.to_string())
                } else {
                    Err("Failed to parse response".into())
                }
            }
        }
    } else {
        Err(format!("Request failed with status: {}", res.status()).into())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_translate() {
        let result = translate(
            "これらのフレーズは、日常で非常によく使われる基本的なものです。/no_think".to_string(),
            "中文",
        )
        .await;
        assert!(result.is_ok());
        println!("Translation: {:?}", result.unwrap());
    }
}
