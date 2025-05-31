use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    Zh,
    En,
    Ja,
    Ko,
    Auto,
}

impl Lang {
    pub fn to_full_name(self) -> &'static str {
        match self {
            Lang::Zh => "中文",
            Lang::En => "英语",
            Lang::Ja => "日语",
            Lang::Ko => "韩语",
            Lang::Auto => "自动",
        }
    }
}

impl From<Lang> for String {
    fn from(val: Lang) -> Self {
        match val {
            Lang::Zh => "zh".to_string(),
            Lang::En => "en".to_string(),
            Lang::Ja => "ja".to_string(),
            Lang::Ko => "ko".to_string(),
            Lang::Auto => "auto".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lang_into() {
        assert_eq!(Lang::Zh.to_full_name(), "中文");
        assert_eq!(Lang::En.to_full_name(), "英语");
        assert_eq!(Lang::Ja.to_full_name(), "日语");
        assert_eq!(Lang::Ko.to_full_name(), "韩语");
    }
}
