import { invoke, initializeTauri } from './tauri-api.js';
import { handleTextInput } from './translation.js';
import { setupAutoResize, updateSourcePlaceholder, updateTargetPlaceholder } from './ui-utils.js';
import { swapLanguages } from './language-swap.js';
import { toggleSettingsPage, saveSettings, resetSettings, togglePasswordVisibility } from './settings.js';

// 全局元素引用
window.sourceLang = document.getElementById('sourceLang');
window.targetLang = document.getElementById('targetLang');
window.sourceText = document.getElementById('sourceText');
window.translatedText = document.getElementById('translatedText');

// 页面加载完成后初始化
document.addEventListener('DOMContentLoaded', async () => {
    // 初始化占位符
    updateSourcePlaceholder();
    updateTargetPlaceholder();
    
    // 绑定翻译相关事件
    if (window.sourceText) {
        window.sourceText.addEventListener('input', handleTextInput);
        window.handleTextInput = handleTextInput; // 暴露给全局
    }
    
    // 绑定语言交换
    const swapBtn = document.querySelector('.swap-btn');
    if (swapBtn) {
        swapBtn.addEventListener('click', swapLanguages);
    }
    
    // 绑定关闭按钮
    const closeBtn = document.querySelector('.close-icon');
    if (closeBtn) {
        closeBtn.addEventListener('click', async () => {
            try {
                await invoke('close_window');
            } catch (error) {
                console.error('关闭窗口失败:', error);
            }
        });
    }
    
    // 绑定设置相关事件
    const settingsIcon = document.querySelector('.settings-icon');
    if (settingsIcon) {
        settingsIcon.addEventListener('click', toggleSettingsPage);
    }
    
    const saveBtn = document.getElementById('saveBtn');
    if (saveBtn) {
        saveBtn.addEventListener('click', saveSettings);
    }
    
    const resetBtn = document.getElementById('resetBtn');
    if (resetBtn) {
        resetBtn.addEventListener('click', resetSettings);
    }
    
    const toggleApiKeyBtn = document.getElementById('toggleApiKeyBtn');
    if (toggleApiKeyBtn) {
        toggleApiKeyBtn.addEventListener('click', togglePasswordVisibility);
    }
    
    // 绑定语言选择变化事件
    if (window.sourceLang) {
        window.sourceLang.addEventListener('change', () => {
            updateSourcePlaceholder();
            if (window.sourceText.value.trim()) {
                handleTextInput();
            }
        });
    }
    
    // 设置自动调整高度
    setupAutoResize();
    
    // 初始化Tauri功能
    await initializeTauri();
});