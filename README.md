# Rust Commit Tracker

[![Rust](https://github.com/kWAYTV/rust-commit-tracker/actions/workflows/rust.yml/badge.svg)](https://github.com/kWAYTV/rust-commit-tracker/actions/workflows/rust.yml)

A reliable Discord bot that monitors Facepunch's Rust game commits and sends real-time notifications with SQLite persistence.

## Features

- **Real-time monitoring** - Tracks new commits from Facepunch's Rust repository
- **Discord integration** - Rich embed notifications with commit details
- **SQLite persistence** - Prevents duplicate notifications across restarts
- **Auto-configuration** - Creates config file on first run
- **Automatic cleanup** - Maintains database size with configurable retention

## Quick Start

1. **Clone and build**

   ```bash
   git clone https://github.com/kWAYTV/rust-commit-tracker.git
   cd rust-commit-tracker
   cargo build --release
   ```

2. **First run** (creates config file)

   ```bash
   cargo run
   ```

3. **Configure** - Edit `config.toml` with your Discord webhook URL

4. **Run**
   ```bash
   cargo run
   ```

## Configuration

The bot creates `config.toml` on first run:

```toml
[discord]
webhook_url = "YOUR_DISCORD_WEBHOOK_URL"
bot_name = "Rust Commit Tracker"
bot_avatar_url = "https://i.imgur.com/on47Qk9.png"

[monitoring]
commits_url = "https://commits.facepunch.com/?format=json"
check_interval_secs = 50

[database]
url = "sqlite:commits.db"
cleanup_keep_last = 1000
```

## Requirements

- Rust 1.70+
- Discord webhook URL

## Files

- `config.toml` - Configuration (auto-created)
- `commits.db` - SQLite database (auto-created)
- Both files are git-ignored for security

## License

MIT
