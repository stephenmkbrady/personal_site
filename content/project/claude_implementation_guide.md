---
title: "Claude Implementation Guide"
date: "2024-01-15"
tags: ["rust", "actix-web", "portfolio", "api"]
description: "Claude Implementation guide"
feature: false
---
# CLAUDE.md - Rust Portfolio Implementation Guide

This guide provides complete implementation details for Claude Code to build the self-hosted Rust/WASM portfolio with plugin system, security features, and Docker deployment.

## 📚 Documentation System Overview

This implementation uses an integrated documentation system for autonomous development:

- **CLAUDE.md** (this file) - Complete technical specifications and architecture details
- **PROMPTS.md** - 14-stage incremental implementation guide with context window management
- **TESTING.md** - Comprehensive testing strategies, automation, and quality assurance
- **DEBUG.md** - Autonomous debugging protocols and systematic error recovery
- **SUCCESS.md** - Intelligence-driven development patterns and adaptive strategies

**→ Start implementation using PROMPTS.md for stage-by-stage guidance**

## 🎯 Project Overview

**Tech Stack**: Rust + WASM (frontend) + Node.js (backend) + nginx (reverse proxy) + Docker
**Security**: Geographic blocking (Europe/Americas), bot detection, rate limiting
**Features**: Hotpluggable games/projects, AI chatbot, multi-language support, accessibility
**Deployment**: Self-hosted with Docker Compose

## 📁 Complete Project Structure

