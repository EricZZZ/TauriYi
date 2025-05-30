// 自动调整textarea高度
function autoResizeTextarea(textarea) {
    textarea.style.height = 'auto';
    const newHeight = Math.min(textarea.scrollHeight, 200);
    const minHeight = 40;
    textarea.style.height = Math.max(newHeight, minHeight) + 'px';
}

// 设置自动调整功能
function setupAutoResize() {
    const sourceTextarea = document.getElementById('sourceText');
    const translatedTextarea = document.getElementById('translatedText');
    
    if (sourceTextarea) {
        sourceTextarea.addEventListener('input', function() {
            autoResizeTextarea(this);
        });
        
        sourceTextarea.addEventListener('paste', function() {
            setTimeout(() => {
                autoResizeTextarea(this);
            }, 0);
        });
    }
    
    if (translatedTextarea) {
        let lastValue = translatedTextarea.value;
        setInterval(() => {
            if (translatedTextarea.value !== lastValue) {
                lastValue = translatedTextarea.value;
                autoResizeTextarea(translatedTextarea);
            }
        }, 100);
    }
}

// 语言对应的占位符
const placeholders = {
    'zh': '输入文本',
    'en': 'Enter text',
    'ja': 'テキストを入力',
    'ko': '텍스트 입력'
};

// 更新占位符
function updateSourcePlaceholder() {
    const lang = window.sourceLang.value === 'auto' ? 'zh' : window.sourceLang.value;
    window.sourceText.placeholder = placeholders[lang] || '输入文本';
}

function updateTargetPlaceholder() {
    if (!window.targetLang) return;
    const lang = window.targetLang.value;
    window.translatedText.placeholder = placeholders[lang] || 'Translation';
}

export { autoResizeTextarea, setupAutoResize, updateSourcePlaceholder, updateTargetPlaceholder };