
// Tauri API导入 - 安全检查
let invoke, listen;

if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.event) {
    // 在Tauri环境中，使用正确的API路径
    invoke = window.__TAURI__.core.invoke;
    listen = window.__TAURI__.event.listen;
} else {
    // 在浏览器环境中，提供空函数避免错误
    invoke = async () => {
        console.warn('invoke函数在非Tauri环境中不可用');
        return Promise.resolve();
    };
    listen = async () => {
        console.warn('listen函数在非Tauri环境中不可用');
        return () => {}; // 返回一个空的取消监听函数
    };
}

// 元素引用
const sourceLang = document.getElementById('sourceLang');
const targetLang = document.getElementById('targetLang');
const swapBtn = document.querySelector('.swap-btn');
const sourceText = document.getElementById('sourceText');
const translatedText = document.getElementById('translatedText');

// 语言映射
const languages = {
    'zh': '中文',
    'en': '英语',
    'ja': '日语',
    'ko': '韩语'
};

// 语言对应的占位符
const placeholders = {
    'zh': '输入文本',
    'en': 'Enter text',
    'ja': 'テキストを入力',
    'ko': '텍스트 입력'
};

// 模拟翻译数据库
const translationDatabase = {
    'zh-en': {
        '你好': 'Hello',
        '谢谢': 'Thank you',
        '再见': 'Goodbye',
        '早上好': 'Good morning',
        '晚上好': 'Good evening',
        '我爱你': 'I love you',
        '今天天气很好': 'The weather is nice today',
        '你叫什么名字': 'What is your name',
        '很高兴见到你': 'Nice to meet you',
        '请问洗手间在哪里': 'Where is the bathroom please'
    },
    'zh-ja': {
        '你好': 'こんにちは',
        '谢谢': 'ありがとう',
        '再见': 'さようなら',
        '早上好': 'おはよう',
        '晚上好': 'こんばんは',
        '我爱你': '愛してる',
        '今天天气很好': '今日はいい天気ですね',
        '你叫什么名字': 'お名前は何ですか',
        '很高兴见到你': 'お会いできて嬉しいです',
        '请问洗手间在哪里': 'トイレはどこですか'
    },
    'en-zh': {
        'hello': '你好',
        'thank you': '谢谢',
        'goodbye': '再见',
        'good morning': '早上好',
        'good evening': '晚上好',
        'i love you': '我爱你',
        'how are you': '你好吗',
        'what is your name': '你叫什么名字',
        'nice to meet you': '很高兴见到你',
        'where is the bathroom': '洗手间在哪里'
    },
    'ja-zh': {
        'こんにちは': '你好',
        'ありがとう': '谢谢',
        'さようなら': '再见',
        'おはよう': '早上好',
        'こんばんは': '晚上好',
        '愛してる': '我爱你',
        'お名前は何ですか': '你叫什么名字',
        'お会いできて嬉しいです': '很高兴见到你'
    }
};

// 翻译状态
let isTranslating = false;
let translateTimeout = null;

// Tauri事件监听 - 监听剪贴板内容事件
async function setupTauriEventListeners() {
    try {
        console.log('开始设置事件监听器...');
        console.log('listen函数类型:', typeof listen);
        
        // 监听clipboard-content事件
        const unlisten = await listen('clipboard-content', (event) => {
            console.log('收到剪贴板内容事件:', event);
            console.log('事件payload:', event.payload);
            
            // 将剪贴板内容设置到sourceText控件中
            if (sourceText && event.payload) {
                sourceText.value = event.payload;
                console.log('设置文本内容:', event.payload);
                
                // 直接调用翻译函数，而不是触发事件
                handleTextInput();
                
                // 可选：聚焦到文本框
                sourceText.focus();
                console.log('剪贴板内容已设置到输入框并触发翻译');
            } else {
                console.warn('sourceText元素不存在或payload为空');
            }
        });
        
        console.log('Tauri事件监听器已设置，unlisten函数:', typeof unlisten);
        
        // 返回取消监听的函数，可以在需要时调用
        return unlisten;
    } catch (error) {
        console.error('设置Tauri事件监听器失败:', error);
    }
}

// 初始化Tauri功能
async function initializeTauri() {
    // 检查是否在Tauri环境中运行
    if (window.__TAURI__) {
        console.log('检测到Tauri环境，初始化事件监听器');
        await setupTauriEventListeners();
        
    } else {
        console.log('非Tauri环境，跳过Tauri功能初始化');
    }
}