```
rust-portfolio/
├── CLAUDE.md                          # This implementation guide
├── README.md                          # User documentation
├── Cargo.toml                         # Rust dependencies
├── Cargo.lock                         # Locked dependencies
├── trunk.toml                         # Trunk build configuration
├── index.html                         # Main HTML template
├── Dockerfile                         # Production Docker image
├── Dockerfile.dev                     # Development Docker image
├── docker-compose.yml                 # Development environment
├── docker-compose.prod.yml            # Production environment
├── .env.example                       # Environment template
├── .gitignore                         # Git ignore patterns
├── .github/workflows/
│   └── deploy.yml                     # CI/CD pipeline
├── src/                               # Rust frontend source
│   ├── lib.rs                         # Main library entry
│   ├── app.rs                         # Main app component
│   ├── components/                    # UI components
│   │   ├── mod.rs
│   │   ├── node_graph.rs              # Interactive node graph
│   │   ├── theme_switcher.rs          # Theme system
│   │   ├── chatbot.rs                 # AI chatbot interface
│   │   ├── game_embed.rs              # WASM game embedding
│   │   ├── project_detail.rs          # Project detail view
│   │   ├── navigation.rs              # Navigation components
│   │   └── accessibility.rs           # A11y components
│   ├── systems/                       # Core systems
│   │   ├── mod.rs
│   │   ├── plugins.rs                 # Plugin management system
│   │   ├── physics.rs                 # Physics simulation
│   │   ├── rendering.rs               # WGPU rendering
│   │   ├── input.rs                   # Input handling
│   │   ├── themes.rs                  # Theme management
│   │   ├── animations.rs              # Animation engine
│   │   ├── i18n.rs                    # Internationalization
│   │   ├── accessibility.rs           # Accessibility system
│   │   └── nodes.rs                   # Node graph logic
│   ├── content/                       # Content integration
│   │   ├── mod.rs
│   │   ├── api_client.rs              # Backend API client
│   │   ├── projects.rs                # Project data structures
│   │   └── games.rs                   # Game data structures
│   └── utils/                         # Utility functions
│       ├── mod.rs
│       ├── math.rs                    # Math utilities
│       ├── colors.rs                  # Color manipulation
│       └── animations.rs              # Animation helpers
├── backend/                           # Node.js API server
│   ├── package.json                   # Node.js dependencies
│   ├── package-lock.json              # Locked dependencies
│   ├── server.js                      # Main server file
│   ├── Dockerfile.dev                 # Development backend
│   ├── middleware/                    # Security middleware
│   │   ├── security.js                # Input validation & filtering
│   │   ├── bot-detection.js           # Bot detection system
│   │   ├── geo-blocker.js             # Geographic restrictions
│   │   └── plugin-watcher.js          # File watcher for plugins
│   ├── routes/                        # API routes
│   │   ├── chat.js                    # Chatbot endpoints
│   │   ├── github.js                  # GitHub API proxy
│   │   ├── contact.js                 # Contact form handler
│   │   └── analytics.js               # Usage analytics
│   ├── locales/                       # Translation files
│   │   ├── en.json                    # English translations
│   │   ├── es.json                    # Spanish translations
│   │   ├── fr.json                    # French translations
│   │   └── de.json                    # German translations
│   └── tests/                         # Backend tests
│       ├── security.test.js           # Security middleware tests
│       ├── api.test.js                # API endpoint tests
│       └── plugins.test.js            # Plugin system tests
├── nginx/                             # nginx configuration
│   ├── nginx.conf                     # Production nginx config
│   ├── nginx.dev.conf                 # Development nginx config
│   └── ssl/                           # SSL certificates
│       ├── cert.pem
│       └── key.pem
├── plugins/                           # Plugin system content
│   ├── plugins.yaml                   # Main plugin configuration
│   ├── projects/                      # Project plugins
│   │   ├── rust-game-engine/
│   │   │   ├── plugin.yaml            # Project config
│   │   │   ├── README.md              # Auto-parsed description
│   │   │   ├── screenshots/           # Auto-detected images
│   │   │   │   ├── demo1.png
│   │   │   │   └── demo2.gif
│   │   │   └── metadata.json          # Additional metadata
│   │   └── portfolio-website/
│   │       ├── plugin.yaml
│   │       ├── README.md
│   │       └── screenshots/
│   └── games/                         # Game plugins
│       ├── snake-game/
│       │   ├── plugin.yaml            # Game config
│       │   ├── snake.wasm             # Compiled WASM
│       │   ├── snake.js               # JS bindings
│       │   ├── controls.md            # Control instructions
│       │   ├── thumbnail.png          # Game thumbnail
│       │   └── metadata.json          # Scores, difficulty
│       └── tetris-clone/
│           ├── plugin.yaml
│           ├── tetris.wasm
│           ├── tetris.js
│           └── thumbnail.png
├── themes/                            # Theme system content
│   ├── themes.yaml                    # Main theme configuration
│   ├── ice/
│   │   ├── theme.yaml                 # Theme metadata and config
│   │   ├── styles.css                 # Theme-specific CSS
│   │   ├── particles.json             # Particle system config
│   │   ├── colors.json                # Color palette definition
│   │   ├── preview.png                # Theme preview image
│   │   └── assets/                    # Theme-specific assets
│   │       ├── background.jpg
│   │       └── textures/
│   ├── mycelium/
│   │   ├── theme.yaml
│   │   ├── styles.css
│   │   ├── particles.json
│   │   ├── colors.json
│   │   ├── preview.png
│   │   └── assets/
│   └── botanical/
│       ├── theme.yaml
│       ├── styles.css
│       ├── particles.json
│       ├── colors.json
│       ├── preview.png
│       └── assets/
│   ├── images/                        # General images
│   ├── icons/                         # Icon files
│   └── data/                          # Static data
│       ├── knowledge-base.md          # Chatbot knowledge
│       └── analytics.json             # Analytics configuration
├── styles/                            # CSS stylesheets
│   ├── main.css                       # Main styles
│   ├── chatbot.css                    # Chatbot specific styles
│   ├── accessibility.css             # A11y styles
│   ├── i18n.css                       # Internationalization styles
│   └── themes/                        # Theme-specific styles
│       ├── ice.css
│       ├── mycelium.css
│       └── botanical.css
├── scripts/                           # Build and deployment
│   ├── build.sh                       # Build script
│   ├── deploy.sh                      # Deployment script
│   ├── server-setup.sh                # Server setup script
│   ├── monitor.sh                     # Health monitoring
│   ├── security-check.sh              # Security audit script
│   └── backup.sh                      # Backup script
├── tests/                             # Frontend tests
│   ├── integration/                   # Integration tests
│   └── unit/                          # Unit tests
└── docs/                              # Documentation
    ├── API.md                         # API documentation
    ├── SECURITY.md                    # Security guide
    ├── DEPLOYMENT.md                  # Deployment guide
    └── PLUGINS.md                     # Plugin development guide
```

