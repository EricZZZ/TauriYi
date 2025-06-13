use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;
use crate::config::{self, get_database, PlatformType};
use crate::lang::Lang;

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

#[derive(Serialize, Debug)]

struct MTranServerRequest {
    from: String,
    to: String,
    text: String,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
enum RequestPayload {
    Chat(ChatRequest),
    MTran(MTranServerRequest),
}

pub async fn translate(text: String, target_lang: Lang, source_lang: Lang) -> Result<String> {
    let config = config::get_config()?;
    let request_payload = build_request_payload(&text, target_lang, source_lang, &config)?;

    let response =
        send_translation_request(&config.api_url, &config.api_key, request_payload).await?;
    let translated_text =
        parse_translation_response(response, &config.platform, &config.model_name).await?;

    let database = {
        match get_database() {
            Ok(db_arc) => match db_arc.lock() {
                Ok(db_guard) => {
                    if let Some(ref database) = *db_guard {
                        database.clone()
                    } else {
                        return Err(anyhow::anyhow!("数据库未初始化"));
                    }
                }
                Err(_) => return Err(anyhow::anyhow!("数据库锁定失败")),
            },
            Err(e) => return Err(anyhow::anyhow!("获取数据库连接失败: {:?}", e)),
        }
    };

    database
        .save_translation(
            &text,
            &translated_text,
            source_lang.into(),
            target_lang.into(),
        )
        .await?;

    Ok(translated_text)
}

async fn send_translation_request(
    api_url: &str,
    api_key: &str,
    request_payload: RequestPayload,
) -> Result<reqwest::Response> {
    let response = config::REQUEST_CLIENT
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_payload)
        .send()
        .await?;
    Ok(response)
}

fn build_request_payload(
    text: &str,
    target_lang: Lang,
    source_lang: Lang,
    config: &AppConfig,
) -> Result<RequestPayload> {
    let system_prompt = config
        .system_prompt
        .replace("{{to}}", source_lang.to_full_name());
    let mut prompt = config
        .prompt
        .replace("{{text}}", text)
        .replace("{{to}}", target_lang.to_full_name());

    match config.platform {
        PlatformType::OLLama => {
            if config.model_name.contains("qwen3") {
                prompt.push_str(" /no_think");
            }
            Ok(RequestPayload::Chat(ChatRequest {
                model: config.model_name.clone(),
                messages: vec![
                    ChatMessage {
                        role: "system".to_string(),
                        content: system_prompt,
                    },
                    ChatMessage {
                        role: "user".to_string(),
                        content: prompt,
                    },
                ],
                stream: Some(false),
            }))
        }
        PlatformType::MTranServer => Ok(RequestPayload::MTran(MTranServerRequest {
            from: source_lang.into(),
            to: target_lang.into(),
            text: text.to_string(),
        })),
        _ => Ok(RequestPayload::Chat(ChatRequest {
            model: config.model_name.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            stream: Some(false),
        })),
    }
}

async fn parse_translation_response(
    response: reqwest::Response,
    platform: &PlatformType,
    model_name: &str,
) -> Result<String> {
    let json: serde_json::Value = response.json().await?;

    let content = match platform {
        PlatformType::OLLama => json["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to parse OLLama response"))?,
        PlatformType::MTranServer => json["result"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to parse MTranServer response"))?,
        _ => json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to parse ChatGPT/DeepSeek response"))?,
    };

    // 处理特殊模型的响应清理
    if platform == &PlatformType::OLLama && model_name.contains("qwen3") {
        Ok(content.replace("<think>\n\n</think>\n\n", ""))
    } else {
        Ok(content.to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_translate() {
        let result = translate(
            "これらのフレーズは、日常で非常によく使われる基本的なものです。/no_think".to_string(),
            Lang::Zh,
            Lang::Ja,
        )
        .await;
        assert!(result.is_ok());
        println!("Translation: {:?}", result.unwrap());
    }
}
