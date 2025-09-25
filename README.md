# 🦀 Rusted-YADM (Yet Another Dotfile Manager)

<div align="center">

[![License: EUPL](https://img.shields.io/badge/License-EUPL-blue.svg)](https://opensource.org/licenses/EUPL-1.1)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://rustlang.org)
[![Crates.io](https://img.shields.io/crates/v/rusted-yadm.svg)](https://crates.io/crates/rusted-yadm)
[![Built with](https://img.shields.io/badge/Built_with-❤️-red.svg)]()

**A blazing fast, secure, and feature-rich dotfile manager written in Rust**

[Features](#-features) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Commands](#-commands) • [Configuration](#-configuration) • [Contributing](#-contributing)

</div>

## ✨ Features

### 🚀 Core Functionality
- ⚡ **Lightning Fast** - Built with Rust for maximum performance
- 🔐 **Security First** - Comprehensive symlink validation and permission preservation
- 🎣 **Git Hooks** - Automated pre-commit validation with configurable rules
- 🔒 **Encryption Support** - Built-in file encryption/decryption capabilities
- 📦 **Cross-Platform** - Works on Linux, macOS, and Windows

### 🛠️ Advanced Features
- 🔄 **Repository Management** - Full Git integration with fetch, pull, sync
- ⚙️ **Configuration System** - YAML-based configuration with environment overrides
- 🎯 **Alternate Files** - Manage environment-specific dotfiles
- 🚀 **Bootstrap Scripts** - Automated setup and initialization
- 📊 **Status & Monitoring** - Real-time repository status and file tracking

## 📦 Installation

### Prerequisites

- 🦀 **Rust and Cargo** (latest stable version)
- 🌐 **Git** (for repository operations)

### Quick Install

```bash
cargo install rusted-yadm
```

### Build from Source

1. **Clone the repository**:
   ```bash
   git clone https://git.sr.ht/~m1n/rusted-yadm
   cd rusted-yadm
   ```

2. **Build in release mode**:
   ```bash
   cargo build --release
   ```

3. **Install locally**:
   ```bash
   cargo install --path .
   ```

## 🚀 Quick Start

### 1. Initialize Your Dotfile Repository

```bash
# Initialize a new repository
rusted-yadm init

# Or clone an existing repository
rusted-yadm clone https://git.sr.ht/~m1n/dotfiles
```

### 2. Add Your First Dotfile

```bash
# Add your bash configuration
rusted-yadm add ~/.bashrc

# Add your Vim configuration
rusted-yadm add ~/.vimrc
```

### 3. Commit and Push Changes

```bash
# Commit your changes
rusted-yadm commit "Add bash and vim configurations"

# Push to remote repository
rusted-yadm push
```

### 4. Install Git Hooks (Recommended)

```bash
# Install pre-commit hooks for automatic validation
rusted-yadm hook-install
```

## 🎯 Commands

### 📁 Repository Operations

| Command | Description | Example |
|---------|-------------|---------|
| `init` | Initialize a new repository | `rusted-yadm init` |
| `clone` | Clone a repository | `rusted-yadm clone https://git.sr.ht/~user/dotfiles` |
| `status` | Show repository status | `rusted-yadm status` |
| `list` | List tracked files | `rusted-yadm list` |
| `fetch` | Fetch remote changes | `rusted-yadm fetch` |
| `pull` | Pull changes from remote | `rusted-yadm pull` |
| `sync` | Sync with remote (fetch + pull) | `rusted-yadm sync` |

### 📝 File Management

| Command | Description | Example |
|---------|-------------|---------|
| `add` | Add file to repository | `rusted-yadm add ~/.bashrc` |
| `commit` | Commit changes | `rusted-yadm commit "Update bashrc"` |
| `push` | Push to remote | `rusted-yadm push` |

### 🔐 Security & Encryption

| Command | Description | Example |
|---------|-------------|---------|
| `encrypt` | Encrypt a file | `rusted-yadm encrypt secrets.yaml` |
| `decrypt` | Decrypt a file | `rusted-yadm decrypt secrets.yaml.enc` |
| `hook-install` | Install git hooks | `rusted-yadm hook-install` |
| `hook-uninstall` | Uninstall git hooks | `rusted-yadm hook-uninstall` |
| `hook-pre-commit` | Run pre-commit validation | `rusted-yadm hook-pre-commit` |

### ⚙️ Configuration

| Command | Description | Example |
|---------|-------------|---------|
| `config` | Manage configuration | `rusted-yadm config core.encrypt true` |
| `alt` | Manage alternate files | `rusted-yadm alt create hostname` |
| `bootstrap` | Run bootstrap script | `rusted-yadm bootstrap setup.sh` |
| `version` | Show version | `rusted-yadm version` |

## 🔧 Configuration

Rusted-YADM uses YAML configuration files for advanced settings:

### Example Configuration

```yaml
# ~/.config/rusted-yadm/config.yaml
core:
  encrypt: true
  permissions: preserve

hooks:
  pre-commit:
    validate_symlinks: true
    validate_permissions: true
    validate_encrypted: true

alt:
  - class: "hostname"
    pattern: "##HOSTNAME##"
```

### Environment Variables

```bash
# Override repository location
export YADM_DIR="$HOME/.dotfiles"

# Override configuration file
export YADM_CONFIG="$HOME/.config/rusted-yadm/config.yaml"

# Set alternate file class
export YADM_CLASS="work"
```

## 🎣 Git Hooks

The built-in git hooks system provides automatic validation:

### 🔍 Validations Performed

- **Symlink Validation** - Detects broken symlinks and resolves relative paths
- **Permission Preservation** - Maintains original file permissions
- **Encrypted File Integrity** - Validates encrypted file signatures
- **Security Checks** - Prevents committing potentially dangerous files

### ⚙️ Hook Configuration

```yaml
hooks:
  pre-commit:
    # Enable/disable specific validations
    validate_symlinks: true
    validate_permissions: true
    validate_encrypted: true

    # Custom validation rules
    rules:
      - name: "no-secrets"
        pattern: "\\b(AWS_.*|API_KEY|SECRET)\\b"
        message: "Potential secret detected in file"
```

## 🤝 Contributing

We welcome contributions! Please see our [contributing guidelines](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://git.sr.ht/~m1n/rusted-yadm
cd rusted-yadm

# Install development dependencies
cargo install cargo-watch cargo-clippy

# Run tests
cargo test

# Check code style
cargo clippy -- -D warnings

# Format code
cargo fmt
```

## 📄 License

This project is licensed under the **European Union Public License (EUPL)** - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by the original [yadm](https://yadm.io/) project
- Built with amazing Rust libraries: [clap](https://github.com/clap-rs/clap), [git2](https://github.com/rust-lang/git2-rs), [serde](https://github.com/serde-rs/serde)
- Hosted on [SourceHut](https://sr.ht/) - The hacker's forge
- Thanks to all contributors and the Rust community

---

<div align="center">

Made with ❤️ by the Rusted-YADM community

[🔝 Back to top](#-rusted-yadm-yet-another-dotfile-manager)

</div>