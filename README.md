# Rust TODO Desktop Application - Complete Setup Guide

**A Modern Cross-Platform TODO Application Built with Rust, Tauri v2, and PostgreSQL**

> 🎓 **Capstone Project**: Moringa School AI Engineering Program (September-October 2025)  
> 🤖 **AI-Assisted Development**: Complete prompt documentation included

---

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#Features)
- [Technology Stack](#TechnologyStack)
- [Prerequisites](#Prerequisites)
- [Installation Guide](#InstallationGuide)
- [Usage](#Usage)
- [Project Structure](#ProjectStructure)
- [Database Architecture](#database-architecture)
- [API Reference](#api-reference)
- [Troubleshooting](#troubleshooting)
- [AI Development Log](#ai-development-log)
- [Building from Source](#building-from-source)
- [Contributing](#contributing)
- [License](#license)

---

# overview

This is a **production-ready desktop TODO application** that demonstrates modern Rust development practices, type-safe database operations, and cross-platform desktop app creation using Tauri v2. Built as a capstone project, it showcases AI-assisted development workflows and clean architecture patterns.

### Why This Project?

- **Learn Rust Ecosystem**: Hands-on experience with Cargo, SQLx, Serde, and async/await
- **Desktop App Development**: Tauri v2 for native performance with web UI
- **Database Integration**: PostgreSQL with migrations, triggers, and custom types
- **AI-Assisted Workflow**: Complete documentation of GenAI prompts used during development

---

---

## 📹 Video Walkthrough & Pitch Deck

A complete video tutorial covering installation, usage, and development is available:

|                                                                        |                                                                                                             |
| ---------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| **[📺 Watch on YouTube](https://www.youtube.com/watch?v=C7wSsC0dq5w)** | **[📑 View Pitch Deck](https://gamma.app/docs/Rust-TODO-Local-First-Desktop-Productivity-ezquliyogepn8ab)** |

**Topics covered**

- Installing prerequisites on WSL Ubuntu
- Database setup and migration
- Running the application
- Code walkthrough
- Common troubleshooting steps

---

# Features

### Core Functionality

- ✅ **Full CRUD Operations**: Create, read, update, delete todos with validation
- 🎯 **Priority Management**: Four-level priority system (low, medium, high, urgent)
- ✔️ **Smart Completion**: Toggle status with automatic timestamp tracking
- 🔍 **Filtering**: View all tasks, active only, or completed only
- 📊 **Sorting**: By creation date, priority level, or alphabetical title
- 📈 **Live Statistics**: Real-time counters for total, active, and completed tasks

### Technical Highlights

- 🦀 **Type-Safe Backend**: Compile-time SQL verification with SQLx
- 🚀 **Native Performance**: Tauri v2 for <5MB binaries with Rust speed
- 🗄️ **Robust Database**: PostgreSQL with ACID compliance and custom enums
- 🔒 **Secure**: No localStorage usage, all data in PostgreSQL
- 🎨 **Clean UI**: Vanilla JavaScript with no framework dependencies
- 🌐 **Cross-Platform**: Builds for Linux, Windows (WSL/native)

---

# TechnologyStack

| Layer                 | Technology          | Version | Purpose                                           |
| --------------------- | ------------------- | ------- | ------------------------------------------------- |
| **Backend**           | Rust                | 1.70+   | Core application logic                            |
| **Desktop Framework** | Tauri               | v2.0    | Native desktop wrapper                            |
| **Database**          | PostgreSQL          | 14+     | Data persistence                                  |
| **ORM**               | SQLx                | 0.8.6   | Async database queries with compile-time checking |
| **Frontend**          | Vanilla JS/HTML/CSS | ES6+    | User interface (no frameworks)                    |
| **Build Tool**        | Cargo + Tauri CLI   | Latest  | Compilation and bundling                          |
| **Environment**       | dotenvy             | 0.15    | Configuration management                          |

---

# Prerequisites

### System Requirements

#### For Linux/WSL Users (Primary Target)

###### For first time installation of a new wsl ditro like debian(preferred)

ensure you update the terminal

```bash
#updates
sudo apt update
```

then upgrade the terminal

```bash
#upgrade
sudo apt upgrade
```

type Y to allow all

**1. Rust Toolchain**
install curl if not yet installed

```bash
sudo apt install curl
```

```bash
# Install Rust (includes cargo) when prompted press enter for the default
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installation
rustc --version  # Should show 1.70+ or higher
cargo --version
```

**2. PostgreSQL Database**

```bash
# Install PostgreSQL
sudo apt install -y postgresql postgresql-contrib

# Start PostgreSQL service
sudo service postgresql start

# Verify it's running
sudo service postgresql status
```

**3. Tauri System Dependencies**

````bash
# Install all required development libraries
sudo apt install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# For Ubuntu 24.04 specifically
#if apt is noisy about a package not starting from the delayed queue, confirm if it really installed by running
```bash
dpkg -l | grep libxvidcore4
```

# Note: Use libwebkit2gtk-4.1-dev, NOT 4.0-dev (deprecated)

**5. Tauri CLI**

```bash
# Install Tauri CLI globally
cargo install tauri-cli --version "^2.0.0"

# Verify installation
cargo tauri --version
````

#### For Windows Users

1. **Rust**: Download from [rustup.rs](https://rustup.rs/)
2. **Node.js**: Download from [nodejs.org](https://nodejs.org/)
3. **PostgreSQL**: Download from [postgresql.org](https://www.postgresql.org/download/windows/)
4. **Visual Studio Build Tools**: Required for Rust compilation
   - Install "Desktop development with C++" workload

---

# InstallationGuide

### Step 1: Clone the Repository

install git

```bash
sudo apt install git
```

```bash
# Clone from your GitHub repository
git clone https://github.com/kabakadev/rust_todo_application.git
cd rust_todo_application
```

### Step 2: Database Setup

# Database & Environment Setup

This app uses **PostgreSQL** with **SQLx**. Follow these steps to get a working local setup.

---

## 1) Create a Dedicated DB User and Database (Recommended)

Open a Postgres shell:

```bash
# Linux/macOS/WSL
sudo -u postgres psql
```

Then run these SQL commands:

```sql
-- 1) Create a least-privileged app user
CREATE ROLE rust_todo WITH LOGIN PASSWORD 'password';

-- 2) Create the app database owned by that user
CREATE DATABASE rust_todo_db OWNER rust_todo;

-- 3) (Optional) Ensure privileges are explicit
GRANT ALL PRIVILEGES ON DATABASE rust_todo_db TO rust_todo;

-- exit psql
\q
```

> **Why this way?** It mirrors production and avoids using the superuser `postgres` for app connections.

---

## 2) Environment Variables

SQLx reads your connection string in two places:

- `DATABASE_URL` — used at **runtime**.
- `SQLX_DATABASE_URL` — used by **SQLx macros/CLI** at **compile time** (usually set to the same value).

**Recommended contents for `.env` (project root):**
create the .env file

```bash
touch .env
```

open the .env with the default nano

```bash
nano .env
```

then copy these values below and in nano, right click to paste

```dotenv
# Postgres (use the TCP URL that works on your machine)
DATABASE_URL=postgres://rust_todo:password@localhost:5432/rust_todo_db

# Let sqlx-cli and compile-time macros pick it up automatically
SQLX_DATABASE_URL=${DATABASE_URL}

# Optional: Rust logging level
RUST_LOG=debug

# If your app exposes an HTTP layer (adjust/ignore if not used)
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

> **Location:** Put `.env` in the **project root** (not inside `src-tauri/`).
> **Ignore:** `.env` is already in `.gitignore` and won’t be committed.

### Keep `.env.example` in the Repo

Use realistic defaults to match these docs:

```dotenv
# Database Configuration
DATABASE_URL=postgres://rust_todo:password@localhost:5432/rust_todo_db

# Ensures SQLx macros/CLI see the same URL during build-time tasks
SQLX_DATABASE_URL=${DATABASE_URL}

# Server Configuration (if used)
SERVER_HOST=127.0.0.1
SERVER_PORT=8080

# Application Configuration
RUST_LOG=debug
```

## 3) Verify the Connection

Pick any one of the following:

```bash
# Using psql (relies on your shell env)
psql "$DATABASE_URL" -c '\dt'             # should connect; tables may be empty

# Or pass the URL directly
psql "postgres://rust_todo:password@localhost:5432/rust_todo_db" -c '\dt'

# Using sqlx-cli (install first: `cargo install sqlx-cli`)
sqlx database verify                      # reads SQLX_DATABASE_URL / .env
```

If `psql` can connect, your URL is correct.

---

## 4) Run Migrations (If Present)

first install the sqlx-cli tool

```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

```bash
# Run migrations to create tables, enums, and triggers(migrations are at the root directory)
cargo sqlx migrate run

```

**Expected Output:**

```
Applied 20250929121338/migrate create todos table
Applied 20250929154718/init todos
Applied 20250929173325/cleanup legacy updated at trigger
```

### Step 5: Run the Application

**Development Mode (with hot reload):**

```bash
cargo tauri dev
```

> SQLx compile-time checks/macros use `SQLX_DATABASE_URL`. Runtime DB access uses `DATABASE_URL`.

---

## Troubleshooting

- **Authentication failed**
  Check `DATABASE_URL` credentials and that the role exists. In `psql`:
  sudo -u postgres psql

  ```sql
  \du rust_todo
  -- or reset the password
  ALTER ROLE rust_todo WITH PASSWORD 'strongpass';
  \q
  ```

- **Can’t connect to server**
  Ensure Postgres is running and listening on TCP. On Ubuntu:

  ```bash
  sudo systemctl status postgresql
  ```

- **Migrations can’t create tables**
  DB ownership matters. Recreate the DB owned by `rust_todo`:
  sudo -u postgres psql

  ```sql
  -- in psql as postgres superuser:
  DROP DATABASE IF EXISTS rust_todo_db;
  CREATE DATABASE rust_todo_db OWNER rust_todo;
  \q
  ```

- **Windows/PowerShell note**
  When running `psql` with env vars:

  ```powershell
  $env:DATABASE_URL="postgres://rust_todo:password@localhost:5432/rust_todo_db"
  psql $env:DATABASE_URL -c "\dt"
  ```

---

### Step 4: Run Database Migrations

**First Launch Notes:**

- Compilation takes 2-5 minutes on first run
- Subsequent runs are much faster due to caching
- Application window opens automatically
- Console output shows database connection status

---

# 📖 Usage

### Adding Your First Task

1. **Enter Title**: Type task name in the "What needs to be done?" field
2. **Add Description** (Optional): Provide additional details
3. **Set Priority**: Choose from dropdown (Low/Medium/High/Urgent)
4. **Click "Add Task"**: Task appears in the list instantly

### Managing Tasks

#### Toggle Completion

- Click the **checkbox** next to any task
- Completed tasks show with strikethrough styling
- `completed_at` timestamp is automatically recorded

#### Edit Task

1. Click the **"Edit"** button on any task
2. Modify title, description, or priority in the prompts
3. Changes save immediately

#### Delete Task

1. Click the **"Delete"** button
2. Confirm deletion in the dialog
3. Task is permanently removed from database

### Filtering & Sorting

**Filter Buttons:**

- **All**: Shows every task
- **Active**: Only incomplete tasks
- **Completed**: Only finished tasks

**Sort Dropdown:**

- **Date Created**: Newest first (default)
- **Priority**: Urgent → High → Medium → Low
- **Title**: Alphabetical A-Z

### Statistics Footer

Live counters update automatically:

- **Total**: Count of all tasks
- **Active**: Incomplete tasks
- **Completed**: Finished tasks

---

# ProjectStructure

```
rust-todo-app/
├── 📁 frontend/                    # UI Layer (served by Tauri)
│   ├── index.html                 # Main application interface
│   ├── styles.css                 # Modern responsive styling
│   └── app.js                     # Tauri IPC bridge + UI logic
│
├── 📁 src/                         # Core Library (rust-todo-core)
│   ├── lib.rs                     # Public module exports
│   └── 📁 models/                 # Data structures
│       ├── mod.rs                 # Model module exports
│       ├── priority.rs            # Priority enum (SQLx mapped)
│       └── todo.rs                # Todo, CreateTodo, UpdateTodo structs
│
├── 📁 src-tauri/                   # Tauri v2 Application
│   ├── 📁 src/
│   │   ├── lib.rs                 # Tauri commands + app setup
│   │   ├── main.rs                # Entry point
│   │   ├── repo.rs                # Database CRUD operations
│   │   └── error.rs               # Error handling utilities
│   ├── 📁 capabilities/           # Tauri security permissions
│   ├── 📁 icons/                  # Application icons (all sizes)
│   ├── Cargo.toml                 # Tauri dependencies
│   ├── tauri.conf.json            # Tauri configuration
│   └── build.rs                   # Build script
│
├── 📁 migrations/                  # SQLx Database Migrations
│   ├── 20250929121338_create_todos_table.sql
│   ├── 20250929154718_init_todos.sql
│   └── 20250929173325_cleanup_legacy_updated_at_trigger.sql
│
├── .env                           # Database connection (not in git)
├── .gitignore                     # Excludes target/, .env, etc.
├── Cargo.toml                     # Workspace manifest
├── Cargo.lock                     # Dependency lock file
├── README.md                      # This file
└── AI_DEVELOPMENT_LOG.md          # Complete AI prompt history
```

---

# database-architecture

### Schema Overview

```sql
-- Custom Enum Type
CREATE TYPE priority_level AS ENUM ('low', 'medium', 'high', 'urgent');

-- Main Table
CREATE TABLE todos (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    is_completed BOOLEAN NOT NULL DEFAULT false,
    priority priority_level NOT NULL DEFAULT 'medium',
    created_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc'),
    completed_at TIMESTAMPTZ
);

-- Indexes for Performance
CREATE INDEX idx_todos_is_completed ON todos(is_completed);
CREATE INDEX idx_todos_priority ON todos(priority);
CREATE INDEX idx_todos_created_at ON todos(created_at DESC);
```

### Automatic Timestamp Management

**Trigger 1: Auto-Update `updated_at`**

```sql
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at := NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_updated_at
  BEFORE UPDATE ON todos
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();
```

**Trigger 2: Auto-Manage `completed_at`**

```sql
CREATE OR REPLACE FUNCTION set_completed_at()
RETURNS TRIGGER AS $$
BEGIN
  IF NEW.is_completed = TRUE AND OLD.is_completed = FALSE THEN
    NEW.completed_at := NOW();
  ELSIF NEW.is_completed = FALSE AND OLD.is_completed = TRUE THEN
    NEW.completed_at := NULL;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_completed_timestamp
  BEFORE UPDATE ON todos
  FOR EACH ROW
  EXECUTE FUNCTION set_completed_at();
```

---

# api-reference

### Tauri Commands (Backend → Frontend Bridge)

All commands are invoked from JavaScript using:

```javascript
await window.__TAURI__.core.invoke("command_name", { param: value });
```

#### 1. Database Health Check

```rust
#[tauri::command]
async fn db_ping(pool: tauri::State<'_, PgPool>) -> Result<String, String>
```

**Parameters**: None
**Returns**: `"db ok: 1"` on success
**Usage**:

```javascript
const result = await invoke("db_ping");
console.log(result); // "db ok: 1"
```

#### 2. List All Todos

```rust
#[tauri::command]
async fn list_todos(pool: tauri::State<'_, PgPool>) -> Result<Vec<Todo>, String>
```

**Parameters**: None
**Returns**: Array of `Todo` objects (newest first)
**Usage**:

```javascript
const todos = await invoke("list_todos");
// [{ id: 1, title: "Learn Rust", is_completed: false, ... }, ...]
```

#### 3. Get Single Todo

```rust
#[tauri::command]
async fn get_todo(pool: tauri::State<'_, PgPool>, id: i64) -> Result<Todo, String>
```

**Parameters**: `id` (number)
**Returns**: Single `Todo` object
**Usage**:

```javascript
const todo = await invoke("get_todo", { id: 1 });
```

#### 4. Create Todo

```rust
#[tauri::command]
async fn create_todo(pool: tauri::State<'_, PgPool>, payload: CreateTodo) -> Result<Todo, String>
```

**Parameters**: `payload` object with:

- `title` (string, required, max 200 chars)
- `description` (string, optional, max 1000 chars)
- `priority` (string, one of: "low", "medium", "high", "urgent")

**Returns**: Created `Todo` object
**Usage**:

```javascript
const newTodo = await invoke("create_todo", {
  payload: {
    title: "Learn Tauri",
    description: "Build a desktop app",
    priority: "high",
  },
});
```

#### 5. Update Todo (Partial)

```rust
#[tauri::command]
async fn update_todo(pool: tauri::State<'_, PgPool>, id: i64, patch: UpdateTodo) -> Result<Todo, String>
```

**Parameters**:

- `id` (number)
- `patch` (object with any of: `title`, `description`, `priority`, `is_completed`)

**Returns**: Updated `Todo` object
**Usage**:

```javascript
const updated = await invoke("update_todo", {
  id: 1,
  patch: {
    title: "Learn Tauri v2",
    priority: "urgent",
  },
});
```

#### 6. Toggle Completion

```rust
#[tauri::command]
async fn toggle_todo(pool: tauri::State<'_, PgPool>, id: i64, to_completed: bool) -> Result<Todo, String>
```

**Parameters**:

- `id` (number)
- `toCompleted` (boolean) - Note: camelCase in JS, snake_case in Rust

**Returns**: Updated `Todo` object
**Usage**:

```javascript
const toggled = await invoke("toggle_todo", {
  id: 1,
  toCompleted: true,
});
```

#### 7. Delete Todo

```rust
#[tauri::command]
async fn delete_todo(pool: tauri::State<'_, PgPool>, id: i64) -> Result<u64, String>
```

**Parameters**: `id` (number)
**Returns**: Number of rows deleted (1 on success, 0 if not found)
**Usage**:

```javascript
const rowsDeleted = await invoke("delete_todo", { id: 1 });
if (rowsDeleted === 1) {
  console.log("Todo deleted successfully");
}
```

---

# troubleshooting

### Common Issues & Solutions

#### 1. Database Connection Fails

**Symptom:**

```
Error: error connecting to database: Connection refused
```

**Solutions:**

```bash
# Check if PostgreSQL is running
sudo service postgresql status

# If not running, start it
sudo service postgresql start

# Verify database exists
psql -U postgres -l | grep rust_todo_db

# Test connection manually
psql postgresql://postgres:password@localhost/rust_todo_db
```

#### 2. Build Fails: Missing Dependencies (Linux)

**Symptom:**

```
error: failed to run custom build command for `webkit2gtk-sys`
```

**Solution:**

```bash
# Install all required system libraries
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    libssl-dev \
    librsvg2-dev
```

#### 3. Tauri API Not Available in Browser

**Symptom:**

```javascript
TypeError: Cannot read property 'invoke' of undefined
```

**Solution:**
Ensure `tauri.conf.json` has:

```json
{
  "app": {
    "withGlobalTauri": true,
    "windows": [
      {
        "title": "Rust TODO",
        "width": 1000,
        "height": 700
      }
    ]
  }
}
```

#### 4. SQLx Compile-Time Verification Fails

**Symptom:**

```
error: error returned from database: relation "todos" does not exist
```

**Solution:**

```bash
# Ensure migrations have run
cd src-tauri
cargo sqlx migrate run

# Regenerate SQLx metadata
cargo sqlx prepare --database-url postgresql://postgres:password@localhost/rust_todo_db

# Clean and rebuild
cargo clean
cargo build
```

#### 5. Port Already in Use (Dev Mode)

**Symptom:**

```
Error: Address already in use (os error 98)
```

**Solution:**

```bash
# Find process using port 1420 (Tauri default)
lsof -i :1420

# Kill the process
kill -9 <PID>

# Or use a different port in tauri.conf.json
```

#### 6. Environment Variables Not Loading

**Symptom:**

```
thread 'main' panicked at 'DATABASE_URL not set'
```

**Solution:**

```bash
# Verify .env exists in project root
ls -la .env

# Check contents
cat .env

# Ensure correct format (no spaces around =)
DATABASE_URL=postgresql://postgres:password@localhost/rust_todo_db

# Try absolute path in lib.rs
dotenvy::from_filename("/home/youruser/rust-todo-app/.env")
```

---

# ai-development-log

This project was built using AI-assisted development. Below is the complete chronicle of GenAI prompts used during development.

### Phase 1: Project Setup (Monday, Sept 29)

**Prompt 1: Initial Project Structure**

```
Help me set up a new Rust project structure for a TODO CRUD application using Tauri
and PostgreSQL. Include the necessary dependencies in Cargo.toml and explain the
project folder structure.
```

**AI Response Summary**: Provided Cargo workspace setup with separate library and Tauri app crates, recommended dependencies (tauri, sqlx, serde, tokio), and folder structure explanation.

**Outcome**: ✅ Created `Cargo.toml` workspace with `rust-todo-core` library and `src-tauri` application.

---

**Prompt 2: SQLx Setup and Connection**

```
Show me how to set up SQLx with PostgreSQL in Rust, including connection pooling,
environment variables for database URL, and basic connection testing.
```

**AI Response Summary**: Demonstrated `PgPoolOptions` configuration, `dotenvy` for environment variables, and connection health check query.

**Outcome**: ✅ Implemented database connection in `lib.rs` with proper pool settings and sanity ping.

---

### Phase 2: Database Design (Monday-Tuesday)

**Prompt 3: Database Schema Creation**

```
Create Rust structs for a TODO application using serde for serialization, and show me
how to create database migration files with SQLx for PostgreSQL.
```

**AI Response Summary**: Provided migration SQL with `CREATE TABLE`, custom enum type, triggers for automatic timestamps, and corresponding Rust structs with `#[derive(sqlx::FromRow)]`.

**Outcome**: ✅ Created migrations with enum support, triggers, and indexes.

---

**Prompt 4: CRUD Operations Implementation**

```
Implement CRUD operations for a TODO application in Rust using SQLx and PostgreSQL.
Include functions for create, read all, read by ID, update, and delete operations
with proper error handling.
```

**AI Response Summary**: Showed `query_as` usage for type-safe queries, `COALESCE` for partial updates, and `RETURNING` clause for immediate results.

**Outcome**: ✅ Implemented `repo.rs` with all CRUD functions using runtime queries.

---

### Phase 3: Frontend Integration (Wednesday)

**Prompt 5: Frontend UI Creation**

```
Create a simple HTML/CSS/JavaScript frontend for a TODO application that will work
with Tauri. Include a form for adding todos, a list for displaying todos, and buttons
for edit/delete operations.
```

**AI Response Summary**: Provided semantic HTML structure, modern CSS with flexbox, and JavaScript event delegation patterns.

**Outcome**: ✅ Built `frontend/` directory with clean UI and no framework dependencies.

---

**Prompt 6: Tauri Command Binding**

```
Show me how to create Tauri commands to connect my Rust backend functions with the
JavaScript frontend. Include examples for all CRUD operations.
```

**AI Response Summary**: Demonstrated `#[tauri::command]` macro, `tauri::State` for dependency injection, and `invoke_handler` registration.

**Outcome**: ✅ Connected all backend functions to frontend via Tauri IPC.

---

**Prompt 7: Frontend-Backend Communication**

```
Demonstrate how to call Rust functions from JavaScript in a Tauri application for a
TODO app. Show proper error handling and loading states in the frontend.
```

**AI Response Summary**: Showed `window.__TAURI__.core.invoke()` usage, try-catch error handling, and UI state management.

**Outcome**: ✅ Implemented robust error handling and loading indicators in `app.js`.

---

### Phase 4: Testing & Build (Wednesday-Thursday)

**Prompt 8: Building and Packaging**

```
Guide me through building and packaging my Tauri TODO application for distribution
on Linux/WSL. Include development and production build commands.
```

**AI Response Summary**: Explained `cargo tauri dev` for development, `cargo tauri build` for production, and output locations for different platforms.

**Outcome**: ✅ Successfully built `.AppImage`, `.deb`, and `.rpm` packages.

---

### Key Learnings from AI Assistance

1. **Rapid Prototyping**: AI helped scaffold the entire project in 2 days vs. estimated 1 week
2. **Best Practices**: Learned idiomatic Rust patterns (error handling, async/await, derive macros)
3. **Debugging**: AI helped resolve SQLx compile-time verification and Tauri CSP issues
4. **Architecture**: Guided toward clean separation of concerns (models → repo → commands)
5. **Documentation**: Assisted in writing comprehensive README and inline comments

**Total AI Interactions**: ~15-20 prompts across 4 development phases
**Time Saved**: Estimated 30-40 hours of documentation reading and trial-and-error

---

# building-from-source

### Development Build

```bash
# Clean previous builds
cargo clean

# Run in development mode with hot reload
cargo tauri dev

# Application opens automatically
# File changes trigger automatic recompilation
```

### Production Build

```bash
# Full release build with optimizations
cargo tauri build

# Build output locations:
# Linux:
#   - AppImage: src-tauri/target/release/bundle/appimage/
#   - Debian: src-tauri/target/release/bundle/deb/
#   - RPM: src-tauri/target/release/bundle/rpm/
#
# Windows:
#   - MSI: src-tauri\target\release\bundle\msi\
#   - NSIS: src-tauri\target\release\bundle\nsis\
```

### Build Optimization Tips

```bash
# Parallel compilation (faster builds)
export CARGO_BUILD_JOBS=4

# Skip unused dependencies
cargo build --release --no-default-features

# Smaller binary size
[profile.release]
strip = true
opt-level = "z"
lto = true
codegen-units = 1
```

### Cross-Compilation (Advanced)

```bash
# Install target
rustup target add x86_64-pc-windows-gnu

# Build for Windows from Linux
cargo tauri build --target x86_64-pc-windows-gnu
```

# contributing

While this is a capstone project, contributions and feedback are welcome:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Guidelines

- Follow Rust idioms (run `cargo clippy`)
- Format code with `cargo fmt`
- Add tests for new features
- Update documentation
- Ensure migrations are reversible

---

# license

This project is licensed under the **MIT License**.

```
MIT License

Copyright (c) 2025 Ian Kabaka

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.


```

---

## 🙏 Acknowledgments

- **Moringa School**: For the AI Engineering Capstone Program framework
- **Tauri Team**: For the excellent desktop application framework and documentation
- **SQLx Contributors**: For compile-time SQL verification and async PostgreSQL support
- **Rust Community**: For comprehensive documentation, crates.io ecosystem, and helpful forums
- **AI Assistants**: Claude (Anthropic) and chatgpt(OpenAI) for development guidance throughout the project

---

## 📞 Support & Contact

**Project Author**: Ian Kabaka
**Institution**: Moringa School AI Engineering Program
**Cohort**: September-October 2025
**Project Type**: Capstone (Individual)

**Resources**:

- 📚 [Rust Documentation](https://doc.rust-lang.org/)
- 🖥️ [Tauri Documentation](https://tauri.app/v2/guide/)
- 🐘 [PostgreSQL Docs](https://www.postgresql.org/docs/)
- 🦀 [SQLx Repository](https://github.com/launchbadge/sqlx)

---

## 🎓 Educational Context

**Capstone Requirements Met**:

- ✅ New technology exploration (Tauri v2, SQLx)
- ✅ Runnable project with clear setup instructions
- ✅ Complete AI prompt documentation
- ✅ Troubleshooting guide with common errors
- ✅ Reference resources and learning reflections
- ✅ GitHub repository with comprehensive README
- ✅ Peer testing completed

**Skills Demonstrated**:

- Rust systems programming
- Async/await concurrency
- Database design and migrations
- Desktop application development
- IPC communication patterns
- Error handling and validation
- Cross-platform build processes
- AI-assisted development workflows

---

**Built with 🦀 Rust | Powered by 🚀 Tauri | Stored in 🐘 PostgreSQL**

_Last Updated: October 1, 2025_

```

```
