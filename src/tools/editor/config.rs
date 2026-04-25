
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use crate::tools::editor::theme::{Theme, Color};
use crate::tools::editor::cursor::LineNumberMode;
use crate::tools::editor::error::{Result, EditorError};
use dirs;
use toml;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    pub theme: ThemeConfig,
    pub editor: EditorBehaviorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    #[serde(flatten)]
    pub colors: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorBehaviorConfig {
    pub tab_size: usize,
    pub expand_tab: bool,
    pub auto_indent: bool,
    pub line_numbers: LineNumberMode,
    pub rulers: Vec<usize>,
    pub scrolloff: usize,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            theme: ThemeConfig {
                name: "幻语默认".to_string(),
                colors: std::collections::HashMap::new(),
            },
            editor: EditorBehaviorConfig {
                tab_size: 4,
                expand_tab: true,
                auto_indent: true,
                line_numbers: LineNumberMode::Absolute,
                rulers: vec![80, 120],
                scrolloff: 8,
            },
        }
    }
}

impl EditorConfig {
    pub fn default_config_path() -> Option<PathBuf> {
        if let Some(home_dir) = dirs::home_dir() {
            let config_dir = home_dir.join(".huan");
            Some(config_dir.join("config.toml"))
        } else {
            None
        }
    }
    
    pub fn load() -> Result<Self> {
        if let Some(path) = Self::default_config_path() {
            if path.exists() {
                let content = fs::read_to_string(&path)?;
                let config: EditorConfig = toml::from_str(&content)
                    .map_err(|e| EditorError::ConfigError(e.to_string()))?;
                return Ok(config);
            }
        }
        Ok(Self::default())
    }
    
    pub fn load_from_path(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: EditorConfig = toml::from_str(&content)
            .map_err(|e| EditorError::ConfigError(e.to_string()))?;
        Ok(config)
    }
    
    pub fn save(&self) -> Result<()> {
        if let Some(path) = Self::default_config_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let content = toml::to_string_pretty(self)
                .map_err(|e| EditorError::ConfigError(e.to_string()))?;
            fs::write(path, content)?;
        }
        Ok(())
    }
    
    pub fn to_theme(&self) -> Theme {
        let mut theme = Theme::default();
        theme.name = self.theme.name.clone();
        
        for (key, value) in &self.theme.colors {
            match key.as_str() {
                "background" => theme.background = Color::from_hex(value),
                "foreground" => theme.foreground = Color::from_hex(value),
                "keyword" => theme.keyword = Color::from_hex(value),
                "type_name" => theme.type_name = Color::from_hex(value),
                "function" => theme.function = Color::from_hex(value),
                "string" => theme.string = Color::from_hex(value),
                "comment" => theme.comment = Color::from_hex(value),
                "number" => theme.number = Color::from_hex(value),
                "operator" => theme.operator = Color::from_hex(value),
                "selection" => theme.selection = Color::from_hex(value),
                "line_number" => theme.line_number = Color::from_hex(value),
                "status_line_bg" => theme.status_line_bg = Color::from_hex(value),
                "status_line_fg" => theme.status_line_fg = Color::from_hex(value),
                "error" => theme.error = Color::from_hex(value),
                "warning" => theme.warning = Color::from_hex(value),
                "info" => theme.info = Color::from_hex(value),
                _ => {}
            }
        }
        
        theme
    }
}