// 交换语言
function swapLanguages() {
    if (!targetLang) return;
    
    const tempLang = sourceLang.value;
    const tempText = sourceText.value;
    
    // 交换语言选择
    sourceLang.value = targetLang.value;
    targetLang.value = tempLang === 'auto' ? 'zh' : tempLang;
    
    // 交换文本内容
    sourceText.value = translatedText.value;
    translatedText.value = tempText;
    
    // 更新占位符
    updateSourcePlaceholder();
    updateTargetPlaceholder();
    
    // 如果有文本，重新翻译
    if (sourceText.value.trim()) {
        translateText();
    }
}

// 更新源语言占位符
function updateSourcePlaceholder() {
    const lang = sourceLang.value === 'auto' ? 'zh' : sourceLang.value;
    sourceText.placeholder = placeholders[lang] || '输入文本';
}

// 更新目标语言占位符
function updateTargetPlaceholder() {
    if (!targetLang) return;
    const lang = targetLang.value;
    translatedText.placeholder = placeholders[lang] || 'Translation';
}

// 处理文本输入
function handleTextInput() {
    console.log('handleTextInput被调用，当前文本:', sourceText.value);
    
    // 清除之前的定时器
    if (translateTimeout) {
        clearTimeout(translateTimeout);
    }
    
    const text = sourceText.value.trim();
    
    if (!text) {
        translatedText.value = '';
        return;
    }
    
    // 防抖处理，用户停止输入500ms后开始翻译
    translateTimeout = setTimeout(() => {
        console.log('开始翻译:', text);
        translateText();
    }, 500);
}

// 模拟翻译API调用
// 翻译文本函数
async function translateText() {
    const text = sourceText.value.trim();
    if (!text || isTranslating) return;
    
    const source = sourceLang.value === 'auto' ? detectLanguage(text) : sourceLang.value;
    const target = targetLang ? targetLang.value : 'en';
    
    // 如果源语言和目标语言相同，不需要翻译
    if (source === target) {
        translatedText.value = text;
        return;
    }
    
    try {
        isTranslating = true;
        
        // 显示加载状态
        translatedText.value = '翻译中...';
        translatedText.style.opacity = '0.6';
        
        // 使用 Tauri invoke 调用后端翻译服务
        const result = await invoke('translate', {
            text: text,
            targetLang: languages[target]
        });
        
        // 处理后端返回的 R<String> 结构
        if (result.code === 0 && result.data) {
            // 翻译成功
            // 在translateText函数中，设置翻译结果后
            translatedText.value = result.data;
            translatedText.style.opacity = '1';
            autoResizeTextarea(translatedText); // 添加这行
        } else {
            // 翻译失败
            const errorMsg = result.msg || '翻译失败，请重试';
            translatedText.value = errorMsg;
            translatedText.style.opacity = '1';
            console.error('Translation failed:', result);
        }
        
    } catch (error) {
        translatedText.value = '翻译失败，请重试';
        translatedText.style.opacity = '1';
        console.error('Translation error:', error);
    } finally {
        isTranslating = false;
    }
}

// 模拟翻译API
function simulateTranslationAPI(text, sourceLang, targetLang) {
    return new Promise((resolve, reject) => {
        // 模拟网络延迟 500-1500ms
        const delay = Math.random() * 1000 + 500;
        
        setTimeout(() => {
            try {
                const result = getTranslation(text, sourceLang, targetLang);
                resolve(result);
            } catch (error) {
                reject(error);
            }
        }, delay);
    });
}

// 获取翻译结果
function getTranslation(text, sourceLang, targetLang) {
    const key = `${sourceLang}-${targetLang}`;
    const database = translationDatabase[key];
    
    if (database) {
        // 尝试精确匹配
        const exactMatch = database[text.toLowerCase()];
        if (exactMatch) {
            return exactMatch;
        }
        
        // 尝试部分匹配
        for (const [key, value] of Object.entries(database)) {
            if (text.toLowerCase().includes(key) || key.includes(text.toLowerCase())) {
                return value;
            }
        }
    }
    
    // 如果没有找到匹配，返回通用翻译格式
    return generateFallbackTranslation(text, sourceLang, targetLang);
}

