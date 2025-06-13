import { invoke } from './tauri-api.js';

// 历史页面状态
let isHistoryVisible = false;
let currentPage = 1;
const pageSize = 10;
let isSearchMode = false;
let currentSearchQuery = '';
let totalPages = 0;
let allHistory = [];

// 切换历史页面显示
export function toggleHistoryPage() {
    console.log('切换历史页面');
    const historyPage = document.getElementById('historyPage');
    const translationPage = document.querySelector('.translation-area');
    const settingsPage = document.querySelector('.settings-page');
    const historyIcon = document.querySelector('.history-icon');
    const settingsIcon = document.querySelector('.settings-icon');
    
    if (!historyPage || !translationPage) {
        console.error('找不到历史页面或翻译页面元素');
        return;
    }
    
    isHistoryVisible = !isHistoryVisible;
    
    if (isHistoryVisible) {
        // 显示历史页面
        historyPage.style.display = 'flex';
        translationPage.style.display = 'none';
        if (settingsPage) settingsPage.style.display = 'none';
        
        // 更新图标状态
        if (historyIcon) historyIcon.classList.add('active');
        if (settingsIcon) settingsIcon.classList.remove('active');
        
        // 加载历史记录
        loadHistory();
    } else {
        // 隐藏历史页面
        historyPage.style.display = 'none';
        translationPage.style.display = 'flex';
        
        // 更新图标状态
        if (historyIcon) historyIcon.classList.remove('active');
    }
}

// 返回主页面
export function goBack() {
    const historyPage = document.getElementById('historyPage');
    const translationPage = document.querySelector('.translation-area');
    const historyIcon = document.querySelector('.history-icon');
    
    if (historyPage && translationPage) {
        historyPage.style.display = 'none';
        translationPage.style.display = 'flex';
        isHistoryVisible = false;
        
        if (historyIcon) historyIcon.classList.remove('active');
    }
}

// 加载历史记录
export async function loadHistory() {
    try {
        console.log('开始加载历史记录');
        const result = await invoke('get_translation_history');
        console.log('历史记录结果:', result);
        
        if (result.code === 0 && result.data) {
            allHistory = result.data;
            displayHistory();
        } else {
            console.error('加载历史记录失败:', result.message);
            showNoHistory('加载历史记录失败');
        }
    } catch (error) {
        console.error('加载历史记录出错:', error);
        showNoHistory('加载历史记录出错');
    }
}

// 显示历史记录
function displayHistory() {
    const historyList = document.getElementById('historyList');
    if (!historyList) return;
    
    let filteredHistory = allHistory;
    
    // 如果是搜索模式，过滤历史记录
    if (isSearchMode && currentSearchQuery) {
        filteredHistory = allHistory.filter(item => 
            item.source_text.toLowerCase().includes(currentSearchQuery.toLowerCase()) ||
            item.translated_text.toLowerCase().includes(currentSearchQuery.toLowerCase())
        );
    }
    
    // 计算分页
    totalPages = Math.ceil(filteredHistory.length / pageSize);
    const startIndex = (currentPage - 1) * pageSize;
    const endIndex = startIndex + pageSize;
    const pageHistory = filteredHistory.slice(startIndex, endIndex);
    
    // 清空列表
    historyList.innerHTML = '';
    
    if (pageHistory.length === 0) {
        showNoHistory(isSearchMode ? '没有找到匹配的记录' : '暂无翻译历史');
        return;
    }
    
    // 渲染历史记录
    pageHistory.forEach(item => {
        const historyItem = createHistoryItem(item);
        historyList.appendChild(historyItem);
    });
    
    // 更新分页
    updatePagination();
}

// 创建历史记录项
function createHistoryItem(item) {
    const div = document.createElement('div');
    div.className = 'history-item';
    
    const time = new Date(item.created_at).toLocaleString('zh-CN');
    
    div.innerHTML = `
        <div class="history-item-header">
            <span class="history-item-time">${time}</span>
            <button class="history-item-delete" onclick="deleteHistoryItem('${item.id}')">
                删除
            </button>
        </div>
        <div class="history-item-content">
            <div class="history-item-source">
                <div class="history-item-lang">${getLanguageName(item.source_lang)}</div>
                <div class="history-item-text">${escapeHtml(item.source_text)}</div>
            </div>
            <div class="history-item-target">
                <div class="history-item-lang">${getLanguageName(item.target_lang)}</div>
                <div class="history-item-text">${escapeHtml(item.translated_text)}</div>
            </div>
        </div>
    `;
    
    return div;
}

