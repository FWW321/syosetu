use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use regex::Regex;
use reqwest::Client;
use reqwest::header;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::config::get_config;
use crate::utils;

lazy_static! {
    static ref NCODE_RE: Regex = Regex::new(r"/usernovelmanage/top/ncode/(\d+)/").unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub author: Option<String>,
    pub language: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub subject: Vec<String>,
}

impl Metadata {
    pub async fn load(metadata_path: &Path) -> Result<Self> {
        println!("加载小说元数据: {}", metadata_path.display());
        let file_content = fs::read_to_string(metadata_path).await?;

        config::Config::builder()
            .add_source(config::File::from_str(
                &file_content,
                config::FileFormat::Toml,
            ))
            .build()?
            .try_deserialize()
            .map_err(|e| anyhow::anyhow!("加载小说信息失败: {}", e))
    }

    pub async fn create(&self, client: &Client) -> Result<String> {
        println!("创建小说: {}", self.title);
        let mut form_data = HashMap::new();
        form_data.insert("title", self.title.as_str());
        form_data.insert("mode1", "保存する");

        let resp = client
            .post(format!("{}/usernovel/add/", get_config().base_url))
            .form(&form_data)
            .send()
            .await?;

        if !resp.status().is_redirection() {
            anyhow::bail!("添加书籍失败，状态码: {}", resp.status());
        }

        // location /usernovelmanage/top/ncode/2918565/
        let Some(location) = resp.headers().get(header::LOCATION) else {
            anyhow::bail!("未能获取到重定向地址");
        };

        let Some(redirect_url) = location.to_str().ok() else {
            anyhow::bail!("重定向地址格式错误");
        };

        let ncode = utils::extract_id_from_url(redirect_url, &NCODE_RE)?;
        Ok(ncode)
    }

    pub async fn update_novel_info(&self, client: &Client, novel_id: &str) -> Result<()> {
        println!("更新小说信息: {}", novel_id);
        let update_page = client
            .get(format!(
                "{}/draftnovelmanage/updateinput/ncode/{}/",
                get_config().base_url,
                novel_id
            ))
            .send()
            .await?
            .text()
            .await?;

        let post_url = utils::extract_form_action(&update_page, "usernovelmanageForm")?;

        let mut form_data = HashMap::new();
        // 标题
        form_data.insert("title", self.title.as_str());
        if let Some(author) = &self.author {
            form_data.insert("writer_radio", "1");
            form_data.insert("writer", author.as_str());
        } else {
            // 作者
            // - 0：与用户名相同
            // - 1：自定义
            form_data.insert("writer_radio", "0");
            // 作者名称：作者选1填写
            form_data.insert("writer", "");
        }
        // 作品种类
        // - 1：连载
        // - 2：短篇
        form_data.insert("noveltype", "1");
        // 年龄限制
        // - 1: 所有年龄
        form_data.insert("age_limit", "1");
        // R15标记
        // form_data.insert("isr15", "1");
        form_data.insert("nocgenre", "1");
        // 分类
        // - 1：原创
        form_data.insert("classification", "1");
        form_data.insert("ff_type", "1");
        form_data.insert("ff_keyword", "");
        form_data.insert("ff_ncode", "");
        form_data.insert("trpg_replay_type", "1");
        form_data.insert("trpg_replay_keyword_id", "");
        form_data.insert("scenario_author", "");
        form_data.insert("scenario_name", "");
        // 主要类别
        // - 1：恋爱
        form_data.insert("biggenre", "1");
        // 次要类别
        // - 101：异世界
        form_data.insert("genre", "101");
        // 简介
        form_data.insert("ex", self.description.as_deref().unwrap_or("这是简介, 并且有14个字符"));
        // 作品中包含的元素
        // 参考描写
        // form_data.insert("iszankoku", "1");
        // bl
        // form_data.insert("isbl", "0");
        // gl
        // form_data.insert("isgl", "0");
        // 转生
        // form_data.insert("istensei", "1");
        // 穿越
        // form_data.insert("istenni", "1");
        // 选择关键字
        form_data.insert("auto_keyword_array", "");
        let mut keywords = "".to_string();
        // 自定义关键词
        if !self.subject.is_empty() {
            keywords = vec_to_json_string(&self.subject);
        }
        form_data.insert("unique_keyword_array", &keywords);
        form_data.insert("is_monetized", "0");

        client
            .post(format!("{}/{}", get_config().base_url, post_url))
            .form(&form_data)
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_novel_setting(client: &Client, novel_id: &str) -> Result<()> {
        let settings_page = client
            .get(format!(
                "{}/draftnovelmanage/receptionsinput/ncode/{}/",
                get_config().base_url,
                novel_id
            ))
            .send()
            .await?
            .text()
            .await?;

        let post_url = utils::extract_form_action(&settings_page, "usernovelmanageForm")?;

        let mut form_data = HashMap::new();
        form_data.insert("notkansou", "0");
        form_data.insert("notreview", "0");
        form_data.insert("notpoint", "0");
        form_data.insert("notpointview", "0");
        form_data.insert("notreport", "0");
        // 搜索排除设置
        // - 0：包含在搜索中
        // - 1：从搜索和列表中排除
        let not_search = get_config().novel.searchable.negate();
        form_data.insert("notsearch", not_search.as_str());
        form_data.insert("csrf_onetimepass", "");

        client
            .post(format!("{}/{}", get_config().base_url, post_url))
            .form(&form_data)
            .send()
            .await?;

        Ok(())
    }
}

fn vec_to_json_string(values: &[String]) -> String {
    let items: Vec<String> = values
        .iter()
        .map(|s| format!(r#"{{"value":"{}"}}"#, s))
        .collect();

    format!("[{}]", items.join(","))
}
