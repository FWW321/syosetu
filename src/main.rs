use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Ok, Result};
use regex::Regex;
use reqwest::cookie::Jar;
use reqwest::{Client, Url, header};
use scraper::{Html, Selector};

static BASE_URL: &str = "https://syosetu.com";

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none()) // 禁用自动重定向
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36 Edg/141.0.0.0")
        .cookie_provider(get_jar())
        .build()?;

    let novel_id = create(
        &client,
        "Nhỏ đàn em của tôi dường như đang quá nhập tâm vào vai diễn Yandere rồi thì phải?",
    )
    .await?;

    update_novel_setting(&client, &novel_id).await?;

    update_novel_info(&client, &novel_id).await?;

    let title = "Chương 1: Yandere là gì vậy?";
    let content = r#" 
    <h1>Chương 1: Yandere là gì vậy?</h1>
    <p>Nội dung chương 1: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.</p>
    <p>Nội dung chương 1 tiếp tục: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    <p>Nội dung chương 1 kết thúc: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
    "#;

    let draft_id = add_draft(&client, &novel_id, title, content).await?;
    publish_draft(&client, &draft_id).await?;
    Ok(())
}

async fn publish_draft(client: &Client, draft_id: &str) -> Result<()> {
    let resp = client
        .get(&format!(
            "{}/draftepisode/postconfirmapi/?draftepisodeid={}&reserve=off&end=1&_={}",
            BASE_URL,
            draft_id,
            chrono::Utc::now().timestamp_millis() // 当前时间戳
        ))
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("发布草稿失败，状态码: {}", resp.status());
    }

    client
        .post(&format!("{}/draftepisode/postapi/", BASE_URL))
        .form(&[("draftepisodeid", draft_id)])
        .send()
        .await?;
    Ok(())
}

// 保存草稿
async fn add_draft(client: &Client, novel_id: &str, title: &str, content: &str) -> Result<String> {
    let add_chapter_page = client
        .get(format!(
            "{}/draftepisode/input/ncode/{}/",
            BASE_URL, novel_id
        ))
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&add_chapter_page);
    let selector = Selector::parse("input[name='csrf_onetimepass']").unwrap();
    let Some(csrf_token_element) = document.select(&selector).next() else {
        anyhow::bail!("未找到 CSRF token 元素");
    };
    let Some(csrf_token) = csrf_token_element.value().attr("value") else {
        anyhow::bail!("未找到 CSRF token 的值");
    };

    let mut form_data = HashMap::new();
    // 章节标题
    form_data.insert("subtitle", title);
    // 章节内容 要发布必须大于200个字符
    form_data.insert("novel", content);
    // 前言
    form_data.insert("preface", "");
    // 后记
    form_data.insert("postscript", "");
    // 文件上传大小限制
    form_data.insert("MAX_FILE_SIZE", "1048576");
    // 上传的文件
    form_data.insert("novel-file", "");
    // CSRF token
    form_data.insert("csrf_onetimepass", csrf_token); // 从页面获取的 CSRF token
    // 备注
    form_data.insert("freememo", "");

    let resp = client
        .post(format!(
            "{}/draftepisode/add/ncode/{}/",
            BASE_URL, novel_id
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

    let re = Regex::new(r"/draftepisode/view/draftepisodeid/(\d+)/")?;

    let Some(caps) = re.captures(redirect_url) else {
        anyhow::bail!("未能获取到重定向地址");
    };

    let draft_id = caps.get(1).unwrap().as_str();
    println!("草稿添加成功, 草稿ID: {}", draft_id);

    Ok(draft_id.to_string())
}

async fn create(client: &Client, title: &str) -> Result<String> {
    let mut form_data = HashMap::new();
    form_data.insert("title", title);
    form_data.insert("mode1", "保存する");

    let resp = client
        .post(format!("{}/usernovel/add/", BASE_URL))
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

    let re = Regex::new(r"/usernovelmanage/top/ncode/(\d+)/")?;

    let Some(caps) = re.captures(redirect_url) else {
        anyhow::bail!("未能获取到重定向地址");
    };

    let ncode = caps.get(1).unwrap().as_str();
    Ok(ncode.to_string())
}

async fn update_novel_setting(client: &Client, novel_id: &str) -> Result<()> {
    let settings_page = client
        .get(format!(
            "{}/draftnovelmanage/receptionsinput/ncode/{}/",
            BASE_URL, novel_id
        ))
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&settings_page);
    let form_selector = Selector::parse("form#usernovelmanageForm").unwrap();
    let Some(form_element) = document.select(&form_selector).next() else {
        anyhow::bail!("未找到设置表单");
    };
    let Some(post_url) = form_element.value().attr("action") else {
        anyhow::bail!("未找到表单的 action 属性");
    };
    let mut form_data = HashMap::new();
    form_data.insert("notkansou", "0");
    form_data.insert("notreview", "0");
    form_data.insert("notpoint", "0");
    form_data.insert("notpointview", "0");
    form_data.insert("notreport", "0");
    // 搜索排除设置
    // - 0：包含在搜索中
    // - 1：从搜索和列表中排除
    form_data.insert("notsearch", "1");
    form_data.insert("csrf_onetimepass", "");

    client
        .post(format!("{}/{}", BASE_URL, post_url))
        .form(&form_data)
        .send()
        .await?;

    Ok(())
}

async fn update_novel_info(client: &Client, novel_id: &str) -> Result<()> {
    let update_page = client
        .get(format!(
            "{}/draftnovelmanage/updateinput/ncode/{}/",
            BASE_URL, novel_id
        ))
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&update_page);
    let form_selector = Selector::parse("form#usernovelmanageForm").unwrap();
    let Some(form_element) = document.select(&form_selector).next() else {
        anyhow::bail!("未找到更新表单");
    };
    let Some(post_url) = form_element.value().attr("action") else {
        anyhow::bail!("未找到表单的 action 属性");
    };

    let mut form_data = HashMap::new();
    // 标题
    form_data.insert(
        "title",
        "Nhỏ đàn em của tôi dường như đang quá nhập tâm vào vai diễn Yandere rồi thì phải?",
    );
    // 作者
    // - 0：与用户名相同
    // - 1：自定义
    form_data.insert("writer_radio", "0");
    // 作者名称：作者选1填写
    form_data.insert("writer", "");
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
    form_data.insert("ex", "简介简介简介简介简介简介");
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
    // 自定义关键词
    form_data.insert("unique_keyword_array", r#"[{"value":"111"},{"value":"2"}]"#);
    form_data.insert("is_monetized", "0");

    client
        .post(format!("{}/{}", BASE_URL, post_url))
        .form(&form_data)
        .send()
        .await?;
    Ok(())
}

fn get_jar() -> Arc<Jar> {
    let jar = Jar::default();
    let url = Url::parse(BASE_URL).unwrap();
    jar.add_cookie_str("ses=2rrn1h3ursvscrvrngnuf7o7br; domain=syosetu.com", &url);
    jar.add_cookie_str(
        "userl=2974317%3C%3Eadb451e5412f18a5c2a7b10c82db5c7acc49f66592c4857120a2693562a5883a%3C%3E1760770828; domain=syosetu.com",
        &url
    );
    Arc::new(jar)
}
