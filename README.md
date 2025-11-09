# Z-Cleaner

A fast, secure, and modern system cleaner for Windows and macOS, built with Rust, Tauri, and React.

## Features

- 🧹 **System Cleaning**: Remove temporary files, browser cache, logs, and application cache
- 🔍 **Disk Analysis**: Identify large files and monitor disk usage
- ⚙️ **System Optimization**: Manage startup programs and optimize performance
- 🎨 **Modern UI**: Beautiful, responsive interface with light/dark theme
- 🛡️ **Secure**: 100% offline, no data collection, local processing only
- ⚡ **Fast**: Rust backend for optimal performance
- 🔒 **Safe Mode**: Scan without deleting for testing

## Tech Stack

- **Backend**: Rust with Tauri
- **Frontend**: React + TypeScript
- **Styling**: TailwindCSS
- **Icons**: Lucide React
- **Animations**: Framer Motion

## Project Structure

```
z-cleaner-2/
├── src/                          # React frontend
│   ├── components/
│   │   ├── Dashboard.tsx
│   │   ├── Cleaner.tsx
│   │   ├── Analyzer.tsx
│   │   ├── Optimizer.tsx
│   │   └── Settings.tsx
│   ├── App.tsx
│   ├── main.tsx
│   └── index.css
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── models.rs
│   │   └── commands/
│   │       ├── mod.rs
│   │       ├── analyzer.rs
│   │       ├── cleaner.rs
│   │       └── optimizer.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
├── tailwind.config.js
└── vite.config.ts
```

## Installation

### Prerequisites

- Node.js 16+ and npm
- Rust 1.70+
- Tauri CLI

### Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/z-cleaner.git
   cd z-cleaner-2
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Install Tauri CLI** (if not already installed)
   ```bash
   npm install -g @tauri-apps/cli
   ```

## Development

### Run in development mode

```bash
npm run dev
```

This will start the Vite dev server and Tauri dev window.

### Build for production

```bash
npm run build
```

The compiled application will be in `src-tauri/target/release/`.

## Features Breakdown

### Dashboard
- Real-time disk usage statistics
- System health overview
- Quick action buttons
- Large files counter

### Cleaner
- Temporary files removal
- Browser cache cleaning (Chrome, Firefox, Safari)
- Log file cleanup
- Application cache clearing
- Cleaning history with results

### Analyzer
- Disk space analysis
- Large file detection (>100MB)
- File listing with sizes and modification dates
- Rescan functionality

### Optimizer
- Startup program management
- Enable/disable programs
- Performance optimization tips
- System performance monitoring

### Settings
- Theme selection (Light/Dark/Auto)
- Language preferences
- Auto-scan configuration
- Safe mode toggle
- About information

## Security

- ✅ No internet connection required
- ✅ No data collection or telemetry
- ✅ All operations are local
- ✅ Code signing available for Windows/macOS
- ✅ Sandbox mode enabled
- ✅ Safe deletion with validation

## Platform Support

- ✅ macOS 10.13+
- ✅ Windows 7+
- 🔜 Linux (planned)

## Configuration

Edit `src-tauri/tauri.conf.json` to customize:
- Window size and properties
- Security settings
- Bundle configuration
- File system permissions

## Performance

- Lightweight: ~50MB bundle size
- Fast startup: <1s
- Efficient scanning: Multi-threaded with Tokio
- Low memory footprint

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details

## Troubleshooting

### Build fails with Rust errors
- Ensure Rust is up to date: `rustup update`
- Clean build: `cargo clean` in `src-tauri/`

### Frontend not updating
- Clear node_modules: `rm -rf node_modules && npm install`
- Clear Vite cache: `rm -rf dist`

### Tauri window not opening
- Check console for errors: `npm run dev` with verbose logging
- Ensure all dependencies are installed

## Future Enhancements

- [ ] Linux support
- [ ] Real-time performance monitoring
- [ ] Scheduled automatic cleaning
- [ ] Cloud backup integration
- [ ] System restore points
- [ ] Advanced filtering options
- [ ] Duplicate file finder
- [ ] Registry cleaner (Windows)

## Support

For issues, questions, or suggestions, please open an issue on GitHub.

## Credits

Built with ❤️ using Rust, Tauri, and React.
