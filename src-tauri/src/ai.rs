use serde::{Deserialize, Serialize};

use crate::config::{self, PlatformType};
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

pub async fn translate(
    text: String,
    target_lang: Lang,
    source_lang: Lang,
) -> Result<String, Box<dyn std::error::Error>> {
    let (api_url, api_key, model_name, platform,mut system_prompt,mut prompt) = {
        let config = config::CONFIG.get().unwrap().lock().unwrap();
        (
            config.api_url.clone(),
            config.api_key.clone(),
            config.model_name.clone(),
            config.platform,
            config.system_prompt.clone(),
            config.prompt.clone(),
        )
    };
    system_prompt = system_prompt.replace("{{to}}", source_lang.to_full_name());
    prompt = prompt.replace("{{text}}", &text).replace("{{to}}", target_lang.to_full_name());
    if model_name.contains("qwen3") {
        prompt.push_str(" /no_think");
    }
    let request_payload: RequestPayload = match platform {
        PlatformType::OLLama => { 
            RequestPayload::Chat(ChatRequest {
                model: model_name.to_string(),
                messages: vec![
                    ChatMessage {
                        role: "system".to_string(),
                        content: system_prompt.clone(),
                    }
                    ,ChatMessage {
                    role: "user".to_string(),
                    content: prompt.clone(),
                }],
                stream: Some(false),
            })
        }
        PlatformType::MTranServer => RequestPayload::MTran(MTranServerRequest {
            from: source_lang.into(), // 假设 MTranServer 支持 "auto" 检测，或者您需要传入源语言参数
            to: target_lang.into(),
            text: text.clone(), // 使用原始输入文本
        }),
        _ => RequestPayload::Chat(ChatRequest {
            model: model_name.to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.clone(),
                },
                ChatMessage {
                role: "user".to_string(),
                content: prompt.clone(),
            }],
            stream: Some(false),
        }),
    };
    println!("content: {:?}", prompt.clone());

    let res = config::REQUEST_CLIENT
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_payload)
        .send()
        .await?;

    if res.status().is_success() {
        let response: serde_json::Value = res.json().await?;
        match platform {
            PlatformType::OLLama => {
                if let Some(message) = response["message"]["content"].as_str() {
                    if model_name.contains("qwen3") {
                        // 移除 <think></think> 标记
                        println!("content: {:?}", message);
                        let message = message.replace("<think>\n\n</think>\n\n", "");
                        Ok(message.to_string())
                    } else {
                        Ok(message.to_string())    
                    }
                    
                } else {
                    Err("Failed to parse response".into())
                }
            }  
            PlatformType::MTranServer => {
                if let Some(message) = response["result"].as_str() {
                    Ok(message.to_string())
                } else {
                    Err("Failed to parse response".into())
                }
            }
            _ => {
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
            Lang::Zh,
            Lang::Ja,
        )
        .await;
        assert!(result.is_ok());
        println!("Translation: {:?}", result.unwrap());
    }
}
