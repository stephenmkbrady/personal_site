---
title: "Portfolio Website"
date: "2024-01-15"
tags: ["rust", "actix-web", "portfolio", "api"]
description: "A modern portfolio website built with Rust backend and vanilla JavaScript frontend"
feature: false
---

# Portfolio Website

This is a comprehensive portfolio website built with modern web technologies. The backend is powered by Rust using the Actix-web framework, providing fast and reliable API endpoints for content management and GitHub integration.

## Features

- **Content Management**: Dynamic markdown-based content system with frontmatter support
- **GitHub Integration**: Automatic fetching and caching of repository information and README files
- **API-First Design**: RESTful API endpoints for all functionality
- **Caching System**: Intelligent caching to reduce API calls and improve performance
- **Modern Frontend**: Vanilla JavaScript with responsive design

## Technical Stack

### Backend
- **Rust**: Systems programming language for performance and safety
- **Actix-web**: High-performance web framework
- **Pulldown-cmark**: Markdown parsing and HTML generation
- **Serde**: Serialization framework for JSON/YAML handling
- **Reqwest**: HTTP client for external API calls

### Frontend
- **Vanilla JavaScript**: No framework dependencies for simplicity
- **Modern CSS**: Grid and Flexbox for responsive layouts
- **Fetch API**: For backend communication

## Architecture

The application follows a clean separation of concerns:

1. **Content Layer**: Markdown files with YAML frontmatter
2. **API Layer**: RESTful endpoints for all operations
3. **Caching Layer**: In-memory caching for performance
4. **Presentation Layer**: Dynamic frontend with API integration

## Getting Started

1. Clone the repository
2. Install Rust dependencies: `cargo build`
3. Start the backend server: `cargo run`
4. Access the frontend at `http://localhost:8080`

The API provides endpoints for content management, GitHub integration, and administrative functions.