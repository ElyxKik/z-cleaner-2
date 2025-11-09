# Z-Cleaner Setup Checklist

## ✅ Pre-Installation Requirements

- [ ] Node.js 16+ installed (`node --version`)
- [ ] npm 7+ installed (`npm --version`)
- [ ] Rust 1.70+ installed (`rustc --version`)
- [ ] Cargo installed (`cargo --version`)
- [ ] Git installed (`git --version`)
- [ ] macOS 10.13+ or Windows 7+ (for target OS)

## 📦 Installation Steps

### Step 1: Install Node Dependencies
```bash
cd /Users/elykik/Documents/Dev/z-cleaner-2
npm install
```

**Expected output:**
- ✅ No errors
- ✅ `node_modules/` folder created
- ✅ `package-lock.json` generated

**Verify:**
```bash
npm list react react-dom tauri
```

### Step 2: Verify Project Structure
```bash
# Check key files exist
ls -la src/
ls -la src-tauri/
ls -la src-tauri/src/
```

**Expected files:**
- ✅ `src/main.tsx`
- ✅ `src/App.tsx`
- ✅ `src/components/`
- ✅ `src-tauri/src/main.rs`
- ✅ `src-tauri/Cargo.toml`

### Step 3: Install Tauri CLI (if needed)
```bash
npm install -g @tauri-apps/cli@latest
```

**Verify:**
```bash
tauri --version
```

## 🚀 Running the Application

### Development Mode
```bash
npm run dev
```

**Expected behavior:**
- ✅ Vite dev server starts on port 5173
- ✅ Rust compilation begins
- ✅ Tauri window opens after ~30-60 seconds
- ✅ React app loads in window
- ✅ Dashboard displays with mock data

**Troubleshooting:**
- If window doesn't open: Check terminal for errors
- If port 5173 is in use: Kill process or change port in vite.config.ts
- If Rust compilation fails: Run `cargo clean` in `src-tauri/`

### Building for Production
```bash
npm run build
```

**Expected output:**
- ✅ React build completes
- ✅ Rust compilation succeeds
- ✅ Platform-specific installer created
  - macOS: `src-tauri/target/release/bundle/dmg/Z-Cleaner_*.dmg`
  - Windows: `src-tauri/target/release/bundle/msi/Z-Cleaner_*.msi`

**Build time:** 5-15 minutes (first build is slower)

## 🔍 Verification Tests

### Test 1: Frontend Loads
- [ ] App window opens
- [ ] Sidebar visible with navigation
- [ ] Dashboard tab active
- [ ] Dark theme applied

### Test 2: Navigation Works
- [ ] Click "Analyzer" → Analyzer page loads
- [ ] Click "Cleaner" → Cleaner page loads
- [ ] Click "Optimizer" → Optimizer page loads
- [ ] Click "Settings" → Settings page loads

### Test 3: Theme Toggle
- [ ] Click sun/moon icon in top-right
- [ ] Theme switches between light and dark
- [ ] Colors update correctly

### Test 4: Dashboard Functions
- [ ] Click "Refresh" button
- [ ] Disk stats load
- [ ] Progress bars display
- [ ] No errors in console

### Test 5: Analyzer Functions
- [ ] Click "Rescan" button
- [ ] Large files list loads
- [ ] File sizes display correctly
- [ ] Table is sortable

### Test 6: Cleaner Functions
- [ ] Checkboxes work
- [ ] "Start Cleaning" button clickable
- [ ] Operations execute (check console)
- [ ] Results display in history

### Test 7: Optimizer Functions
- [ ] Startup programs load
- [ ] Disable buttons work
- [ ] Tips display correctly

### Test 8: Settings Functions
- [ ] Theme dropdown works
- [ ] Language dropdown works
- [ ] Checkboxes toggle
- [ ] Save button works

## 🐛 Debugging Checklist

### If App Won't Start

1. **Check Node installation:**
   ```bash
   node --version  # Should be 16+
   npm --version   # Should be 7+
   ```

