# 🚀 START HERE - Z-Cleaner Quick Launch

Welcome to Z-Cleaner! This file will get you up and running in **5 minutes**.

## ⚡ Quick Start (Copy & Paste)

### 1. Install Dependencies (2 minutes)
```bash
cd /Users/elykik/Documents/Dev/z-cleaner-2
npm install
```

### 2. Run the App (1 minute)
```bash
npm run dev
```

**That's it!** The app will open automatically. You should see:
- ✅ A desktop window with Z-Cleaner
- ✅ Dark theme by default
- ✅ Dashboard with system stats
- ✅ Sidebar with navigation

## 🎯 What to Try First

### 1. Explore the Dashboard
- Click the "Refresh" button
- Watch the disk stats update
- See the progress bars

### 2. Try the Analyzer
- Click "Analyzer" in the sidebar
- Click "Rescan" button
- View the list of large files

### 3. Test the Cleaner
- Click "Cleaner" in the sidebar
- Check the boxes for operations
- Click "Start Cleaning" (won't actually delete in safe mode)

### 4. Check Settings
- Click "Settings" in the sidebar
- Try switching the theme (sun/moon icon)
- Change language preference

### 5. Toggle Dark Mode
- Click the sun/moon icon in top-right
- Watch the entire UI switch themes

## 📁 Project Structure

```
z-cleaner-2/
├── src/                    # React frontend
│   ├── components/         # 5 main pages
│   └── App.tsx            # Main app
├── src-tauri/             # Rust backend
│   └── src/
│       ├── main.rs        # Commands
│       └── commands/      # Logic
└── docs/                  # Documentation
```

## 🔧 Common Commands

```bash
# Start development server
npm run dev

# Build for production
npm run build

# Check TypeScript errors
npm run type-check

# Clean Rust build
cd src-tauri && cargo clean && cd ..
```

## 🐛 Troubleshooting

### App won't start?
```bash
# Clear cache and reinstall
rm -rf node_modules package-lock.json
npm install
npm run dev
```

### Port 5173 already in use?
```bash
# Kill the process
lsof -ti:5173 | xargs kill -9

# Or use a different port
# Edit vite.config.ts and change the port
```

### Rust compilation error?
```bash
# Update Rust
rustup update

# Clean build
cd src-tauri
cargo clean
cd ..

# Try again
npm run dev
```

## 📚 Documentation

| Document | Purpose |
|----------|---------|
| `README.md` | Full documentation |
| `QUICKSTART.md` | Detailed setup guide |
| `ARCHITECTURE.md` | System design |
| `PROJECT_SUMMARY.md` | Feature overview |
| `SETUP_CHECKLIST.md` | Verification steps |

## 🎨 UI Features to Explore

### Dashboard
- Real-time disk usage
- System health status
- Quick action buttons

### Analyzer
- Large files detection
- Disk space analysis
- File listing

### Cleaner
- Multiple cleaning options
- Operation history
- Space freed tracking

### Optimizer
- Startup programs list
- Enable/disable programs
- Performance tips

### Settings
- Theme selection
- Language preferences
- Auto-scan options

## 🔑 Key Technologies

- **Rust**: Fast, safe backend
- **React**: Modern UI framework
- **Tauri**: Desktop app framework
- **TailwindCSS**: Beautiful styling
- **TypeScript**: Type-safe code

## 💡 Tips

1. **Hot Reload**: React changes auto-reload (Rust changes need restart)
2. **DevTools**: Press `Cmd+Option+I` (macOS) to open browser console
3. **Dark Mode**: Automatically matches system preference
4. **Responsive**: Works on all screen sizes

## 🚀 Next Steps

### To Learn More
1. Read `README.md` for full documentation
2. Check `ARCHITECTURE.md` for system design
3. Review `PROJECT_SUMMARY.md` for features

### To Modify the Code
1. Frontend: Edit files in `src/components/`
2. Backend: Edit files in `src-tauri/src/commands/`
3. Styling: Edit `src/index.css` or `tailwind.config.js`

### To Build for Distribution
1. Run: `npm run build`
2. Find installers in: `src-tauri/target/release/bundle/`
3. Test on clean system before distribution

## ✅ Verification Checklist

After running `npm run dev`, verify:

- [ ] App window opens
- [ ] Sidebar visible with 5 tabs
- [ ] Dashboard shows disk stats
- [ ] Theme toggle works (sun/moon icon)
- [ ] Navigation between tabs works
- [ ] No errors in browser console
- [ ] No errors in terminal

## 🎓 Learning Path

1. **Beginner**: Just run the app and explore
2. **Intermediate**: Read the documentation
3. **Advanced**: Modify components and add features
4. **Expert**: Extend the backend with new commands

## 📞 Need Help?

1. Check the troubleshooting section above
2. Read the relevant documentation file
3. Check browser console for errors (DevTools)
4. Check terminal output for Rust errors

## 🎉 You're Ready!

Everything is set up and ready to go. Just run:

```bash
npm run dev
```

Enjoy Z-Cleaner! 🚀

---

**Pro Tip**: Keep this file open while developing. It has all the quick commands you'll need!
