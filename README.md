# SysInfoPro
![Version](https://img.shields.io/badge/version-0.1.0-green.svg)

一個基於 Rust 和 Slint UI 打造的高性能系統監控工具，專為各式處理器（如 AMD Ryzen 9 5900XT）優化。

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![UI](https://img.shields.io/badge/UI-Slint-blueviolet.svg)

## ✨ 特色
- **高性能核心監控**：支援 32 執行緒同步顯示，採用 `GridLayout` 排版。
- **精美 Cupertino 風格**：內建 macOS 質感的 UI 元件。
- **輕量化設計**：Release 版本僅約 21MB，啟動迅速。

## 🛠️ 技術棧
- **核心**: Rust / **UI 框架**: Slint
- **數據抓取**: sysinfo-rs / **風格**: Cupertino

## 🚀 快速開始
### 前置要求 (Fedora)
```bash
sudo dnf install fontconfig-devel
```

###    編譯與執行
```bash
git clone https://github.com/masato-ro/sysinfopro.git
cd sysinfopro
SLINT_STYLE=cupertino cargo run --release
```

## 📝 開發筆記
- **針對 5900XT 優化**：使用 `ScrollView` 配合雙欄式 `GridLayout`，在 850x950 視窗中完美呈現 32 核心負載。
- **環境變數**：內嵌 `SLINT_STYLE=cupertino` 設定，確保視覺一致性。

## 📄 授權條款
本專案採用 MIT 授權條款。
