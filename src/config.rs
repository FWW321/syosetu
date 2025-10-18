use std::sync::LazyLock;

use anyhow::Result;
use serde::{Deserialize, Serialize};

static CONFIG: LazyLock<SyosetuConfig> = LazyLock::new(|| {
    SyosetuConfig::load().expect("无法加载配置，请确保 config.toml 存在且格式正确")
});

/// Syosetu 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyosetuConfig {
    /// 网站基础 URL
    #[serde(default = "default_base_url")]
    pub base_url: String,

    #[serde(default = "default_data_dir")]
    pub data_dir: String,

    /// 认证信息
    pub cookie: CookieConfig,

    /// 小说默认设置
    #[serde(default)]
    pub novel: Novel,
}

impl SyosetuConfig {
    pub fn cookie(&self) -> &CookieConfig {
        &self.cookie
    }

    pub fn novel(&self) -> &Novel {
        &self.novel
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieConfig {
    /// 会话 cookie (ses)
    pub ses: String,

    /// 用户登录 cookie (userl)
    pub userl: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Novel {
    /// 是否可被搜索 (0: 不可以, 1: 可以)
    #[serde(default = "default_search")]
    pub searchable: YesNo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum YesNo {
    No,
    Yes,
}

impl YesNo {
    pub fn new(value: bool) -> Self {
        if value { YesNo::Yes } else { YesNo::No }
    }

    pub fn negate(&self) -> Self {
        match self {
            YesNo::Yes => YesNo::No,
            YesNo::No => YesNo::Yes,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            YesNo::Yes => "1",
            YesNo::No => "0",
        }
    }
}

impl Default for Novel {
    fn default() -> Self {
        Self {
            searchable: default_search(),
        }
    }
}

fn default_base_url() -> String {
    "https://syosetu.com".to_string()
}

fn default_search() -> YesNo {
    YesNo::Yes
}

fn default_data_dir() -> String {
    "./output".to_string()
}

impl SyosetuConfig {
    fn load() -> Result<Self> {
        config::Config::builder()
            .add_source(config::File::with_name("config").format(config::FileFormat::Toml))
            .build()?
            .try_deserialize()
            .map_err(|e| anyhow::anyhow!("加载配置失败: {}", e))
    }
}

pub fn get_config() -> &'static SyosetuConfig {
    &CONFIG
}
