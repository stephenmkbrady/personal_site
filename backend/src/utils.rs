use std::fs;
use std::io;
use std::path::Path;
use serde_yaml;
use reqwest;
use pulldown_cmark::{Parser, Options, html, Event, Tag, CodeBlockKind};
use regex::Regex;
use base64::Engine;
use crate::models::*;

/// Input validation error types
#[derive(Debug)]
pub enum ValidationError {
    InvalidCategory(String),
    InvalidSlug(String),
    PathTraversal(String),
    TooLong(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidCategory(msg) => write!(f, "Invalid category: {}", msg),
            ValidationError::InvalidSlug(msg) => write!(f, "Invalid slug: {}", msg),
            ValidationError::PathTraversal(msg) => write!(f, "Path traversal detected: {}", msg),
            ValidationError::TooLong(msg) => write!(f, "Input too long: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validates category parameter
/// Categories must be alphanumeric with hyphens/underscores only
pub fn validate_category(category: &str) -> Result<(), ValidationError> {
    // Check length
    if category.is_empty() {
        return Err(ValidationError::InvalidCategory("Category cannot be empty".to_string()));
    }
    if category.len() > 50 {
        return Err(ValidationError::TooLong("Category name too long".to_string()));
    }
    
    // Check for path traversal
    if category.contains("..") || category.contains('/') || category.contains('\\') {
        return Err(ValidationError::PathTraversal("Category contains path traversal characters".to_string()));
    }
    
    // Only allow alphanumeric, hyphens, and underscores
    let valid_chars = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    if !valid_chars.is_match(category) {
        return Err(ValidationError::InvalidCategory("Category must contain only letters, numbers, hyphens, and underscores".to_string()));
    }
    
    // Check against whitelist of allowed categories
    let allowed_categories = ["project", "blog", "page"];
    if !allowed_categories.contains(&category) {
        return Err(ValidationError::InvalidCategory(format!("Category '{}' not allowed. Allowed: {:?}", category, allowed_categories)));
    }
    
    Ok(())
}

/// Validates slug parameter
/// Slugs must be URL-safe and prevent path traversal
pub fn validate_slug(slug: &str) -> Result<(), ValidationError> {
    // Check length
    if slug.is_empty() {
        return Err(ValidationError::InvalidSlug("Slug cannot be empty".to_string()));
    }
    if slug.len() > 100 {
        return Err(ValidationError::TooLong("Slug too long".to_string()));
    }
    
    // Check for path traversal
    if slug.contains("..") || slug.contains('/') || slug.contains('\\') {
        return Err(ValidationError::PathTraversal("Slug contains path traversal characters".to_string()));
    }
    
    // Check for null bytes and other dangerous characters
    if slug.contains('\0') || slug.contains('\n') || slug.contains('\r') {
        return Err(ValidationError::InvalidSlug("Slug contains invalid characters".to_string()));
    }
    
    // Only allow URL-safe characters: alphanumeric, hyphens, underscores, and dots
    let valid_chars = regex::Regex::new(r"^[a-zA-Z0-9._-]+$").unwrap();
    if !valid_chars.is_match(slug) {
        return Err(ValidationError::InvalidSlug("Slug must contain only letters, numbers, dots, hyphens, and underscores".to_string()));
    }
    
    // Prevent starting or ending with dots (hidden files)
    if slug.starts_with('.') || slug.ends_with('.') {
        return Err(ValidationError::InvalidSlug("Slug cannot start or end with dots".to_string()));
    }
    
    Ok(())
}

/// Creates a safe file path by validating and joining components
pub fn create_safe_content_path(content_base: &str, category: &str, filename: Option<&str>) -> Result<String, ValidationError> {
    validate_category(category)?;
    
    let mut path = format!("{}/{}", content_base, category);
    
    if let Some(file) = filename {
        validate_slug(file)?;
        path = format!("{}/{}.md", path, file);
    }
    
    // Final safety check: ensure the resolved path stays within content directory
    let canonical_base = std::path::Path::new(content_base).canonicalize()
        .map_err(|_| ValidationError::PathTraversal("Invalid content base path".to_string()))?;
    
    let canonical_target = std::path::Path::new(&path).canonicalize();
    if let Ok(target) = canonical_target {
        if !target.starts_with(&canonical_base) {
            return Err(ValidationError::PathTraversal("Path escapes content directory".to_string()));
        }
    }
    
    Ok(path)
}

pub fn get_content_files(category: &str, content_path: &str) -> Result<Vec<String>, io::Error> {
    let content_dir = format!("{}/{}", content_path, category);
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

pub fn load_github_config(content_path: &str) -> Result<GitHubConfig, Box<dyn std::error::Error>> {
    let config_path = format!("{}/github/config.yaml", content_path);
    let config_content = fs::read_to_string(&config_path)?;
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
            
            // Process markdown to fix image URLs
            let processed_markdown = process_github_images(&markdown_content, &repo.owner, &repo.repo);
            
            // Convert markdown to HTML
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            options.insert(Options::ENABLE_TABLES);
            options.insert(Options::ENABLE_FOOTNOTES);
            options.insert(Options::ENABLE_TASKLISTS);
            
            let parser = Parser::new_ext(&processed_markdown, options);
            let mut html_content = String::new();
            
            // Process events to add language classes for syntax highlighting
            let events: Vec<Event> = parser.into_iter().map(|event| {
                match event {
                    Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                        let lang_class = if lang.is_empty() {
                            "language-text".to_string()
                        } else {
                            format!("language-{}", lang)
                        };
                        Event::Html(format!("<pre><code class=\"{}\">", lang_class).into())
                    }
                    Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
                        Event::Html("</code></pre>".into())
                    }
                    _ => event,
                }
            }).collect();
            
            html::push_html(&mut html_content, events.into_iter());
            
            // Further process HTML to ensure all GitHub images work
            process_github_html_images(&html_content, &repo.owner, &repo.repo)
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

fn process_github_images(markdown: &str, owner: &str, repo: &str) -> String {
    use regex::Regex;
    
    // Pattern to match markdown images: ![alt](src)
    let img_regex = Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap();
    
    img_regex.replace_all(markdown, |caps: &regex::Captures| {
        let alt = &caps[1];
        let src = &caps[2];
        
        let new_src = convert_github_image_url(src, owner, repo);
        format!("![{}]({})", alt, new_src)
    }).to_string()
}

fn process_github_html_images(html: &str, owner: &str, repo: &str) -> String {
    use regex::Regex;
    
    // Pattern to match HTML img tags: <img src="..." /> or <img src="..." >
    let img_regex = Regex::new(r#"<img([^>]*?)src="([^"]*)"([^>]*?)/?>?"#).unwrap();
    
    img_regex.replace_all(html, |caps: &regex::Captures| {
        let before_src = &caps[1];
        let src = &caps[2];
        let after_src = &caps[3];
        
        let new_src = convert_github_image_url(src, owner, repo);
        format!(r#"<img{}src="{}"{}/>"#, before_src, new_src, after_src)
    }).to_string()
}

fn convert_github_image_url(src: &str, owner: &str, repo: &str) -> String {
    // If it's already a full URL, return as is
    if src.starts_with("http://") || src.starts_with("https://") {
        return src.to_string();
    }
    
    // If it's a relative path, convert to GitHub raw URL
    if src.starts_with("./") || src.starts_with("../") || !src.starts_with("/") {
        // Remove leading "./" if present
        let clean_src = src.strip_prefix("./").unwrap_or(src);
        
        // Handle "../" paths by going up directories (simplified - assume they want root)
        let clean_src = if clean_src.starts_with("../") {
            clean_src.strip_prefix("../").unwrap_or(clean_src)
        } else {
            clean_src
        };
        
        return format!("https://raw.githubusercontent.com/{}/{}/main/{}", owner, repo, clean_src);
    }
    
    // If it starts with "/", it's an absolute path in the repo
    if src.starts_with("/") {
        return format!("https://raw.githubusercontent.com/{}/{}/main{}", owner, repo, src);
    }
    
    // Default case - treat as relative to repo root
    format!("https://raw.githubusercontent.com/{}/{}/main/{}", owner, repo, src)
}