## 🔧 Implementation Order

### Phase 1: Basic Infrastructure (Days 1-3)
1. **Set up Rust project structure**
2. **Create basic Docker development environment**
3. **Implement minimal WGPU rendering**
4. **Set up trunk build system**
5. **Create basic nginx reverse proxy**

### Phase 2: Backend API (Days 4-6)
1. **Create Express.js server with security middleware**
2. **Implement rate limiting and bot detection**
3. **Set up geographic blocking with GeoIP**
4. **Create health check and basic API endpoints**
5. **Add WebSocket support for plugin updates**

### Phase 3: Plugin System Core (Days 7-9)
1. **Implement plugin manager in Rust**
2. **Create YAML configuration parsing**
3. **Build file watcher system in backend**
4. **Add hot-reloading capability**
5. **Create example plugins for testing**

### Phase 4: Node Graph System (Days 10-13)
1. **Implement force-directed layout algorithm**
2. **Create node rendering with WGPU**
3. **Add input handling (zoom, pan, click)**
4. **Implement physics simulation**
5. **Connect to plugin system for dynamic nodes**

### Phase 5: Game Integration (Days 14-16)
1. **Create WASM game embedding system**
2. **Implement game plugin loading**
3. **Add game controls and UI**
4. **Create game metadata display**
5. **Add high score persistence**

### Phase 6: AI Chatbot (Days 17-19)
1. **Integrate OpenRouter API securely**
2. **Create chatbot UI component**
3. **Implement knowledge base system**
4. **Add multi-language chatbot support**
5. **Create conversation management**

### Phase 7: Advanced Features (Days 20-25)
1. **Implement theme system with particle effects**
2. **Add accessibility features**
3. **Create internationalization system**
4. **Build animation engine**
5. **Add analytics and monitoring**

### Phase 8: Production Deployment (Days 26-28)
1. **Create production Docker configuration**
2. **Set up SSL and security hardening**
3. **Implement CI/CD pipeline**
4. **Add monitoring and alerting**
5. **Perform load testing and optimization**

## 🎨 Theme System Implementation

### Theme Discovery System
The theme system works exactly like the plugin system - completely plug-and-play:

#### Adding a New Theme (Zero Code Changes Required)
```bash
# 1. Create theme folder
mkdir themes/my-custom-theme

# 2. Add theme configuration
cat > themes/my-custom-theme/theme.yaml << EOF
name: "My Custom Theme"
description: "A custom theme I created"
version: "1.0.0"
category: "custom"

colors:
  primary: "#FF6B35"
  secondary: "#F7931E"
  accent: "#FFE66D"
  background: "#006A6B"
  text: "#FFFFFF"

particles:
  type: "sparkle"
  count: 30
  speed: 1.2
  size_range: [3, 10]

effects:
  node_glow: true
  connection_animation: "wave"
EOF

# 3. Add theme styles
cat > themes/my-custom-theme/styles.css << EOF
.theme-my-custom-theme {
  --primary-color: #FF6B35;
  --secondary-color: #F7931E;
  --accent-color: #FFE66D;
  --background-color: #006A6B;
  --text-color: #FFFFFF;
}

.theme-my-custom-theme .node {
  box-shadow: 0 0 20px var(--accent-color);
}
EOF

# 4. Add to main registry (optional - auto-discovery can handle this)
# The theme will appear automatically in the theme switcher!
```

