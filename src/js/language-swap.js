import { updateSourcePlaceholder, updateTargetPlaceholder } from './ui-utils.js';
import { translateText } from './translation.js';

// 交换语言
function swapLanguages() {
    if (!window.targetLang) return;
    
    const tempLang = window.sourceLang.value;
    const tempText = window.sourceText.value;
    
    window.sourceLang.value = window.targetLang.value;
    window.targetLang.value = tempLang === 'auto' ? 'zh' : tempLang;
    
    window.sourceText.value = window.translatedText.value;
    window.translatedText.value = tempText;
    
    updateSourcePlaceholder();
    updateTargetPlaceholder();
    
    if (window.sourceText.value.trim()) {
        translateText();
    }
}

export { swapLanguages };