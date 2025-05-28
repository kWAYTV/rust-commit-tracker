use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub discord: DiscordConfig,
    pub monitoring: MonitoringConfig,
    pub appearance: AppearanceConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordConfig {
    pub webhook_url: String,
    pub bot_name: String,
    pub bot_avatar_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub commits_url: String,
    pub check_interval_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub embed_color: String,
    pub footer_icon_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub cleanup_keep_last: i64,
}

impl Config {
    pub fn load_or_create() -> Result<Self, Box<dyn Error>> {
        if Path::new(CONFIG_FILE).exists() {
            let config = Self::load_from_file()?;
            config.validate()?;
            Ok(config)
        } else {
            Self::create_default_and_prompt()
        }
    }

    fn load_from_file() -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(CONFIG_FILE)?;

        // Try to parse the config, but handle missing fields gracefully
        match toml::from_str::<Config>(&content) {
            Ok(config) => Ok(config),
            Err(e) => {
                // If parsing fails due to missing fields, merge with defaults
                if e.to_string().contains("missing field") {
                    println!("⚠️  Config file is missing new fields, updating...");

                    // Parse as a generic value first
                    let mut existing: toml::Value = toml::from_str(&content)?;
                    let default_config = Self::default();
                    let default_value = toml::Value::try_from(&default_config)?;

                    // Merge missing fields from defaults
                    if let (toml::Value::Table(existing_table), toml::Value::Table(default_table)) =
                        (&mut existing, default_value)
                    {
                        for (key, value) in default_table {
                            if !existing_table.contains_key(&key) {
                                existing_table.insert(key, value);
                            }
                        }
                    }

                    // Convert back to Config and save the updated version
                    let updated_config: Config = existing.try_into()?;
                    let updated_content = toml::to_string_pretty(&updated_config)?;
                    fs::write(CONFIG_FILE, &updated_content)?;

                    println!("✅ Config file updated with new fields");
                    Ok(updated_config)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    fn create_default_and_prompt() -> Result<Self, Box<dyn Error>> {
        println!("🔧 First time setup - Creating configuration file...");

        let default_config = Self::default();
        let toml_content = toml::to_string_pretty(&default_config)?;

        fs::write(CONFIG_FILE, &toml_content)?;

        println!("✅ Created '{}'", CONFIG_FILE);
        println!();
        println!("📝 Please edit the configuration file with your settings:");
        println!("   - Discord webhook URL (REQUIRED)");
        println!("   - Bot name and avatar (optional)");
        println!("   - Monitoring settings (optional)");
        println!("   - Database path (optional)");
        println!();
        print!("Press Enter when you've finished editing the config file...");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // Reload and validate the config after user edits
        let config = Self::load_from_file()?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        // Check if webhook URL is still the placeholder
        if self.discord.webhook_url == "REPLACE_WITH_YOUR_DISCORD_WEBHOOK_URL" 
            || self.discord.webhook_url.trim().is_empty() {
            return Err(format!(
                "❌ Discord webhook URL not configured!\n\
                Please edit '{}' and set a valid Discord webhook URL.\n\
                You can get one from your Discord server settings → Integrations → Webhooks",
                CONFIG_FILE
            ).into());
        }

        // Basic webhook URL validation
        if !self.discord.webhook_url.starts_with("https://discord.com/api/webhooks/") 
            && !self.discord.webhook_url.starts_with("https://discordapp.com/api/webhooks/") {
            return Err(format!(
                "❌ Invalid Discord webhook URL format!\n\
                Expected: https://discord.com/api/webhooks/...\n\
                Got: {}",
                self.discord.webhook_url
            ).into());
        }

        Ok(())
    }

    pub fn rust_color(&self) -> u32 {
        // Parse hex color string to u32
        if self.appearance.embed_color.starts_with('#') {
            u32::from_str_radix(&self.appearance.embed_color[1..], 16).unwrap_or(0xCD412B)
        } else {
            u32::from_str_radix(&self.appearance.embed_color, 16).unwrap_or(0xCD412B)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            discord: DiscordConfig {
                webhook_url: "REPLACE_WITH_YOUR_DISCORD_WEBHOOK_URL".to_string(),
                bot_name: "Rust Commit Tracker".to_string(),
                bot_avatar_url: "https://i.imgur.com/on47Qk9.png".to_string(),
            },
            monitoring: MonitoringConfig {
                commits_url: "https://commits.facepunch.com/?format=json".to_string(),
                check_interval_secs: 50,
            },
            appearance: AppearanceConfig {
                embed_color: "#CD412B".to_string(), // Rust orange
                footer_icon_url: "https://i.imgur.com/on47Qk9.png".to_string(),
            },
            database: DatabaseConfig {
                url: "sqlite:commits.db".to_string(),
                cleanup_keep_last: 1000,
            },
        }
    }
}