#### Theme Implementation Code Structure
```rust
// src/systems/themes.rs
pub struct ThemeManager {
    available_themes: RefCell<Vec<Theme>>,
    current_theme: RefCell<Option<Theme>>,
    config: RefCell<Option<ThemeConfig>>,
    connection_renderer: RefCell<Option<ConnectionRenderer>>,
    shader_manager: RefCell<Option<ShaderManager>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub category: String,
    pub colors: ThemeColors,
    pub connections: ConnectionConfig,
    pub shaders: ShaderConfig,
    pub particles: ParticleConfig,
    pub effects: ThemeEffects,
    pub assets: ThemeAssets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub pattern: ConnectionPattern,
    pub enabled: bool,
    pub style: ConnectionStyle,
    pub pattern_settings: PatternSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionPattern {
    Tendril,
    Tree,
    Mycelium,
    Snowflake,
    Lightning,
    Simple,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderConfig {
    pub enabled: bool,
    pub node_shader: NodeShader,
    pub connection_shader: ConnectionShader,
    pub background_shader: BackgroundShader,
    pub settings: ShaderSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeShader {
    Glow,
    Ice,
    Crystal,
    Metal,
    Energy,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionShader {
    Ice,
    Lightning,
    Energy,
    Fiber,
    None,
}

impl ThemeManager {
    pub async fn discover_themes(&self) -> Result<Vec<Theme>, String> {
        // Automatically scan themes/ directory
        // Parse theme.yaml files
        // Validate theme configurations including new fields
        // Load connection patterns and shader configurations
        // Return list of available themes
    }
    
    pub async fn switch_theme(&self, theme_id: &str) -> Result<(), String> {
        // Load theme configuration
        // Initialize connection pattern renderer
        // Compile and load shaders
        // Apply CSS styles dynamically
        // Update particle system
        // Trigger smooth transition
    }
    
    pub async fn hot_reload_themes(&self) -> Result<(), String> {
        // Watch for file system changes
        // Reload changed themes
        // Recompile shaders if needed
        // Update connection patterns
        // Update UI automatically
    }
    
    pub fn render_connections(&self, nodes: &[Node], context: &RenderContext) -> Result<(), String> {
        let theme = self.current_theme.borrow();
        if let Some(theme) = theme.as_ref() {
            if !theme.connections.enabled {
                return Ok(());
            }
            
            let connection_renderer = self.connection_renderer.borrow();
            if let Some(renderer) = connection_renderer.as_ref() {
                renderer.render_pattern(
                    &theme.connections.pattern,
                    nodes,
                    &theme.connections.style,
                    &theme.connections.pattern_settings,
                    context
                )?;
            }
        }
        Ok(())
    }
    
    pub fn apply_node_shader(&self, node: &Node, context: &RenderContext) -> Result<(), String> {
        let theme = self.current_theme.borrow();
        if let Some(theme) = theme.as_ref() {
            if !theme.shaders.enabled {
                return Ok(());
            }
            
            let shader_manager = self.shader_manager.borrow();
            if let Some(manager) = shader_manager.as_ref() {
                manager.apply_node_shader(
                    &theme.shaders.node_shader,
                    node,
                    &theme.shaders.settings,
                    context
                )?;
            }
        }
        Ok(())
    }
}

// src/systems/connections.rs
pub struct ConnectionRenderer {
    pattern_generators: HashMap<ConnectionPattern, Box<dyn PatternGenerator>>,
    webgl_context: Option<WebGlRenderingContext>,
}

pub trait PatternGenerator {
    fn generate_connections(&self, nodes: &[Node], settings: &PatternSettings) -> Vec<Connection>;
    fn animate_connections(&self, connections: &mut [Connection], delta_time: f32);
}

pub struct TendrilGenerator;
impl PatternGenerator for TendrilGenerator {
    fn generate_connections(&self, nodes: &[Node], settings: &PatternSettings) -> Vec<Connection> {
        // Generate organic, flowing curves between nodes
        // Use Bezier curves with organic variation
        // Apply curve strength and variation from settings
    }
}

pub struct SnowflakeGenerator;
impl PatternGenerator for SnowflakeGenerator {
    fn generate_connections(&self, nodes: &[Node], settings: &PatternSettings) -> Vec<Connection> {
        // Generate 6-fold symmetric crystalline structure
        // Place nodes at geometric angles
        // Create radial pattern from center
    }
}

// src/systems/shaders.rs
pub struct ShaderManager {
    compiled_shaders: HashMap<String, WebGlProgram>,
    webgl_context: WebGlRenderingContext,
    uniform_locations: HashMap<String, WebGlUniformLocation>,
}

impl ShaderManager {
    pub fn compile_shader(&mut self, shader_type: &str, source: &str) -> Result<WebGlShader, String> {
        // Compile GLSL shader source
        // Handle compilation errors gracefully
        // Return compiled shader or fallback
    }
    
    pub fn apply_ice_shader(&self, geometry: &Geometry, settings: &IceShaderSettings) -> Result<(), String> {
        // Apply ice refraction shader
        // Set uniforms: frost_intensity, refraction_strength, ice_tint
        // Handle transparency and distortion effects
    }
    
    pub fn apply_lightning_shader(&self, connections: &[Connection], settings: &LightningShaderSettings) -> Result<(), String> {
        // Apply electrical bloom effect
        // Animate flicker and electrical noise
        // Handle branching glow effects
    }
}
```

