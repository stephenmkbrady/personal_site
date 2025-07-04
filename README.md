# Portfolio Website Design Document

## Project Overview

A personal portfolio website with Actix-web backend and vanilla HTML/CSS/JS frontend. Features include GitHub project showcase, AI chatbot, and markdown content management.

## File Structure

```
PORTFOLIO_WEBSITE/
├── backend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── handlers/
│   │   ├── models/
│   │   └── utils/
│   └── Cargo.toml
├── frontend/
│   ├── index.html
│   ├── admin.html
│   ├── static/
│   │   ├── css/
│   │   ├── js/
│   │   └── images/
└── content/
    ├── github/
    │   └── config.yaml
    ├── project/
    │   ├── project1.md
    │   └── project2.md
    ├── blog/
    │   ├── title1.md
    │   └── title2.md
    ├── about.md
    └── chatbot.md
```

## GitHub Integration Section

### Configuration
- **Config file**: `content/github/config.yaml`
- **Format**:
  ```yaml
  repositories:
    - owner: "yourusername"
      repo: "project1"
      display_name: "My Cool Project"
    - owner: "yourusername" 
      repo: "project2"
      display_name: "Another Project"
  ```

### Caching Strategy
- **Cache location**: In-memory HashMap or SQLite file
- **Cache duration**: 24 hours
- **Force refresh**: Admin panel button → `/api/admin/refresh-github`
- **Cache key**: `{owner}/{repo}`

### Implementation
- **GitHub API**: Use reqwest to fetch `https://api.github.com/repos/{owner}/{repo}/readme`
- **Rate limiting**: GitHub allows 60 requests/hour without auth, 5000 with token
- **Error handling**: If README fetch fails, show cached version or placeholder

### API Endpoints
- `GET /api/github/projects` - Returns list of projects with cached README content
- `POST /api/admin/refresh-github` - Force refresh all GitHub caches (admin only)

## Chatbot Section

### System Prompt Configuration
- **File**: `content/chatbot.md` contains system prompt
- **Purpose**: Restrict chatbot to website-related questions only
- **Example prompt**: "You are a helpful assistant for a portfolio website. Only answer questions about the projects, blog posts, and content on this site. Politely decline to discuss other topics."

### Chat Interface
- **Location**: Fixed popup in bottom-right corner
- **Behavior**: 
  - Small collapsed icon initially
  - Expands to chat window when clicked
  - Single conversation thread
  - Chat history maintained during session only
  - Cleared on page refresh/close

### OpenRouter Integration
- **Model**: Best available free model (likely Claude or Llama)
- **Context**: Include relevant site content in API calls
- **Session management**: Store chat history in browser localStorage temporarily

### API Endpoints
- `POST /api/chat` - Send message to chatbot
- `GET /api/chat/system-prompt` - Get system prompt from chatbot.md

## Markdown to HTML Section

### File Organization
- **Projects**: `content/project/*.md`
- **Blog posts**: `content/blog/*.md`  
- **Pages**: `content/*.md` (about, contact, etc.)

### Frontmatter Format
```yaml
---
title: "Project Name"
date: "2024-01-15"
tags: ["rust", "web", "portfolio"]
description: "Short description for previews"
---
```

### Processing Pipeline
1. **Parse frontmatter** using a YAML parser
2. **Convert markdown to HTML** using `pulldown-cmark` crate
3. **Apply syntax highlighting** for code blocks
4. **Generate table of contents** from headers
5. **Sort content** by date/tags for listings

### Routing Strategy
- `/project/project-name` → `content/project/project-name.md`
- `/blog/post-title` → `content/blog/post-title.md`
- `/about` → `content/about.md`

### API Endpoints
- `GET /api/content/{category}` - List all files in category (project, blog)
- `GET /api/content/{category}/{slug}` - Get specific markdown file as HTML
- `GET /api/content/tags` - Get all available tags for filtering

## Admin Authentication Section

### Login System
- **Route**: `/knockknock` (hidden admin login)
- **Method**: Simple username/password form
- **Session**: JWT token or session cookie
- **Storage**: Hardcode credentials in environment variables for simplicity

### Admin Panel Features
- **Route**: `/admin` (only accessible when authenticated)
- **Features**:
  - Button to refresh GitHub cache
  - View cache status and last update times
  - Basic content management (list markdown files)
  - System status (API health checks)

### Security
- **Rate limiting**: Max 5 login attempts per hour
- **Session timeout**: 2 hours of inactivity
- **HTTPS only**: In production

## API Endpoints Summary

### Public Endpoints
- `GET /api/github/projects` - GitHub projects with README
- `GET /api/content/{category}` - Content listings
- `GET /api/content/{category}/{slug}` - Specific content
- `GET /api/content/tags` - Available tags
- `POST /api/chat` - Chatbot messages
- `GET /api/chat/system-prompt` - Chatbot configuration

### Admin Endpoints (require authentication)
- `POST /api/auth/login` - Admin login
- `POST /api/admin/refresh-github` - Force GitHub cache refresh
- `GET /api/admin/status` - System status

## Frontend Structure

### Main Pages
- **index.html**: Home page with project showcases
- **project.html**: Individual project pages
- **blog.html**: Blog listing and individual posts
- **admin.html**: Admin panel (hidden)

### JavaScript Components
- **chatbot.js**: Chat popup functionality
- **content-loader.js**: Dynamic content loading
- **admin.js**: Admin panel interactions
- **github-display.js**: GitHub project rendering

### CSS Organization
- **main.css**: Base styles and layout
- **components.css**: Reusable UI components
- **admin.css**: Admin panel specific styles

## Development Order

1. **Basic Actix-web server** with static file serving
2. **Markdown processing** and content API endpoints
3. **GitHub integration** with caching
4. **Simple admin authentication** and panel
5. **OpenRouter chatbot** integration
6. **Frontend JavaScript** for dynamic loading
7. **Polish UI/UX** and responsive design
8. **Rate limiting** and production hardening

## Dependencies

### Backend (Rust)
```toml
[dependencies]
actix-web = "4"
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
pulldown-cmark = "0.9"
tokio = { version = "1", features = ["full"] }
jsonwebtoken = "8"
```

### Frontend
- Vanilla JavaScript (no framework dependencies)
- Modern CSS with CSS Grid/Flexbox
- Fetch API for backend communication