mod chapter;
mod metadata;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use tokio::task;
use anyhow::Result;
use reqwest::cookie::Jar;
use reqwest::{Client, Url};

use crate::config::{SyosetuConfig, get_config};
use chapter::Chapter;
use metadata::Metadata;

pub fn create_client(config: &SyosetuConfig) -> Result<Client> {
    let jar = create_cookie_jar(config)?;

    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36 Edg/141.0.0.0")
        .cookie_provider(jar)
        .build()?;

    Ok(client)
}

/// 创建包含认证信息的 Cookie Jar
fn create_cookie_jar(config: &SyosetuConfig) -> Result<Arc<Jar>> {
    let jar = Jar::default();
    let url = Url::parse(&config.base_url)?;

    jar.add_cookie_str(
        &format!("ses={}; domain=syosetu.com", config.cookie.ses),
        &url,
    );
    jar.add_cookie_str(
        &format!("userl={}; domain=syosetu.com", config.cookie.userl),
        &url,
    );

    Ok(Arc::new(jar))
}

pub async fn upload_novel(client: &Client) -> Result<()> {
    let data_path = Path::new(get_config().data_dir.as_str());
    if !data_path.exists() {
        anyhow::bail!("数据目录不存在: {}", data_path.display());
    }

    let mut join_set = task::JoinSet::new();

    for entry in std::fs::read_dir(data_path)? {
        let entry = entry?;
        let book_dir = entry.path();
        if book_dir.is_dir() {
            join_set.spawn(process_book(book_dir, client.clone()));
        }
    }
    while let Some(res) = join_set.join_next().await {
        res??;
    }
    println!("上传完成");
    Ok(())
}

async fn process_book(book_dir: PathBuf, client: Client) -> Result<()> {
            let metadata_path = book_dir.join("metadata.toml");
            if !metadata_path.exists() {
                anyhow::bail!("小说元数据文件不存在: {}", metadata_path.display());
            }
            let chapters_dir = book_dir.join("chapters");
            if !(chapters_dir.exists() && chapters_dir.is_dir()) {
                anyhow::bail!("章节目录不存在: {}", chapters_dir.display());
            }
            let metadata = Metadata::load(&metadata_path).await?;
            let novel_id = metadata.create(&create_client(&get_config())?).await?;
            metadata.update_novel_info(&client, &novel_id).await?;
            Metadata::update_novel_setting(&client, &novel_id).await?;

            let mut chapter_files: Vec<_> = std::fs::read_dir(&chapters_dir)?
                .filter_map(|res| res.ok())
                .filter(|e| {
                    let path = e.path();
                    path.is_file() && path.extension().map_or(false, |ext| ext == "txt")
                })
                .collect();

            chapter_files.sort_by(|a, b| {
                let a_num = extract_chapter_number(&a.path());
                let b_num = extract_chapter_number(&b.path());
                a_num.cmp(&b_num)
            });

            for chapter_entry in chapter_files {
                let chapter_path = chapter_entry.path();
                let chapter = Chapter::read(&chapter_path).await?;
                let draft_id = chapter.add_draft(&client, &novel_id).await?;
                chapter.publish_draft(&client, &draft_id).await?;
            }
    Ok(())
}

fn extract_chapter_number(path: &Path) -> u32 {
    path.file_name()
        .and_then(|name| name.to_str())
        .and_then(|s| s.strip_prefix("chapter_"))
        .and_then(|s| s.strip_suffix(".txt"))
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0)
}
