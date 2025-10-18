use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;
use reqwest::header;
use std::collections::HashMap;
use tokio::fs::File;


use crate::utils;
use crate::config::get_config;

lazy_static! {
    static ref DRAFT_RE: Regex = Regex::new(r"/draftepisode/view/draftepisodeid/(\d+)/").unwrap();
}

pub struct Chapter {
    pub title: String,
    pub content: String,
}

impl Chapter {
    pub async fn read(chapter_path: &Path) -> Result<Self> {
        println!("加载章节: {}", chapter_path.display());
        let file = File::open(chapter_path).await?;
        let reader = BufReader::new(file);

        let mut title = String::new();
        let mut content = String::new();
        let mut found_title = false;

        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            if !found_title {
                if !line.trim().is_empty() {
                    title = line.trim().to_string();
                    found_title = true;
                }
            } else {
                content.push_str(&line);
                content.push('\n');
            }
        }

        // 确保至少有一个标题
        if title.is_empty() {
            return Err(anyhow::anyhow!(
                "No title found in chapter file: {}",
                chapter_path.display()
            ));
        }

        // 移除内容末尾多余的换行符
        content = content.trim_end().to_string();

        Ok(Chapter { title, content })
    }

    // 保存草稿
    pub async fn add_draft(&self, client: &Client, novel_id: &str) -> Result<String> {
        println!("添加草稿: {}", self.title);
        let add_chapter_page = client
            .get(format!(
                "{}/draftepisode/input/ncode/{}/",
                get_config().base_url,
                novel_id
            ))
            .send()
            .await?
            .text()
            .await?;

        let csrf_token = utils::extract_csrf_token(&add_chapter_page)?;

        let mut form_data = HashMap::new();
        // 章节标题
        form_data.insert("subtitle", self.title.as_str());
        // 章节内容 要发布必须大于200个字符
        form_data.insert("novel", self.content.as_str());
        // 前言
        form_data.insert("preface", "");
        // 后记
        form_data.insert("postscript", "");
        // 文件上传大小限制
        form_data.insert("MAX_FILE_SIZE", "1048576");
        // 上传的文件
        form_data.insert("novel-file", "");
        // CSRF token
        form_data.insert("csrf_onetimepass", &csrf_token); // 从页面获取的 CSRF token
        // 备注
        form_data.insert("freememo", "");

        let resp = client
            .post(format!(
                "{}/draftepisode/add/ncode/{}/",
                get_config().base_url,
                novel_id
            ))
            .form(&form_data)
            .send()
            .await?;

        if !resp.status().is_redirection() {
            anyhow::bail!("添加草稿失败，状态码: {}", resp.status());
        }

        // location /draftepisode/view/draftepisodeid/4872097/
        let Some(location) = resp.headers().get(header::LOCATION) else {
            anyhow::bail!("未能获取到重定向地址");
        };

        let Some(redirect_url) = location.to_str().ok() else {
            anyhow::bail!("重定向地址格式错误");
        };

        let draft_id = utils::extract_id_from_url(redirect_url, &DRAFT_RE)?;
        println!("草稿添加成功, 草稿ID: {}", draft_id);

        Ok(draft_id)
    }

    pub async fn publish_draft(&self, client: &Client, draft_id: &str) -> Result<()> {
        println!("发布草稿: {}", draft_id);
        if !self.is_published() {
            println!("章节内容不足200字符, 跳过发布。");
            return Ok(());
        }
        let resp = client
            .get(&format!(
                "{}/draftepisode/postconfirmapi/?draftepisodeid={}&reserve=off&end=1&_={}",
                get_config().base_url,
                draft_id,
                chrono::Utc::now().timestamp_millis() // 当前时间戳
            ))
            .send()
            .await?;

        if !resp.status().is_success() {
            anyhow::bail!("发布草稿失败，状态码: {}", resp.status());
        }

        client
            .post(&format!("{}/draftepisode/postapi/", get_config().base_url))
            .form(&[("draftepisodeid", draft_id)])
            .send()
            .await?;
        Ok(())
    }

    fn is_published(&self) -> bool {
        self.content.chars().count() > 200
    }
}
