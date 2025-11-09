# Z-Cleaner Quick Start Guide

## Prerequisites

Before you begin, ensure you have installed:

- **Node.js** (v16 or higher) - [Download](https://nodejs.org/)
- **Rust** (v1.70 or higher) - [Install](https://rustup.rs/)
- **Tauri CLI** - Will be installed via npm

## Installation & Setup (5 minutes)

### Step 1: Navigate to project directory
```bash
cd /Users/elykik/Documents/Dev/z-cleaner-2
```

### Step 2: Install Node dependencies
```bash
npm install
```

This will install:
- React & React DOM
- Tauri API
- TailwindCSS
- Lucide React icons
- Framer Motion
- Vite (build tool)

### Step 3: Install Rust dependencies
The Rust dependencies will be automatically downloaded when you first run the dev server.

## Running the Application

### Development Mode (with hot reload)
```bash
npm run dev
```

This will:
1. Start the Vite dev server on `http://localhost:5173`
2. Launch the Tauri development window
3. Enable hot module replacement (HMR) for React changes

The window will automatically reload when you modify React files.

### Building for Production
```bash
npm run build
```

This will:
1. Build the React frontend
2. Compile the Rust backend
3. Create platform-specific installers:
   - macOS: `.dmg` file
   - Windows: `.msi` installer

Built files will be in `src-tauri/target/release/`.

## Project Structure Overview

```
z-cleaner-2/
├── src/                          # React Frontend
│   ├── components/               # React components
│   │   ├── Dashboard.tsx        # Main dashboard
│   │   ├── Cleaner.tsx          # Cleaning operations
│   │   ├── Analyzer.tsx         # Disk analysis
│   │   ├── Optimizer.tsx        # System optimization
│   │   └── Settings.tsx         # App settings
│   ├── App.tsx                  # Main app component
│   ├── main.tsx                 # React entry point
│   └── index.css                # Global styles
│
├── src-tauri/                    # Rust Backend
│   ├── src/
│   │   ├── main.rs              # Tauri app entry
│   │   ├── models.rs            # Data structures
│   │   └── commands/            # Tauri commands
│   │       ├── analyzer.rs      # Disk analysis logic
│   │       ├── cleaner.rs       # Cleaning logic
│   │       └── optimizer.rs     # Optimization logic
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri config
│
├── package.json                 # Node dependencies
├── tailwind.config.js           # TailwindCSS config
├── vite.config.ts               # Vite config
└── tsconfig.json                # TypeScript config
```

## Key Features

### 🧹 Cleaner Tab
- Remove temporary files
- Clear browser cache (Chrome, Firefox, Safari)
- Delete old log files
- Clean application caches
- View cleaning history

### 🔍 Analyzer Tab
- Scan disk for large files (>100MB)
- View file sizes and modification dates
- Monitor disk usage percentage
- Identify space-saving opportunities

### ⚙️ Optimizer Tab
- Manage startup programs
- Enable/disable programs at boot
- View program sizes
- Get optimization tips

### 📊 Dashboard Tab
- Real-time disk statistics
- System health status
- Quick action buttons
- Large files counter

### ⚙️ Settings Tab
- Theme selection (Light/Dark/Auto)
- Language preferences
- Auto-scan configuration
- Safe mode toggle

## Common Commands

| Command | Purpose |
|---------|---------|
| `npm run dev` | Start development server |
| `npm run build` | Build for production |
| `npm install` | Install dependencies |
| `cargo clean` | Clean Rust build cache |
| `npm run type-check` | Check TypeScript types |

## Troubleshooting

### Issue: "Cannot find module 'react'"
**Solution**: Run `npm install` again
```bash
npm install
```

### Issue: Tauri window won't open
**Solution**: Check for errors in console
```bash
npm run dev
# Look for error messages in the terminal
```

### Issue: Build fails with Rust errors
**Solution**: Update Rust and clean cache
```bash
rustup update
cargo clean
npm run build
```

### Issue: Port 5173 already in use
**Solution**: Kill the process or use a different port
```bash
# macOS/Linux
lsof -ti:5173 | xargs kill -9

# Or edit vite.config.ts to use different port
```

## Development Tips

### Hot Reload
- React changes auto-reload (HMR enabled)
- Rust changes require restart: `npm run dev`

### Debugging
- Open DevTools: `Ctrl+Shift+I` (Windows) or `Cmd+Option+I` (macOS)
- Check console for errors
- Use `console.log()` in React components

### Performance
- Use React DevTools browser extension
- Profile with Tauri's built-in profiler
- Check Rust performance with `cargo flamegraph`

## Next Steps

1. **Explore the code**: Check out the components in `src/components/`
2. **Modify UI**: Edit components to customize the interface
3. **Add features**: Extend commands in `src-tauri/src/commands/`
4. **Test**: Run `npm run dev` and test your changes
5. **Build**: Create production build with `npm run build`

## Resources

- [Tauri Documentation](https://tauri.app/docs/)
- [React Documentation](https://react.dev/)
- [TailwindCSS Documentation](https://tailwindcss.com/docs)
- [Rust Book](https://doc.rust-lang.org/book/)

## Support

For issues or questions:
1. Check the README.md for more details
2. Review error messages in the console
3. Check Tauri and React documentation
4. Open an issue on GitHub

---

**Happy coding! 🚀**
