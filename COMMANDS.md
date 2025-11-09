# Z-Cleaner - Useful Commands Reference

Quick reference for all common commands used in Z-Cleaner development.

## 🚀 Getting Started

### Initial Setup
```bash
# Navigate to project
cd /Users/elykik/Documents/Dev/z-cleaner-2

# Install all dependencies
npm install

# Verify installation
npm list react react-dom @tauri-apps/api
```

## 🔧 Development Commands

### Start Development Server
```bash
# Start with hot reload
npm run dev

# This will:
# 1. Start Vite dev server on http://localhost:5173
# 2. Compile Rust backend
# 3. Open Tauri window automatically
# 4. Enable hot module replacement (HMR)
```

### Type Checking
```bash
# Check TypeScript errors without building
npm run type-check

# Fix TypeScript errors
npx tsc --noEmit
```

### Build for Production
```bash
# Full production build
npm run build

# This will:
# 1. Build React frontend
# 2. Compile Rust backend (release mode)
# 3. Create platform-specific installers
# 4. Output to src-tauri/target/release/bundle/
```

## 🦀 Rust/Cargo Commands

### Rust Development
```bash
# Check Rust code without building
cd src-tauri
cargo check

# Run tests
cargo test

# Build debug version
cargo build

# Build release version
cargo build --release

# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated
```

### Rust Formatting & Linting
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Lint code
cargo clippy

# Fix clippy warnings
cargo clippy --fix
```

## 📦 npm Commands

### Package Management
```bash
# Install dependencies
npm install

# Install specific package
npm install package-name

# Install dev dependency
npm install --save-dev package-name

# Update all packages
npm update

# Check for outdated packages
npm outdated

# List installed packages
npm list

# Audit for security vulnerabilities
npm audit

# Fix security vulnerabilities
npm audit fix
```

## 🎨 Frontend Development

### React/TypeScript
```bash
# Type check
npm run type-check

# View dependencies
npm list react

# Check for unused dependencies
npm ls --depth=0
```

### Styling
```bash
# Build TailwindCSS
npx tailwindcss -i ./src/index.css -o ./dist/output.css

# Watch mode
npx tailwindcss -i ./src/index.css -o ./dist/output.css --watch
```

## 🏗️ Build & Deployment

### Production Build
```bash
# Full build
npm run build

# Build only frontend
npm run build -- --no-backend

# Build only backend
cd src-tauri && cargo build --release && cd ..
```

### Platform-Specific Builds
```bash
# macOS build
npm run build

# Windows build (on Windows)
npm run build

# Both platforms (requires cross-compilation setup)
npm run build
```

## 🧹 Cleanup Commands

### Clear Cache
```bash
# Clear npm cache
npm cache clean --force

# Clear Vite cache
rm -rf dist

# Clear Rust build cache
cd src-tauri && cargo clean && cd ..

# Clear node_modules
rm -rf node_modules package-lock.json
npm install
```

### Full Reset
```bash
# Complete clean slate
rm -rf node_modules dist src-tauri/target package-lock.json
npm install
npm run dev
```

## 🐛 Debugging Commands

### Enable Debug Logging
```bash
# Rust debug logging
RUST_LOG=debug npm run dev

# Verbose Rust logging
RUST_LOG=trace npm run dev

# Specific module logging
RUST_LOG=z_cleaner=debug npm run dev
```

### Development Tools
```bash
# Open browser DevTools (in Tauri window)
Cmd+Option+I (macOS)
Ctrl+Shift+I (Windows)

# Reload window
Cmd+R (macOS)
Ctrl+R (Windows)

# Reload without cache
Cmd+Shift+R (macOS)
Ctrl+Shift+R (Windows)
```

## 📊 Analysis Commands

### Code Analysis
```bash
# TypeScript strict check
npx tsc --strict --noEmit

# ESLint (if configured)
npx eslint src/

# Rust clippy
cd src-tauri && cargo clippy && cd ..

# Rust fmt check
cd src-tauri && cargo fmt --check && cd ..
```

### Dependency Analysis
```bash
# Show dependency tree
npm list

# Show dependency tree for specific package
npm list react

# Check for circular dependencies
npm ls --all