#### Automatic Theme Discovery Process
1. **Scan Directory**: Automatically scan `themes/` folder for subdirectories
2. **Parse Config**: Read `theme.yaml` from each theme folder  
3. **Validate**: Ensure required fields are present and valid
4. **Load Assets**: Preload CSS, images, and particle configurations
5. **Register**: Add to available themes list in theme switcher
6. **Hot Reload**: Watch for changes and update automatically

#### Theme Hot-Reloading
Just like plugins, themes support hot-reloading:
- Edit `theme.yaml` → Theme updates automatically
- Add new `themes/new-theme/` folder → Appears in switcher immediately  
- Modify `styles.css` → Visual changes apply instantly
- Update `particles.json` → Particle effects change in real-time

### Rust Dependencies (Cargo.toml)
```toml
[package]
name = "rust-portfolio"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
leptos = { version = "0.6", features = ["csr", "nightly"] }
leptos_meta = { version = "0.6", features = ["csr"] }
leptos_router = { version = "0.6", features = ["csr"] }
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = [
  "console",
  "Window",
  "Document",
  "Element",
  "HtmlElement",
  "HtmlCanvasElement",
  "CanvasRenderingContext2d",
  "WebGlRenderingContext",
  "WebGl2RenderingContext",
  "RequestAnimationFrame",
  "Performance",
  "Location",
  "History",
  "Storage",
  "LocalStorage",
  "SessionStorage",
  "Event",
  "EventTarget",
  "MouseEvent",
  "KeyboardEvent",
  "TouchEvent",
  "WheelEvent",
  "ResizeObserver",
  "IntersectionObserver",
  "MutationObserver",
  "WebSocket",
  "MessageEvent",
  "CloseEvent",
  "ErrorEvent"
] }
wgpu = "0.19"
cgmath = "0.18"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
gloo-net = { version = "0.5", features = ["http"] }
gloo-timers = { version = "0.3", features = ["futures"] }
gloo-file = "0.3"
gloo-storage = "0.3"
gloo-events = "0.2"
gloo-utils = "0.2"
wasm-bindgen-futures = "0.4"
fluent = "0.16"
fluent-bundle = "0.15"
unic-langid = "0.9"
getrandom = { version = "0.2", features = ["js"] }
rand = "0.8"
uuid = { version = "1.0", features = ["v4", "wasm-bindgen"] }
console_error_panic_hook = "0.1"
tracing = "0.1"
tracing-wasm = "0.2"

[dependencies.wgpu]
version = "0.19"
features = ["webgpu", "webgl"]

[profile.release]
lto = true
opt-level = "s"
codegen-units = 1
panic = "abort"
```

