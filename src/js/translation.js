import { invoke } from './tauri-api.js';
import { autoResizeTextarea } from './ui-utils.js';

// 语言映射和占位符
const languages = {
    'zh': '中文',
    'en': '英语',
    'ja': '日语',
    'ko': '韩语'
};

const placeholders = {
    'zh': '输入文本',
    'en': 'Enter text',
    'ja': 'テキストを入力',
    'ko': '텍스트 입력'
};

// 翻译状态
let isTranslating = false;
let translateTimeout = null;

// 语言检测
function detectLanguage(text) {
    if (/[\u4e00-\u9fff]/.test(text)) return 'zh';
    if (/[\u3040-\u309f\u30a0-\u30ff]/.test(text)) return 'ja';
    if (/[\uac00-\ud7af]/.test(text)) return 'ko';
    return 'en';
}

// 翻译文本函数
async function translateText() {
    const text = window.sourceText.value.trim();
    if (!text || isTranslating) return;
    
    const source = window.sourceLang.value === 'auto' ? detectLanguage(text) : window.sourceLang.value;
    const target = window.targetLang ? window.targetLang.value : 'en';
    
    if (source === target) {
        window.translatedText.value = text;
        return;
    }
    
    try {
        isTranslating = true;
        window.translatedText.value = '翻译中...';
        window.translatedText.style.opacity = '0.6';
        
        const result = await invoke('translate', {
            text: text,
            targetLang: languages[target]
        });
        
        if (result.code === 0 && result.data) {
            window.translatedText.value = result.data;
            window.translatedText.style.opacity = '1';
            autoResizeTextarea(window.translatedText);
        } else {
            const errorMsg = result.msg || '翻译失败，请重试';
            window.translatedText.value = errorMsg;
            window.translatedText.style.opacity = '1';
            console.error('Translation failed:', result);
        }
    } catch (error) {
        window.translatedText.value = '翻译失败，请重试';
        window.translatedText.style.opacity = '1';
        console.error('Translation error:', error);
    } finally {
        isTranslating = false;
    }
}

// 处理文本输入
function handleTextInput() {
    if (translateTimeout) {
        clearTimeout(translateTimeout);
    }
    
    const text = window.sourceText.value.trim();
    
    if (!text) {
        window.translatedText.value = '';
        return;
    }
    
    translateTimeout = setTimeout(() => {
        translateText();
    }, 500);
}

export { translateText, handleTextInput, languages, placeholders, detectLanguage };