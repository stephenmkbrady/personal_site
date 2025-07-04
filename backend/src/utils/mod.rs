
use std::fs;
use std::path::Path;
use pulldown_cmark::{Parser, Options, html};
use serde_yaml;
use crate::models::{ContentItem, ContentMetadata};

pub fn get_content_files(category: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content_dir = format!("../content/{}", category);
    let mut files = Vec::new();
    
    if Path::new(&content_dir).exists() {
        for entry in fs::read_dir(&content_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    files.push(filename.to_string());
                }
            }
        }
    }
    
    files.sort();
    Ok(files)
}

pub fn parse_markdown_file(file_path: &str, category: &str) -> Result<ContentItem, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    
    // Split frontmatter and content
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err("Invalid markdown format: missing frontmatter".into());
    }
    
    // Parse frontmatter
    let frontmatter: ContentMetadata = serde_yaml::from_str(parts[1])?;
    
    // Parse markdown content
    let markdown_content = parts[2].trim();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    
    let parser = Parser::new_ext(markdown_content, options);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);
    
    // Extract slug from filename
    let filename = Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    
    Ok(ContentItem {
        slug: filename.to_string(),
        metadata: frontmatter,
        html_content,
        category: category.to_string(),
    })
}