### Backend Dependencies (package.json)
```json
{
  "name": "portfolio-backend",
  "version": "1.0.0",
  "description": "Secure backend API for Rust Portfolio",
  "main": "server.js",
  "scripts": {
    "start": "node server.js",
    "dev": "nodemon server.js",
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "lint": "eslint .",
    "lint:fix": "eslint . --fix",
    "security-audit": "npm audit --audit-level high",
    "docker:build": "docker build -t portfolio-backend .",
    "docker:dev": "docker-compose up -d"
  },
  "dependencies": {
    "express": "^4.18.2",
    "cors": "^2.8.5",
    "helmet": "^7.1.0",
    "express-rate-limit": "^7.1.5",
    "express-slow-down": "^2.0.1",
    "geoip-lite": "^1.4.8",
    "node-fetch": "^3.3.2",
    "ws": "^8.14.2",
    "chokidar": "^3.5.3",
    "dotenv": "^16.3.1",
    "yamljs": "^0.3.0",
    "joi": "^17.11.0",
    "bcryptjs": "^2.4.3",
    "jsonwebtoken": "^9.0.2",
    "nodemailer": "^6.9.7",
    "winston": "^3.11.0",
    "compression": "^1.7.4",
    "cookie-parser": "^1.4.6"
  },
  "devDependencies": {
    "nodemon": "^3.0.2",
    "jest": "^29.7.0",
    "supertest": "^6.3.3",
    "eslint": "^8.55.0",
    "eslint-config-standard": "^17.1.0",
    "eslint-plugin-security": "^1.7.1",
    "@types/jest": "^29.5.8",
    "swagger-jsdoc": "^6.2.8",
    "swagger-ui-express": "^5.0.0"
  },
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=9.0.0"
  },
  "keywords": [
    "portfolio",
    "rust",
    "wasm",
    "api",
    "security",
    "docker"
  ],
  "author": "Your Name",
  "license": "MIT"
}
```

## 🏗️ Build Configuration

### Trunk Configuration (trunk.toml)
```toml
[build]
target = "index.html"
dist = "dist"
public_url = "/"

[watch]
watch = ["src", "index.html", "styles", "static"]
ignore = ["dist", "target"]

[serve]
address = "0.0.0.0"
port = 8080
open = false
no_autoreload = false

[clean]
dist = "dist"
cargo = true

[[hooks]]
stage = "pre_build"
command = "echo"
command_arguments = ["Starting Rust/WASM build..."]

[[hooks]]
stage = "post_build"
command = "echo"
command_arguments = ["Build completed successfully!"]

[tools]
sass = { version = "1.69" }
wasm-opt = { version = "0.116" }
```

### Docker Development (docker-compose.yml)
```yaml
version: '3.8'

services:
  frontend:
    build:
      context: .
      dockerfile: Dockerfile.dev
    ports:
      - "8080:8080"
    volumes:
      - .:/app
      - /app/target
      - /app/node_modules
    environment:
      - RUST_LOG=debug
      - TRUNK_SERVE_PORT=8080
      - TRUNK_SERVE_HOST=0.0.0.0
    depends_on:
      - backend
    networks:
      - portfolio-network

  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile.dev
    ports:
      - "3000:3000"
      - "3001:3001"  # WebSocket port
    volumes:
      - ./backend:/app
      - ./plugins:/app/plugins
      - /app/node_modules
    environment:
      - NODE_ENV=development
      - PORT=3000
      - WS_PORT=3001
      - OPENROUTER_API_KEY=${OPENROUTER_API_KEY}
      - GITHUB_TOKEN=${GITHUB_TOKEN}
      - SITE_URL=http://localhost:8080
      - ALLOWED_ORIGINS=http://localhost:8080
    networks:
      - portfolio-network

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
    volumes:
      - ./nginx/nginx.dev.conf:/etc/nginx/nginx.conf
      - ./dist:/usr/share/nginx/html
    depends_on:
      - frontend
      - backend
    networks:
      - portfolio-network

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    networks:
      - portfolio-network

  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=portfolio_analytics
      - POSTGRES_USER=portfolio
      - POSTGRES_PASSWORD=${DB_PASSWORD:-development}
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./backend/sql/init.sql:/docker-entrypoint-initdb.d/init.sql
    networks:
      - portfolio-network

volumes:
  redis_data:
  postgres_data:

networks:
  portfolio-network:
    driver: bridge
```

## 🔐 Environment Variables

