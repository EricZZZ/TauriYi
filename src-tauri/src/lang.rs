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
            Lang::Zh => "chinese",
            Lang::En => "english",
            Lang::Ja => "japanese",
            Lang::Ko => "korean",
            Lang::Auto => "auto",
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
        assert_eq!(Lang::Zh.to_full_name(), "chinese");
        assert_eq!(Lang::En.to_full_name(), "english");
        assert_eq!(Lang::Ja.to_full_name(), "japanese");
        assert_eq!(Lang::Ko.to_full_name(), "korean");
    }
}
