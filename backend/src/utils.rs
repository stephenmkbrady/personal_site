use pulldown_cmark::{Parser, Options, html};
use serde_yaml;
use std::fs;
use std::path::Path;
use crate::models::{ContentItem, ContentMetadata, GitHubConfig, GitHubProject, GitHubRepo};
use anyhow::Result;

pub fn parse_markdown_file(file_path: &str, category: &str) -> Result<ContentItem> {
    let content = fs::read_to_string(file_path)?;
    let (metadata, markdown_content) = parse_frontmatter(&content)?;
    
    // Convert markdown to HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    
    let parser = Parser::new_ext(&markdown_content, options);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);
    
    let slug = Path::new(file_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    
    Ok(ContentItem {
        slug,
        metadata,
        html_content,
        category: category.to_string(),
    })
}

fn parse_frontmatter(content: &str) -> Result<(ContentMetadata, String)> {
    if !content.starts_with("---") {
        return Err(anyhow::anyhow!("No frontmatter found"));
    }
    
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(anyhow::anyhow!("Invalid frontmatter format"));
    }
    
    let frontmatter_str = parts[1].trim();
    let markdown_content = parts[2].trim();
    
    let metadata: ContentMetadata = serde_yaml::from_str(frontmatter_str)?;
    
    Ok((metadata, markdown_content.to_string()))
}

pub fn load_github_config() -> Result<GitHubConfig> {
    let config_path = "../content/github/config.yaml";
    let config_content = fs::read_to_string(config_path)?;
    let config: GitHubConfig = serde_yaml::from_str(&config_content)?;
    Ok(config)
}

pub async fn fetch_github_readme(owner: &str, repo: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/readme", owner, repo);
    
    let response = client
        .get(&url)
        .header("User-Agent", "portfolio-website")
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to fetch README: {}", response.status()));
    }
    
    let readme_data: serde_json::Value = response.json().await?;
    let content = readme_data["content"].as_str()
        .ok_or_else(|| anyhow::anyhow!("No content in README response"))?;
    
    // Decode base64 content
    let decoded = base64::decode(content.replace('\n', ""))?;
    Ok(String::from_utf8(decoded)?)
}

pub async fn fetch_github_repo_info(owner: &str, repo: &str) -> Result<(u32, u32, Option<String>)> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}", owner, repo);
    
    let response = client
        .get(&url)
        .header("User-Agent", "portfolio-website")
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to fetch repo info: {}", response.status()));
    }
    
    let repo_data: serde_json::Value = response.json().await?;
    let stars = repo_data["stargazers_count"].as_u64().unwrap_or(0) as u32;
    let forks = repo_data["forks_count"].as_u64().unwrap_or(0) as u32;
    let description = repo_data["description"].as_str().map(|s| s.to_string());
    
    Ok((stars, forks, description))
}

pub fn get_content_files(category: &str) -> Result<Vec<String>> {
    let content_dir = format!("../content/{}", category);
    let mut files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&content_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "md") {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        files.push(file_name.to_string());
                    }
                }
            }
        }
    }
    
    files.sort();
    Ok(files)
}