### Required Environment Variables (.env.example)
```bash
# API Keys
OPENROUTER_API_KEY=sk-or-your-openrouter-api-key-here
GITHUB_TOKEN=ghp_your-github-personal-access-token-here

# Server Configuration
NODE_ENV=development
PORT=3000
WS_PORT=3001
SITE_URL=http://localhost:8080
ALLOWED_ORIGINS=http://localhost:8080

# Database (Optional)
DB_PASSWORD=your-secure-database-password
DB_HOST=postgres
DB_PORT=5432
DB_NAME=portfolio_analytics
DB_USER=portfolio

# Security Settings
JWT_SECRET=your-jwt-secret-for-session-management
BCRYPT_ROUNDS=12

# Email Configuration (For contact form)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your-email@gmail.com
SMTP_PASSWORD=your-app-password
CONTACT_EMAIL=contact@your-domain.com

# Monitoring & Alerts
SLACK_WEBHOOK_URL=https://hooks.slack.com/services/your-webhook-url
LOG_LEVEL=info

# Rate Limiting
RATE_LIMIT_WINDOW_MS=60000
RATE_LIMIT_MAX_REQUESTS=100
CHAT_RATE_LIMIT_MAX=20
CONTACT_RATE_LIMIT_MAX=3

# Geographic Blocking
ALLOWED_COUNTRIES=US,CA,GB,FR,DE,ES,IT,NL,BE,CH,AT,SE,NO,DK,FI
BLOCKED_COUNTRIES=CN,RU,KP,IR

# Feature Flags
ENABLE_ANALYTICS=true
ENABLE_CHATBOT=true
ENABLE_CONTACT_FORM=true
ENABLE_PLUGIN_WATCHER=true
ENABLE_GEO_BLOCKING=true
ENABLE_BOT_DETECTION=true

# Development Settings
DEBUG=true
HOT_RELOAD=true
MOCK_APIS=false
```

## 🧪 Testing Strategy

### Comprehensive Testing Coverage
This project uses a multi-layered testing approach detailed in **TESTING.md**:

- **Unit Tests**: 95% code coverage (Rust + Node.js)
- **Integration Tests**: API endpoints and plugin system
- **Security Tests**: Rate limiting, bot detection, geo-blocking
- **Performance Tests**: Load testing and memory profiling
- **E2E Tests**: Full user workflows with Playwright
- **Visual Regression**: Theme and UI consistency
- **Accessibility**: WCAG 2.1 AA compliance

### Testing Protocols
- **Each stage** has specific test checkpoints (see PROMPTS.md)
- **Automated CI/CD** runs full test suite on every commit
- **Self-testing** protocols for runtime validation
- **Debug integration** with automated issue detection

**→ See TESTING.md for complete testing implementation details, tools, and automation scripts.**

### Quick Test Commands
```bash
# Frontend tests
cargo test
wasm-pack test --headless --firefox

# Backend tests  
cd backend && npm test

# Full test suite
npm run test:all

# Performance tests
npm run test:performance

# Security tests
./scripts/security-test.sh
```

## 🚀 Deployment Instructions

### Production Server Setup
```bash
# Run on your VPS/dedicated server
curl -fsSL https://raw.githubusercontent.com/yourusername/rust-portfolio/main/scripts/server-setup.sh | bash

# Or manually:
./scripts/server-setup.sh
```

### SSL Certificate Setup
```bash
# Install certbot
sudo apt install certbot

# Get SSL certificate
sudo certbot certonly --standalone -d yourdomain.com

# Copy certificates
sudo cp /etc/letsencrypt/live/yourdomain.com/fullchain.pem nginx/ssl/cert.pem
sudo cp /etc/letsencrypt/live/yourdomain.com/privkey.pem nginx/ssl/key.pem

# Set up auto-renewal
echo "0 2 * * 0 certbot renew --pre-hook 'docker-compose stop' --post-hook 'docker-compose start'" | sudo crontab -
```