// 生成备用翻译
function generateFallbackTranslation(text, sourceLang, targetLang) {
    const languageNames = {
        'zh': '中文',
        'en': 'English',
        'ja': '日本語',
        'ko': '한국어'
    };
    
    const targetName = languageNames[targetLang] || targetLang;
    
    // 根据目标语言返回不同格式的翻译
    switch (targetLang) {
        case 'en':
            return `[Translated to English] ${text}`;
        case 'ja':
            return `[日本語に翻訳] ${text}`;
        case 'ko':
            return `[한국어로 번역] ${text}`;
        case 'zh':
        default:
            return `[已翻译为${targetName}] ${text}`;
    }
}

// 简单的语言检测
function detectLanguage(text) {
    // 检测中文字符
    if (/[\u4e00-\u9fff]/.test(text)) {
        return 'zh';
    }
    // 检测日文字符
    if (/[\u3040-\u309f\u30a0-\u30ff]/.test(text)) {
        return 'ja';
    }
    // 检测韩文字符
    if (/[\uac00-\ud7af]/.test(text)) {
        return 'ko';
    }
    // 默认为英文
    return 'en';
}

// 页面加载完成后初始化
document.addEventListener('DOMContentLoaded', async () => {
    // 初始化占位符
    if (typeof updateSourcePlaceholder === 'function') {
        updateSourcePlaceholder();
    }
    if (typeof updateTargetPlaceholder === 'function') {
        updateTargetPlaceholder();
    }
    
    // 绑定事件监听器
    if (sourceText) {
        sourceText.addEventListener('input', handleTextInput);
        console.log('已绑定sourceText的input事件监听器');
    }
    
    if (swapBtn) {
        swapBtn.addEventListener('click', swapLanguages);
        console.log('已绑定语言交换按钮事件监听器');
    }
    
    // 添加关闭按钮事件监听器
    const closeBtn = document.querySelector('.close-icon');
    if (closeBtn) {
        closeBtn.addEventListener('click', async () => {
            try {
                await invoke('close_window');
                console.log('窗口关闭命令已发送');
            } catch (error) {
                console.error('关闭窗口失败:', error);
            }
        });
        console.log('已绑定关闭按钮事件监听器');
    }
    
    if (sourceLang) {
        sourceLang.addEventListener('change', () => {
            updateSourcePlaceholder();
            if (sourceText.value.trim()) {
                handleTextInput();
            }
        });
    }
    
    // 设置textarea自动调整高度
    setupAutoResize();
    
    // 初始化Tauri功能
    initializeTauri();
});


// 自动调整textarea高度
function autoResizeTextarea(textarea) {
    // 重置高度以获取正确的scrollHeight
    textarea.style.height = 'auto';
    
    // 计算新高度
    const newHeight = Math.min(textarea.scrollHeight, 200); // 最大200px
    const minHeight = 40; // 最小40px
    
    // 设置新高度
    textarea.style.height = Math.max(newHeight, minHeight) + 'px';
}

// 为两个textarea添加事件监听器
function setupAutoResize() {
    const sourceTextarea = document.getElementById('sourceText');
    const translatedTextarea = document.getElementById('translatedText');
    
    if (sourceTextarea) {
        // 输入时自动调整高度
        sourceTextarea.addEventListener('input', function() {
            autoResizeTextarea(this);
        });
        
        // 粘贴时也要调整高度
        sourceTextarea.addEventListener('paste', function() {
            setTimeout(() => {
                autoResizeTextarea(this);
            }, 0);
        });
    }
    
    if (translatedTextarea) {
        // 移除 MutationObserver，只保留 setInterval 监听
        let lastValue = translatedTextarea.value;
        setInterval(() => {
            if (translatedTextarea.value !== lastValue) {
                lastValue = translatedTextarea.value;
                autoResizeTextarea(translatedTextarea);
            }
        }, 100);
    }
}


// 设置页面相关元素
const settingsIcon = document.querySelector('.settings-icon');
const settingsPage = document.getElementById('settingsPage');
const translationPage = document.getElementById('translationPage');
const backBtn = document.getElementById('backBtn');
const saveBtn = document.getElementById('saveBtn');
const resetBtn = document.getElementById('resetBtn');
const toggleApiKeyBtn = document.getElementById('toggleApiKeyBtn');

// 设置表单元素
const apiKeyInput = document.getElementById('apiKey');
const apiUrlInput = document.getElementById('apiUrl');
const platformRadios = document.querySelectorAll('input[name="platform"]');
const modelNameInput = document.getElementById('modelName');

