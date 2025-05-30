import { invoke } from './tauri-api.js';

// 设置状态
let isSettingsVisible = false;

// 切换设置页面
async function toggleSettingsPage() {
    const settingsPage = document.getElementById('settingsPage');
    const translationPage = document.getElementById('translationPage');
    const settingsIcon = document.querySelector('.settings-icon');
    
    if (isSettingsVisible) {
        settingsPage.style.display = 'none';
        translationPage.style.display = 'flex';
        settingsIcon.classList.remove('active');
        isSettingsVisible = false;
    } else {
        translationPage.style.display = 'none';
        settingsPage.style.display = 'flex';
        settingsIcon.classList.add('active');
        await loadSettings();
        isSettingsVisible = true;
    }
}

// 加载设置
async function loadSettings() {
    const settings = await getStoredSettings();
    document.getElementById('apiKey').value = settings.apiKey || '';
    document.getElementById('apiUrl').value = settings.apiUrl || '';
    document.getElementById('modelName').value = settings.modelName || '';
    
    document.querySelectorAll('input[name="platform"]').forEach(radio => {
        radio.checked = radio.value === settings.platform;
    });
}

// 获取存储的设置
async function getStoredSettings() {
    try {
        const result = await invoke('load_config');
        if (result.code === 0 && result.data) {
            return result.data;
        }
        return {};
    } catch (error) {
        console.error('读取设置失败:', error);
        return {};
    }
}

// 保存设置
async function saveSettings() {
    const apiKey = document.getElementById('apiKey').value.trim();
    const apiUrl = document.getElementById('apiUrl').value.trim();
    const modelName = document.getElementById('modelName').value.trim();
    const selectedPlatform = document.querySelector('input[name="platform"]:checked');
    
    if (!apiKey || !apiUrl || !selectedPlatform || !modelName) {
        alert('请填写所有必填项');
        return;
    }
    
    const settings = {
        apiKey,
        apiUrl,
        platform: selectedPlatform.value,
        modelName
    };
    
    try {
        const result = await invoke('update_config', { newConfig: settings });
        if (result.code === 0) {
            console.log('保存配置成功');
        } else {
            console.error('保存配置失败:', result.msg);
        }
    } catch (error) {
        console.error('保存设置失败:', error);
        alert('保存设置失败');
    }
}

// 重置设置
function resetSettings() {
    if (confirm('确定要重置所有设置吗？')) {
        document.getElementById('apiKey').value = '';
        document.getElementById('apiUrl').value = '';
        document.getElementById('modelName').value = '';
        document.querySelectorAll('input[name="platform"]').forEach(radio => {
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

// 切换密码可见性
function togglePasswordVisibility() {
    const input = document.getElementById('apiKey');
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

export { toggleSettingsPage, saveSettings, resetSettings, togglePasswordVisibility };