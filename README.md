# 🧱 CraftIDE

**CraftIDE** is a **desktop Minecraft mod creation program** built with **Svelte** and **Tauri**, targeting **Fabric for Minecraft Java Edition 26.1+**.

CraftIDE is designed to **accelerate mod development without boxing developers in**. It generates boilerplate for common mod elements (blocks, items, etc.) while **always allowing full manual control and custom code**.
---

## ✨ Features

* ⚡ **Fast, lightweight desktop app**. With a svelte frontend + rust backend, CraftIDE is optimized for low resource consumption and good performance
* 🧩 **Class generation** for:
  * Blocks
  * Items
  * Recipes
  * More mod components over time
* ✍️ **Fully editable generated code**

  * Modify generated classes freely
  * Write your own Java code alongside generated files
* 🎨 **Built-in texture editor**

  * Simple pixel editor for block & item textures
  * Asset previews
* 📁 **Project & asset management**
* 🧠 **Mod metadata & config generation**
  * Mod ID
  * Versions
  * Dependencies
* 🖥️ **Cross-platform**

  * Windows
  * macOS
  * Linux

> CraftIDE helps you move faster, but never hides how Minecraft modding actually works.

---

## 🧰 Tech Stack

* **Frontend:** [Svelte](https://svelte.dev)
* **Desktop Runtime:** [Tauri](https://tauri.app)
* **Language:** TypeScript / JavaScript
* **Minecraft Target:** Java Edition **26.1+**

---

## 📦 Installation
In the future, the program will be installable through a website, but it must be compiled manually for now

### Prerequisites

Ensure the following are installed:

* **Node.js** (LTS recommended)
* **Rust** (required for Tauri)
* **Java JDK** (for Minecraft mod development)

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

### Clone & Install

```bash
git clone https://github.com/dokko-games/CraftIDE.git
cd CraftIDE
npm install
```

---

## 🚀 Development

Run CraftIDE in development mode:

```bash
npm run tauri dev
```

This launches the Svelte UI inside a native desktop window with hot reload enabled.

---

## 🏗️ Build

Create a production build:

```bash
npm run tauri build
```

The compiled desktop binaries will be available in the `src-tauri/target/` directory.

---

## 🧠 Philosophy

CraftIDE follows a few core principles:

* **Generated code is yours** — no lock-in, no hidden abstractions
* **Manual coding is first-class** — generators are helpers, not rules
* **Modern Minecraft support** — built for 26.1+ and future versions
* **Learning-friendly** — see real mod code as it’s created

---
## 🤝 Contributing

Contributions, ideas, and feedback are welcome.

* Open an issue for bugs or feature requests
* Submit pull requests for improvements


Happy modding! 🛠️