// 设置页面显示/隐藏
// 显示设置页面
// 在文件顶部（在其他变量定义附近）添加状态变量
let isSettingsVisible = false;

// 添加新的切换函数（在现有函数附近添加）
async function toggleSettingsPage() {
    if (isSettingsVisible) {
        // 关闭设置页面
        settingsPage.style.display = 'none';
        translationPage.style.display = 'flex';
        settingsIcon.classList.remove('active');
        isSettingsVisible = false;
    } else {
        // 打开设置页面
        translationPage.style.display = 'none';
        settingsPage.style.display = 'flex';
        settingsIcon.classList.add('active');
        await loadSettings();
        isSettingsVisible = true;
    }
}

// 修改事件监听器绑定部分
document.addEventListener('DOMContentLoaded', function() {
    // 设置页面事件监听 - 将原来的 showSettingsPage 改为 toggleSettingsPage
    settingsIcon.addEventListener('click', toggleSettingsPage);
    saveBtn.addEventListener('click', saveSettings);
    resetBtn.addEventListener('click', resetSettings);
    toggleApiKeyBtn.addEventListener('click', togglePasswordVisibility);
    // ... existing code ...
});

// 加载设置
// 加载设置
async function loadSettings() {
    const settings = await getStoredSettings();
    console.log('加载的设置:', settings);
    apiKeyInput.value = settings.apiKey || '';
    apiUrlInput.value = settings.apiUrl || '';
    modelNameInput.value = settings.modelName || '';
    
    // 设置平台单选框
    platformRadios.forEach(radio => {
        radio.checked = radio.value === settings.platform;
    });
}

// 获取设置
async function getStoredSettings() {
    try {
        // 使用 Tauri invoke 获取配置
        const result = await invoke('load_config');
        const settings = {}
        // 处理后端返回的 R<String> 结构
        if (result.code === 0 && result.data) {
            // 读取成功
            console.log('读取到的配置:', result.data);
            return result.data;
        } else {
            console.error('Translation failed:', result);
        }
        return {};
    } catch (error) {
        console.error('读取设置失败:', error);
        return {};
    }
}

// 保存设置
async function saveSettings() {
    const selectedPlatform = document.querySelector('input[name="platform"]:checked');
    
    if (!apiKeyInput.value.trim()) {
        alert('请输入API Key');
        return;
    }
    
    if (!apiUrlInput.value.trim()) {
        alert('请输入API URL');
        return;
    }
    
    if (!selectedPlatform) {
        alert('请选择平台');
        return;
    }
    
    if (!modelNameInput.value.trim()) {
        alert('请输入模型名称');
        return;
    }
    
    const settings = {
        apiKey: apiKeyInput.value.trim(),
        apiUrl: apiUrlInput.value.trim(),
        platform: selectedPlatform.value,
        modelName: modelNameInput.value.trim()
    };
    
    try {
        console.log('保存的设置:', settings);
        // 使用 Tauri invoke 调用后端翻译服务
        const result = await invoke('update_config', {
            newConfig: settings
        });
        
        // 处理后端返回的 R<()> 结构
        if (result.code === 0) {
            // 更新配置成功
            console.log('保存配置成功:', result.data);
        } else {
            // 更新配置失败
            const errorMsg = result.msg || '更新配置失败，请重试';
            console.error('Update Config failed:', errorMsg);
        }
        
    } catch (error) {
        console.error('保存设置失败:', error);
        alert('保存设置失败');
    }
}

// 重置设置
function resetSettings() {
    if (confirm('确定要重置所有设置吗？')) {
        apiKeyInput.value = '';
        apiUrlInput.value = '';
        modelNameInput.value = '';
        platformRadios.forEach(radio => {
            radio.checked = false;
        });
        
        try {
            localStorage.removeItem('translationSettings');
            alert('设置已重置');
        } catch (error) {
            console.error('重置设置失败:', error);
        }
    }
}

function togglePasswordVisibility() {
    const input = document.getElementById('apiKey');
    
    // 通过父容器查找按钮
    const container = input.parentElement;
    const button = container.querySelector('.toggle-password');
    
    if (!button) {
        console.error('找不到切换按钮');
        return;
    }
    
    if (input.type === 'password') {
        input.type = 'text';
        button.textContent = '🙈';
    } else {
        input.type = 'password';
        button.textContent = '👁️';
    }
}

