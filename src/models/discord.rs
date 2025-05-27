use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DiscordEmbed {
    pub embeds: Vec<EmbedData>,
}

#[derive(Debug, Serialize)]
pub struct EmbedData {
    pub title: String,
    pub description: String,
    pub color: u32,
    pub author: EmbedAuthor,
    pub fields: Vec<EmbedField>,
    pub footer: EmbedFooter,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: String,
    pub icon_url: String,
}

#[derive(Debug, Serialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Debug, Serialize)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: String,
} 