# Analyze bundle size
npm run build && npx webpack-bundle-analyzer dist/stats.json
```

## 🔍 Verification Commands

### Verify Installation
```bash
# Check Node.js
node --version

# Check npm
npm --version

# Check Rust
rustc --version

# Check Cargo
cargo --version

# Check Tauri CLI
tauri --version
```

### Verify Project
```bash
# Check all files exist
ls -la src/
ls -la src-tauri/src/
ls -la src-tauri/src/commands/

# Check dependencies installed
npm list --depth=0

# Check Cargo dependencies
cd src-tauri && cargo tree && cd ..
```

## 🚀 Quick Workflows

### Daily Development
```bash
# 1. Start dev server
npm run dev

# 2. Make changes to React components
# (Auto-reload in window)

# 3. Make changes to Rust
# (Restart dev server)

# 4. Test in browser DevTools
# (Cmd+Option+I)
```

### Before Committing
```bash
# 1. Type check
npm run type-check

# 2. Format code
cd src-tauri && cargo fmt && cd ..

# 3. Lint code
cd src-tauri && cargo clippy && cd ..

# 4. Test build
npm run build
```

### Preparing for Release
```bash
# 1. Update version in package.json
# 2. Update version in src-tauri/Cargo.toml
# 3. Update version in src-tauri/tauri.conf.json

# 4. Clean build
rm -rf dist src-tauri/target
npm run build

# 5. Test installer
# Install and test the .dmg or .msi file

# 6. Sign and notarize (if needed)
# Configure in src-tauri/tauri.conf.json
```

## 🔐 Security Commands

### Audit Dependencies
```bash
# Check npm security
npm audit

# Fix vulnerabilities
npm audit fix

# Check Rust security
cd src-tauri && cargo audit && cd ..
```

## 📝 Useful Aliases

Add these to your `.zshrc` or `.bashrc`:

```bash
# Z-Cleaner aliases
alias zc='cd /Users/elykik/Documents/Dev/z-cleaner-2'
alias zc-dev='npm run dev'
alias zc-build='npm run build'
alias zc-check='npm run type-check'
alias zc-clean='rm -rf node_modules dist src-tauri/target && npm install'
```

Usage:
```bash
zc          # Navigate to project
zc-dev      # Start development
zc-build    # Build for production
zc-check    # Type check
zc-clean    # Full clean and reinstall
```

## 🎯 Common Scenarios

### Scenario: App won't start
```bash
# 1. Check Node version
node --version

# 2. Clear cache
npm cache clean --force

# 3. Reinstall
rm -rf node_modules package-lock.json
npm install

# 4. Try again
npm run dev
```

### Scenario: Port already in use
```bash
# Find process using port 5173
lsof -i :5173

# Kill process
kill -9 <PID>

# Or use different port in vite.config.ts
```

### Scenario: Rust compilation error
```bash
# Update Rust
rustup update

# Clean build
cd src-tauri && cargo clean && cd ..

# Try again
npm run dev
```

### Scenario: TypeScript errors
```bash
# Check all errors
npm run type-check

# Fix auto-fixable errors
npx tsc --noEmit --pretty
```

## 📚 Documentation Commands

### View Documentation
```bash
# View README
cat README.md

# View Quick Start
cat QUICKSTART.md

# View Architecture
cat ARCHITECTURE.md

# View all docs
ls -la *.md
```

## 🎓 Learning Commands

### Explore Project
```bash
# List all source files
find src -type f -name "*.tsx" -o -name "*.ts"

# List all Rust files
find src-tauri/src -type f -name "*.rs"

# Count lines of code
find src -type f \( -name "*.tsx" -o -name "*.ts" \) | xargs wc -l
find src-tauri/src -type f -name "*.rs" | xargs wc -l
```

---

## 🔗 Command Quick Reference

| Task | Command |
|------|---------|
| Start dev | `npm run dev` |
| Build prod | `npm run build` |
| Type check | `npm run type-check` |
| Install deps | `npm install` |
| Clean all | `rm -rf node_modules dist src-tauri/target && npm install` |
| Format Rust | `cd src-tauri && cargo fmt && cd ..` |
| Lint Rust | `cd src-tauri && cargo clippy && cd ..` |
| Audit security | `npm audit` |
| View logs | `RUST_LOG=debug npm run dev` |

---

**Save this file for quick reference during development!**
