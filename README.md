# Rust Commit Tracker

[![Rust](https://github.com/kWAYTV/rust-commit-tracker/actions/workflows/rust.yml/badge.svg)](https://github.com/kWAYTV/rust-commit-tracker/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/rust-commit-tracker.svg)](https://crates.io/crates/rust-commit-tracker)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A reliable Discord bot that monitors Facepunch's Rust game commits and sends real-time notifications with SQLite persistence.

## Features

- 🔄 **Real-time monitoring** - Tracks new commits from Facepunch's Rust repository
- 💬 **Discord integration** - Rich embed notifications with commit details
- 💾 **SQLite persistence** - Prevents duplicate notifications across restarts
- ⚙️ **Auto-configuration** - Creates config file on first run
- 🧹 **Automatic cleanup** - Maintains database size with configurable retention
- 🚀 **Zero dependencies** - Standalone executable with no runtime requirements

## Installation

### Option 1: Download Pre-built Binary (Recommended)

1. **Download the latest release** for your operating system:

   - Go to [Releases](https://github.com/kWAYTV/rust-commit-tracker/releases)
   - Download the appropriate archive for your OS:
     - `rust-commit-tracker-vX.X.X-x86_64-pc-windows-gnu.zip` (Windows)
     - `rust-commit-tracker-vX.X.X-x86_64-unknown-linux-gnu.tar.gz` (Linux)
     - `rust-commit-tracker-vX.X.X-x86_64-apple-darwin.tar.gz` (macOS)

2. **Extract and run**:
   ```bash
   # Extract the archive
   # On first run, it will create config.toml
   ./rust-commit-tracker
   ```

### Option 2: Build from Source

Requirements: [Rust 1.70+](https://rustup.rs/)

```bash
git clone https://github.com/kWAYTV/rust-commit-tracker.git
cd rust-commit-tracker
cargo build --release
./target/release/rust-commit-tracker
```

### Option 3: Install via Cargo

```bash
cargo install rust-commit-tracker
rust-commit-tracker
```

## Configuration

On first run, the application creates `config.toml`. **Only the Discord webhook URL is required** - all other settings have sensible defaults.

### Required Configuration

Edit `config.toml` and set your Discord webhook URL:

```toml
[discord]
webhook_url = "YOUR_DISCORD_WEBHOOK_URL"  # ← Only required field
```

### Full Configuration Reference

```toml
[discord]
webhook_url = "YOUR_DISCORD_WEBHOOK_URL"  # Required: Your Discord webhook
bot_name = "Rust Commit Tracker"          # Optional: Bot display name
bot_avatar_url = "https://i.imgur.com/on47Qk9.png"  # Optional: Bot avatar

[monitoring]
commits_url = "https://commits.facepunch.com/?format=json"  # API endpoint
check_interval_secs = 50  # How often to check for new commits

[appearance]
embed_color = "#CD412B"  # Rust orange color for Discord embeds
footer_icon_url = "https://i.imgur.com/on47Qk9.png"

[database]
url = "sqlite:commits.db"  # SQLite database location
cleanup_keep_last = 1000   # Number of commits to retain in database
```

## Getting a Discord Webhook URL

1. Open your Discord server settings
2. Go to **Integrations** → **Webhooks**
3. Click **New Webhook**
4. Choose the channel and copy the webhook URL
5. Paste it into your `config.toml` file

## Usage

After configuration, simply run the executable:

```bash
./rust-commit-tracker
```

The bot will:

- Start monitoring Facepunch's commit feed
- Send notifications for new commits to your Discord channel
- Maintain a local database to prevent duplicate notifications
- Automatically resume from the last processed commit after restarts

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Follow [Conventional Commits](https://www.conventionalcommits.org/) format
4. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- 🐛 **Issues**: [GitHub Issues](https://github.com/kWAYTV/rust-commit-tracker/issues)
- 📖 **Documentation**: [GitHub Repository](https://github.com/kWAYTV/rust-commit-tracker)
- 📦 **Crate**: [crates.io](https://crates.io/crates/rust-commit-tracker)