// 删除历史记录项
export async function deleteHistoryItem(id) {
    try {
        const result = await invoke('delete_translation', { id });
        if (result.code === 0) {
            // 重新加载历史记录
            await loadHistory();
        } else {
            console.error('删除历史记录失败:', result.message);
            alert('删除失败: ' + result.message);
        }
    } catch (error) {
        console.error('删除历史记录出错:', error);
        alert('删除出错: ' + error.message);
    }
}

// 搜索历史记录
export function searchHistory(query) {
    currentSearchQuery = query.trim();
    isSearchMode = currentSearchQuery.length > 0;
    currentPage = 1; // 重置到第一页
    displayHistory();
}

// 分页控制
export function goToPage(page) {
    if (page < 1 || page > totalPages) return;
    currentPage = page;
    displayHistory();
}

export function goToPrevPage() {
    if (currentPage > 1) {
        goToPage(currentPage - 1);
    }
}

export function goToNextPage() {
    if (currentPage < totalPages) {
        goToPage(currentPage + 1);
    }
}

// 更新分页显示
function updatePagination() {
    const pagination = document.getElementById('pagination');
    const prevBtn = document.getElementById('prevPage');
    const nextBtn = document.getElementById('nextPage');
    const pageInfo = document.getElementById('pageInfo');
    
    if (!pagination || !prevBtn || !nextBtn || !pageInfo) return;
    
    if (totalPages <= 1) {
        pagination.style.display = 'none';
        return;
    }
    
    pagination.style.display = 'flex';
    
    // 更新按钮状态
    prevBtn.disabled = currentPage <= 1;
    nextBtn.disabled = currentPage >= totalPages;
    
    // 更新页面信息
    pageInfo.textContent = `第 ${currentPage} 页，共 ${totalPages} 页`;
}

// 显示无历史记录
function showNoHistory(message) {
    const historyList = document.getElementById('historyList');
    const pagination = document.getElementById('pagination');
    
    if (historyList) {
        historyList.innerHTML = `<div class="no-history">${message}</div>`;
    }
    
    if (pagination) {
        pagination.style.display = 'none';
    }
}

// 获取语言名称
function getLanguageName(code) {
    const languages = {
        'auto': '自动检测',
        'zh': '中文',
        'en': '英语',
        'ja': '日语',
        'ko': '韩语',
        'fr': '法语',
        'de': '德语',
        'es': '西班牙语',
        'ru': '俄语',
        'it': '意大利语',
        'pt': '葡萄牙语',
        'ar': '阿拉伯语',
        'th': '泰语',
        'vi': '越南语'
    };
    return languages[code] || code;
}

// HTML转义
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// 初始化历史页面事件
export function initializeHistoryEvents() {
    // 返回按钮
    const backBtn = document.getElementById('backBtn');
    if (backBtn) {
        backBtn.addEventListener('click', goBack);
    }
    
    // 搜索输入
    const searchInput = document.getElementById('searchInput');
    if (searchInput) {
        let searchTimeout;
        searchInput.addEventListener('input', (e) => {
            clearTimeout(searchTimeout);
            searchTimeout = setTimeout(() => {
                searchHistory(e.target.value);
            }, 300); // 防抖，300ms后执行搜索
        });
    }
    
    // 分页按钮
    const prevBtn = document.getElementById('prevPage');
    const nextBtn = document.getElementById('nextPage');
    
    if (prevBtn) {
        prevBtn.addEventListener('click', goToPrevPage);
    }
    
    if (nextBtn) {
        nextBtn.addEventListener('click', goToNextPage);
    }
}

// 暴露删除函数到全局作用域
window.deleteHistoryItem = deleteHistoryItem;

// 暴露其他函数到全局作用域
window.toggleHistoryPage = toggleHistoryPage;
window.loadHistory = loadHistory;
window.searchHistory = searchHistory;
window.goToPage = goToPage;
window.goToPrevPage = goToPrevPage;
window.goToNextPage = goToNextPage;
window.goBack = goBack;