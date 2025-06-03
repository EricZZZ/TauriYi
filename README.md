# Tauri Yi
一个基于 Tauri 框架开发的桌面翻译应用，支持多种 AI 翻译平台和快捷键操作。

## 🌟 功能特性
### 核心功能
- 实时翻译 : 支持中文、英语、日语、韩语之间的互译
- 多平台支持 : 集成 OLLama、DeepSeek、ChatGPT、MTranServer 等多种翻译服务
- 智能语言检测 : 自动识别输入文本的语言类型
- 快捷键操作 : 支持 Cmd+J (macOS) 全局快捷键快速调用
- 剪贴板集成 : 快捷键调用时自动读取剪贴板内容
### 界面特性
- 现代化 UI : 采用深色主题，简洁美观
- 紧凑设计 : 300x350 像素的小窗口，不占用过多桌面空间
- 自适应布局 : 文本框自动调整高度，适应不同长度的文本
- 语言切换 : 支持源语言和目标语言的快速切换
- 设置面板 : 可配置 API 密钥、服务地址等参数
### 技术特性
- 跨平台 : 基于 Tauri 2.0，支持 macOS、Windows、Linux
- 高性能 : Rust 后端 + 原生前端，启动快速，资源占用低
- 模块化架构 : 清晰的代码结构，易于维护和扩展
- 配置持久化 : 设置自动保存到本地配置文件
## 🚀 快速开始
### 环境要求
- Rust 1.70+
- Tauri CLI
### 安装依赖
```shell
# 安装 Rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# 安装 Tauri CLI
cargo install tauri-cli --version "^2.0.0" --locked
```
### 开发模式
```shell
# 启动开发服务器
cargo tauri dev
```
### 构建应用
```shell
# 构建生产版本
cargo tauri build
```
## ⚙️ 配置说明
应用支持多种翻译服务，需要在设置中配置相应的 API 信息：

### OLLama (默认)
- API URL : http://localhost:11434/api/chat
- 模型 : qwen3:1.7b (可自定义)
- API Key : 本地部署无需密钥
### DeepSeek
- API URL : https://api.deepseek.com/v1/chat/completions
- API Key : 需要 DeepSeek API 密钥
### ChatGPT
- API URL : https://api.openai.com/v1/chat/completions
- API Key : 需要 OpenAI API 密钥
### MTranServer
- API URL : 自定义翻译服务地址
- API Key : 根据服务要求配置
## 🎯 使用方法
1. 启动应用 : 运行构建后的应用程序
2. 配置服务 : 点击设置图标，配置翻译服务参数
3. 快捷翻译 : 使用 Cmd+J 快捷键快速调用翻译窗口
4. 手动翻译 : 在应用界面直接输入文本进行翻译
5. 语言切换 : 使用交换按钮快速切换源语言和目标语言
## 🏗️ 项目结构
```
tauri-translate/
├── src/                    # 前端代码
│   ├── index.html         # 主界面
│   ├── main.js           # 主要逻辑
│   ├── styles.css        # 样式文件
│   └── js/               # JavaScript 模块
├── src-tauri/             # Rust 后端
│   ├── src/
│   │   ├── ai.rs         # AI 翻译接口
│   │   ├── config.rs     # 配置管理
│   │   ├── commands.rs   # Tauri 命令
│   │   ├── lang.rs       # 语言定义
│   │   └── utils.rs      # 工具函数
│   └── tauri.conf.json   # Tauri 配置
└── README.md
```
## 🛠️ 技术栈
- 前端 : HTML5 + CSS3 + Vanilla JavaScript
- 后端 : Rust + Tauri 2.0
- UI 框架 : 原生 Web 技术
- HTTP 客户端 : reqwest
- 配置管理 : serde + JSON
- 全局快捷键 : tauri-plugin-global-shortcut
- 剪贴板 : tauri-plugin-clipboard-manager
## 📝 开发说明
### 添加新的翻译服务
1. 在 config.rs 中添加新的 PlatformType
2. 在 ai.rs 中实现对应的 API 调用逻辑
3. 在前端设置页面添加相应的配置选项
### 支持新语言
1. 在 lang.rs 中添加新的语言枚举
2. 更新前端的语言选择器
3. 添加相应的语言检测逻辑
## 📄 许可证
本项目采用 MIT 许可证 - 查看 LICENSE 文件了解详情。

## 🤝 贡献
欢迎提交 Issue 和 Pull Request 来帮助改进这个项目！

注意 : 使用前请确保已正确配置相应的翻译服务 API 密钥和地址。