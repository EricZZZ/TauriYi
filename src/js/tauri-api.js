// Tauri API导入和封装
let invoke, listen;

if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.event) {
    invoke = window.__TAURI__.core.invoke;
    listen = window.__TAURI__.event.listen;
} else {
    invoke = async () => {
        console.warn('invoke函数在非Tauri环境中不可用');
        return Promise.resolve();
    };
    listen = async () => {
        console.warn('listen函数在非Tauri环境中不可用');
        return () => {};
    };
}

// Tauri事件监听器设置
async function setupTauriEventListeners() {
    try {
        console.log('开始设置事件监听器...');
        
        const unlisten = await listen('clipboard-content', (event) => {
            console.log('收到剪贴板内容事件:', event);
            
            if (window.sourceText && event.payload) {
                window.sourceText.value = event.payload;
                window.handleTextInput();
                window.sourceText.focus();
                console.log('剪贴板内容已设置到输入框并触发翻译');
            }
        });
        
        return unlisten;
    } catch (error) {
        console.error('设置Tauri事件监听器失败:', error);
    }
}

// 初始化Tauri功能
async function initializeTauri() {
    if (window.__TAURI__) {
        console.log('检测到Tauri环境，初始化事件监听器');
        await setupTauriEventListeners();
    } else {
        console.log('非Tauri环境，跳过Tauri功能初始化');
    }
}

export { invoke, listen, initializeTauri };