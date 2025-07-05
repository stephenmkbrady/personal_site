use std::fs;
use std::io;
use std::path::Path;
use serde_yaml;
use reqwest;
use pulldown_cmark::{Parser, Options, html};
use base64::Engine;
use crate::models::*;

pub fn get_content_files(category: &str) -> Result<Vec<String>, io::Error> {
    let content_dir = format!("../content/{}", category);
    let mut files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&content_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "md" {
                            if let Some(filename) = path.file_name() {
                                if let Some(filename_str) = filename.to_str() {
                                    files.push(filename_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(files)
}

pub fn parse_markdown_file(file_path: &str, category: &str) -> Result<ContentItem, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    
    // Extract frontmatter and content
    let (frontmatter, markdown_content) = if content.starts_with("---\n") {
        let end = content[4..].find("\n---\n").unwrap_or(0) + 4;
        let frontmatter_str = &content[4..end];
        let content_str = &content[end + 4..];
        (frontmatter_str.trim(), content_str.trim())
    } else {
        ("", content.trim())
    };
    
    // Parse frontmatter
    let metadata: ContentMetadata = if frontmatter.is_empty() {
        ContentMetadata {
            title: "Untitled".to_string(),
            date: "2024-01-01".to_string(),
            tags: vec![],
            description: "".to_string(),
            image: None,
            feature: None,
        }
    } else {
        serde_yaml::from_str(frontmatter)?
    };
    
    // Convert markdown to HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    
    let parser = Parser::new_ext(markdown_content, options);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);
    
    // Get slug from filename
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

pub fn load_github_config() -> Result<GitHubConfig, Box<dyn std::error::Error>> {
    let config_path = "../content/github/config.yaml";
    let config_content = fs::read_to_string(config_path)?;
    let config: GitHubConfig = serde_yaml::from_str(&config_content)?;
    Ok(config)
}

pub async fn fetch_github_project(repo: &GitHubRepo) -> Result<GitHubProject, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    // Fetch repository info
    let repo_url = format!("https://api.github.com/repos/{}/{}", repo.owner, repo.repo);
    let repo_response = client
        .get(&repo_url)
        .header("User-Agent", "portfolio-backend")
        .send()
        .await?;
    
    if !repo_response.status().is_success() {
        return Err(format!("GitHub API error: {}", repo_response.status()).into());
    }
    
    let repo_data: serde_json::Value = repo_response.json().await?;
    
    // Fetch README
    let readme_url = format!("https://api.github.com/repos/{}/{}/readme", repo.owner, repo.repo);
    let readme_response = client
        .get(&readme_url)
        .header("User-Agent", "portfolio-backend")
        .send()
        .await?;
    
    let readme_html = if readme_response.status().is_success() {
        let readme_data: serde_json::Value = readme_response.json().await?;
        if let Some(content) = readme_data["content"].as_str() {
            // Decode base64 content
            let decoded = base64::engine::general_purpose::STANDARD.decode(content.replace('\n', ""))?;
            let markdown_content = String::from_utf8(decoded)?;
            
            // Convert markdown to HTML
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            options.insert(Options::ENABLE_TABLES);
            options.insert(Options::ENABLE_FOOTNOTES);
            options.insert(Options::ENABLE_TASKLISTS);
            
            let parser = Parser::new_ext(&markdown_content, options);
            let mut html_content = String::new();
            html::push_html(&mut html_content, parser);
            html_content
        } else {
            "README not available".to_string()
        }
    } else {
        "README not available".to_string()
    };
    
    Ok(GitHubProject {
        owner: repo.owner.clone(),
        repo: repo.repo.clone(),
        display_name: repo.display_name.clone(),
        readme_html,
        url: repo_data["html_url"].as_str().unwrap_or("").to_string(),
        stars: repo_data["stargazers_count"].as_u64().unwrap_or(0) as u32,
        forks: repo_data["forks_count"].as_u64().unwrap_or(0) as u32,
        description: repo_data["description"].as_str().map(|s| s.to_string()),
        feature: repo.feature,
        image: repo.image.clone(),
    })
}