2. **Clear cache and reinstall:**
   ```bash
   rm -rf node_modules package-lock.json
   npm install
   ```

3. **Check Rust installation:**
   ```bash
   rustc --version  # Should be 1.70+
   cargo --version
   ```

4. **Update Rust:**
   ```bash
   rustup update
   ```

### If Compilation Fails

1. **Clean Rust build:**
   ```bash
   cd src-tauri
   cargo clean
   cd ..
   ```

2. **Check for syntax errors:**
   ```bash
   npm run type-check
   ```

3. **View detailed error:**
   ```bash
   RUST_LOG=debug npm run dev
   ```

### If UI Doesn't Load

1. **Check browser console:**
   - Open DevTools: `Cmd+Option+I` (macOS)
   - Look for JavaScript errors

2. **Check Tauri console:**
   - Look at terminal output
   - Check for Rust errors

3. **Verify Tauri API:**
   ```bash
   npm list @tauri-apps/api
   ```

### If Commands Don't Work

1. **Check Tauri bridge:**
   - Open DevTools console
   - Type: `window.__TAURI__` (should exist)

2. **Check command names:**
   - Verify in `src-tauri/src/main.rs`
   - Match exactly in React components

3. **Check error handling:**
   - Look for error messages in console
   - Check Rust error logs

## 📋 Pre-Deployment Checklist

### Code Quality
- [ ] No TypeScript errors: `npm run type-check`
- [ ] No console errors in DevTools
- [ ] All features tested manually
- [ ] No hardcoded paths or secrets

### Performance
- [ ] App starts in <2 seconds
- [ ] UI is responsive
- [ ] No memory leaks
- [ ] Animations are smooth

### Security
- [ ] No external API calls
- [ ] No data collection
- [ ] File operations validated
- [ ] No sensitive data in logs

### Documentation
- [ ] README.md updated
- [ ] Code comments added
- [ ] API documented
- [ ] Build instructions clear

### Build Artifacts
- [ ] macOS .dmg created
- [ ] Windows .msi created
- [ ] File sizes reasonable
- [ ] Installers tested

## 🚀 Deployment Steps

### macOS Distribution
1. [ ] Sign code: Configure certificates in `tauri.conf.json`
2. [ ] Create DMG: `npm run build`
3. [ ] Notarize: Submit to Apple for notarization
4. [ ] Test on clean macOS system
5. [ ] Upload to distribution platform

### Windows Distribution
1. [ ] Sign code: Configure certificate
2. [ ] Create MSI: `npm run build`
3. [ ] Test on clean Windows system
4. [ ] Upload to distribution platform

## 📊 Performance Benchmarks

| Metric | Target | Actual |
|--------|--------|--------|
| Startup time | <2s | - |
| Memory usage | <200MB | - |
| Disk scan (1GB) | <5s | - |
| UI responsiveness | 60 FPS | - |
| Bundle size | <60MB | - |

## 🎓 Learning Resources

- [Tauri Getting Started](https://tauri.app/docs/getting-started/)
- [React Documentation](https://react.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [TailwindCSS Docs](https://tailwindcss.com/docs)

## 📞 Support Resources

- **Tauri Discord**: https://discord.gg/tauri
- **React Community**: https://react.dev/community
- **Rust Community**: https://www.rust-lang.org/community
- **GitHub Issues**: Create issue with error details

## ✨ Success Indicators

You'll know everything is working when:

1. ✅ `npm run dev` launches the app in <60 seconds
2. ✅ Dashboard displays disk statistics
3. ✅ All navigation tabs work
4. ✅ Theme toggle switches colors
5. ✅ No errors in browser console
6. ✅ No errors in terminal output
7. ✅ `npm run build` completes successfully
8. ✅ Installer file is created

---

**If all checks pass, Z-Cleaner is ready to use! 🎉**

For issues, refer to the troubleshooting section or check the documentation files.