### Production Deployment
```bash
# Clone repository
git clone https://github.com/yourusername/rust-portfolio.git
cd rust-portfolio

# Set up environment
cp .env.example .env
# Edit .env with your actual values

# Build and deploy
docker-compose -f docker-compose.prod.yml up -d

# Verify deployment
curl -f http://localhost/health
```

## 🔍 Troubleshooting Guide

### Common Issues and Solutions

#### WASM Loading Issues
```javascript
// Check if WASM is properly served with correct MIME type
// In nginx.conf, ensure:
location ~* \.wasm$ {
    add_header Content-Type application/wasm;
    add_header Cross-Origin-Embedder-Policy require-corp;
    add_header Cross-Origin-Opener-Policy same-origin;
}
```

#### Plugin Loading Failures
```bash
# Check plugin directory permissions
chmod -R 755 plugins/

# Verify YAML syntax
yamllint plugins/plugins.yaml

# Check backend logs
docker-compose logs backend

# Test plugin API endpoint
curl http://localhost:3000/api/plugins/status
```

#### Performance Issues
```bash
# Check memory usage
docker stats

# Monitor network requests
# Use browser dev tools Network tab

# Check WGPU rendering performance
# Use browser Performance tab

# Optimize WASM bundle
wasm-opt -O3 target/wasm32-unknown-unknown/release/rust_portfolio.wasm -o optimized.wasm
```

#### Security Alerts
```bash
# Check rate limiting
curl -I http://localhost/api/chat

# Verify geographic blocking
curl -H "X-Forwarded-For: 1.2.3.4" http://localhost/

# Test bot detection
curl -H "User-Agent: python-requests/2.25.1" http://localhost/
```

## 📋 Implementation Approach

### Documentation Structure
This implementation uses a coordinated documentation system:

- **PROMPTS.md** - 14-stage incremental implementation guide with context management
- **TESTING.md** - Comprehensive testing strategies and automation 
- **DEBUG.md** - Autonomous debugging and error recovery protocols
- **SUCCESS.md** - Intelligence-driven development and self-recovery strategies

### Implementation Workflow
1. **Follow PROMPTS.md** for stage-by-stage implementation (respects context limits)
2. **Use TESTING.md** for comprehensive test coverage at each stage
3. **Apply DEBUG.md** when issues arise (autonomous problem-solving)
4. **Leverage SUCCESS.md** for intelligent defaults and adaptive strategies

### Verification Process
Each stage should achieve:
- ✅ **Compiles successfully** (no build errors)
- ✅ **Tests pass** (using TESTING.md protocols)
- ✅ **Functionality verified** (using DEBUG.md validation)
- ✅ **Ready for next stage** (clear handoff point)
- ✅ **Self-recovering** (using SUCCESS.md patterns)

See **PROMPTS.md** for detailed stage-by-stage implementation instructions.

## 📞 Support and Resources

### Documentation Links
- [Leptos Guide](https://leptos-rs.github.io/leptos/)
- [WGPU Tutorial](https://sotrh.github.io/learn-wgpu/)
- [Docker Compose Reference](https://docs.docker.com/compose/)
- [nginx Configuration](https://nginx.org/en/docs/)

### Command Reference
```bash
# Development commands
trunk serve                           # Start dev server
cargo test                           # Run Rust tests
docker-compose up -d                 # Start all services
docker-compose logs -f backend       # View backend logs

# Build commands
trunk build --release               # Build production frontend
docker build -t portfolio .         # Build production image
./scripts/build.sh                  # Run complete build

# Deployment commands
./scripts/deploy.sh                 # Deploy to production
./scripts/monitor.sh                # Check system health
./scripts/backup.sh                 # Backup data
```

### Performance Targets
- **First Contentful Paint**: < 1.5s
- **Largest Contentful Paint**: < 2.5s
- **Time to Interactive**: < 3.5s
- **Cumulative Layout Shift**: < 0.1
- **API Response Time**: < 200ms (95th percentile)
- **WASM Load Time**: < 500ms
- **Plugin Discovery**: < 100ms

This guide provides everything Claude Code needs to implement the complete self-hosted Rust portfolio with plugin system, security features, and production deployment capabilities.
