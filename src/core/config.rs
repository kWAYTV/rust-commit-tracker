use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::error::Error;
use std::io::{self, Write};

const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub discord: DiscordConfig,
    pub monitoring: MonitoringConfig,
    pub appearance: AppearanceConfig,
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

impl Config {
    pub fn load_or_create() -> Result<Self, Box<dyn Error>> {
        if Path::new(CONFIG_FILE).exists() {
            Self::load_from_file()
        } else {
            Self::create_default_and_prompt()
        }
    }

    fn load_from_file() -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(CONFIG_FILE)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    fn create_default_and_prompt() -> Result<Self, Box<dyn Error>> {
        println!("🔧 First time setup - Creating configuration file...");
        
        let default_config = Self::default();
        let toml_content = toml::to_string_pretty(&default_config)?;
        
        fs::write(CONFIG_FILE, &toml_content)?;
        
        println!("✅ Created '{}'", CONFIG_FILE);
        println!();
        println!("📝 Please edit the configuration file with your settings:");
        println!("   - Discord webhook URL");
        println!("   - Bot name and avatar");
        println!("   - Monitoring settings");
        println!();
        print!("Press Enter when you've finished editing the config file...");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        // Reload the config after user edits
        Self::load_from_file()
    }

    pub fn rust_color(&self) -> u32 {
        // Parse hex color string to u32
        if self.appearance.embed_color.starts_with('#') {
            u32::from_str_radix(&self.appearance.embed_color[1..], 16)
                .unwrap_or(0xCD412B)
        } else {
            u32::from_str_radix(&self.appearance.embed_color, 16)
                .unwrap_or(0xCD412B)
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
        }
    }
} 