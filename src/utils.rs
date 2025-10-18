use anyhow::Result;
use regex::Regex;
use scraper::{Html, Selector};

/// 从 HTML 中提取 CSRF token
pub fn extract_csrf_token(html: &str) -> Result<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("input[name='csrf_onetimepass']")
        .map_err(|e| anyhow::anyhow!("无效的选择器: {:?}", e))?;
    
    let element = document
        .select(&selector)
        .next()
        .ok_or_else(|| anyhow::anyhow!("未找到 CSRF token 元素"))?;
    
    let token = element
        .value()
        .attr("value")
        .ok_or_else(|| anyhow::anyhow!("未找到 CSRF token 的值"))?;
    
    Ok(token.to_string())
}

/// 从 HTML 中提取表单的 action URL
pub fn extract_form_action(html: &str, form_id: &str) -> Result<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(&format!("form#{}", form_id))
        .map_err(|e| anyhow::anyhow!("无效的选择器: {:?}", e))?;
    
    let form = document
        .select(&selector)
        .next()
        .ok_or_else(|| anyhow::anyhow!("未找到表单: {}", form_id))?;
    
    let action = form
        .value()
        .attr("action")
        .ok_or_else(|| anyhow::anyhow!("未找到表单的 action 属性"))?;
    
    Ok(action.to_string())
}

/// 从重定向 URL 中提取 ID
pub fn extract_id_from_url(url: &str, re: &Regex) -> Result<String> {
    let caps = re
        .captures(url)
        .ok_or_else(|| anyhow::anyhow!("无法从 URL 中提取 ID: {}", url))?;
    
    let id = caps
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("正则表达式未捕获到 ID"))?
        .as_str();
    
    Ok(id.to_string())
}
