#[derive(Debug, Clone)]
pub struct Config {
    pub discord_webhook_url: String,
    pub commits_url: String,
    pub check_interval_secs: u64,
    pub rust_color: u32,
    pub footer_icon_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            discord_webhook_url: "https://discord.com/api/webhooks/1377062435000680469/p8IPNirCl90kWHSpmX-YMEDxeNKdaGOUtofNY_jSX_B-w_3vMSXymIaKuvVVP71Xtlnq".to_string(),
            commits_url: "https://commits.facepunch.com/r/rust_reboot".to_string(),
            check_interval_secs: 50,
            rust_color: 0xCD412B,
            footer_icon_url: "https://i.imgur.com/on47Qk9.png".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
} 