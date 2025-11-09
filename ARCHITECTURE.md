# Z-Cleaner Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Z-Cleaner Desktop App                 │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌──────────────────────────────────────────────────┐   │
│  │          React Frontend (TypeScript)             │   │
│  │  ┌────────────────────────────────────────────┐  │   │
│  │  │  App.tsx (Main Router & Theme Manager)     │  │   │
│  │  ├────────────────────────────────────────────┤  │   │
│  │  │ Components:                                │  │   │
│  │  │ • Dashboard    (Stats & Overview)          │  │   │
│  │  │ • Cleaner      (Cleaning Operations)       │  │   │
│  │  │ • Analyzer     (Disk Analysis)             │  │   │
│  │  │ • Optimizer    (Startup Programs)          │  │   │
│  │  │ • Settings     (Preferences)               │  │   │
│  │  └────────────────────────────────────────────┘  │   │
│  │                                                    │   │
│  │  Styling: TailwindCSS + Custom Animations        │   │
│  │  Icons: Lucide React                             │   │
│  │  Animations: Framer Motion                       │   │
│  └──────────────────────────────────────────────────┘   │
│                          ↕ (IPC)                        │
│  ┌──────────────────────────────────────────────────┐   │
│  │         Tauri Bridge & Command Handler           │   │
│  │  • invoke() calls Rust commands                  │   │
│  │  • Serialization/Deserialization (serde)        │   │
│  │  • Error handling & type safety                 │   │
│  └──────────────────────────────────────────────────┘   │
│                          ↕                              │
│  ┌──────────────────────────────────────────────────┐   │
│  │       Rust Backend (Tauri + Tokio)              │   │
│  │  ┌────────────────────────────────────────────┐  │   │
│  │  │  main.rs (Command Handlers)                │  │   │
│  │  │  • analyze_disk_cmd()                      │  │   │
│  │  │  • find_large_files_cmd()                  │  │   │
│  │  │  • clean_*_cmd() (4 variants)              │  │   │
│  │  │  • get_startup_programs_cmd()              │  │   │
│  │  │  • disable_startup_program_cmd()           │  │   │
│  │  │  • get_system_performance_cmd()            │  │   │
│  │  └────────────────────────────────────────────┘  │   │
│  │                                                    │   │
│  │  ┌────────────────────────────────────────────┐  │   │
│  │  │  models.rs (Data Structures)               │  │   │
│  │  │  • DiskStats                               │  │   │
│  │  │  • LargeFile                               │  │   │
│  │  │  • CleanResult                             │  │   │
│  │  │  • StartupProgram                          │  │   │
│  │  │  • AppConfig                               │  │   │
│  │  └────────────────────────────────────────────┘  │   │
│  │                                                    │   │
│  │  ┌────────────────────────────────────────────┐  │   │
│  │  │  commands/ (Business Logic)                │  │   │
│  │  │                                            │  │   │
│  │  │  analyzer.rs:                              │  │   │
│  │  │  • analyze_disk()                          │  │   │
│  │  │  • find_large_files()                      │  │   │
│  │  │  • get_disk_info_macos()                   │  │   │
│  │  │  • get_disk_info_windows()                 │  │   │
│  │  │                                            │  │   │
│  │  │  cleaner.rs:                               │  │   │
│  │  │  • clean_temp_files()                      │  │   │
│  │  │  • clean_browser_cache()                   │  │   │
│  │  │  • clean_log_files()                       │  │   │
│  │  │  • clean_app_cache()                       │  │   │
│  │  │  • get_*_directories() (helpers)           │  │   │
│  │  │                                            │  │   │
│  │  │  optimizer.rs:                             │  │   │
│  │  │  • get_startup_programs()                  │  │   │
│  │  │  • disable_startup_program()               │  │   │
│  │  │  • get_system_performance()                │  │   │
│  │  └────────────────────────────────────────────┘  │   │
│  │                                                    │   │
│  │  Dependencies: Tokio, WalkDir, Serde, Chrono     │   │
│  └──────────────────────────────────────────────────┘   │
│                          ↕                              │
│  ┌──────────────────────────────────────────────────┐   │
│  │         System APIs & File System                │   │
│  │  • File I/O operations                           │   │
│  │  • Directory traversal                           │   │
│  │  • Disk space queries                            │   │
│  │  • Registry access (Windows)                     │   │
│  │  • LaunchAgents (macOS)                          │   │
│  └──────────────────────────────────────────────────┘   │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

## Data Flow

### 1. User Interaction Flow

```
User Action (Click Button)
    ↓
React Component Event Handler
    ↓
invoke('command_name', { params })
    ↓
Tauri IPC Bridge
    ↓
Rust Command Handler
    ↓
Business Logic (analyzer/cleaner/optimizer)
    ↓
System API Calls
    ↓
Result Serialization (serde)
    ↓
Tauri IPC Bridge
    ↓
Promise Resolution in React
    ↓
State Update & UI Re-render
```

### 2. Disk Analysis Flow

```
Dashboard Component
    ↓
useEffect() → invoke('analyze_disk_cmd')
    ↓
Rust: analyze_disk()
    ├─ Get home directory
    ├─ Walk directory tree (max depth 4)
    ├─ Calculate total, used, free space
    ├─ Count large files (>100MB)
    └─ Return DiskStats
    ↓
React: setStats(result)
    ↓
Render statistics cards with progress bars
```

### 3. Cleaning Flow

```
Cleaner Component
    ↓
User selects operations & clicks "Clean"
    ↓
For each selected operation:
    ├─ invoke('clean_temp_files_cmd')
    ├─ invoke('clean_browser_cache_cmd')
    ├─ invoke('clean_log_files_cmd')
    └─ invoke('clean_app_cache_cmd')
    ↓
Rust: clean_*_files()
    ├─ Get target directories
    ├─ Iterate through files
    ├─ Calculate space freed
    ├─ Delete files/directories
    └─ Return CleanResult
    ↓
React: Add result to history
    ↓
Display success message with stats
```

## Module Responsibilities

### Frontend Modules

| Module | Responsibility |
|--------|-----------------|
| `App.tsx` | Main routing, theme management, sidebar navigation |
| `Dashboard.tsx` | System statistics, disk usage, quick actions |
| `Cleaner.tsx` | Cleaning operation UI, history display |
| `Analyzer.tsx` | Large file listing, disk analysis display |
| `Optimizer.tsx` | Startup programs management |
| `Settings.tsx` | User preferences, configuration |

### Backend Modules

| Module | Responsibility |
|--------|-----------------|
| `main.rs` | Tauri command handlers, IPC entry points |
| `models.rs` | Data structures, serialization |
| `analyzer.rs` | Disk scanning, file analysis, space calculation |
| `cleaner.rs` | File deletion, cache clearing, log removal |
| `optimizer.rs` | Startup programs, performance monitoring |

## State Management

### Frontend State

```typescript
// App.tsx
- currentPage: 'dashboard' | 'cleaner' | 'analyzer' | 'optimizer' | 'settings'
- isDark: boolean

// Component-level state
- Dashboard: stats, loading, error
- Cleaner: results[], loading, selectedOps
- Analyzer: files[], loading
- Optimizer: programs[], loading
- Settings: settings, saved
```

### Backend State

```rust
// No persistent state (stateless design)
// Each command is independent and self-contained
// Results are computed fresh on each invocation
```

## Communication Protocol

### Tauri IPC Commands

```typescript
// Format: invoke('command_name', { param1, param2 })

// Analyzer Commands
invoke('analyze_disk_cmd')
invoke('find_large_files_cmd', { limit: 50 })

// Cleaner Commands
invoke('clean_temp_files_cmd')
invoke('clean_browser_cache_cmd')
invoke('clean_log_files_cmd')
invoke('clean_app_cache_cmd')

// Optimizer Commands
invoke('get_startup_programs_cmd')
invoke('disable_startup_program_cmd', { name: 'Program' })
invoke('get_system_performance_cmd')
```

### Response Types

```typescript
// Analyzer
DiskStats {
  total_size: u64
  used_size: u64
  free_size: u64
  percentage_used: f64
  large_files_count: u32
  large_files_size: u64
}

LargeFile {
  path: string
  size: u64
  size_mb: f64
  modified: DateTime
}

// Cleaner
CleanResult {
  id: string
  operation: string
  files_deleted: u32
  space_freed: u64
  timestamp: DateTime
  status: string
}

// Optimizer
StartupProgram {
  name: string
  path: string
  enabled: boolean
  size: u64
}
```

## Error Handling

### Frontend Error Handling

```typescript
try {
  const result = await invoke<T>('command_name')
  setData(result)
  setError(null)
} catch (err) {
  setError(err instanceof Error ? err.message : 'Unknown error')
  console.error('Error:', err)
}
```

### Backend Error Handling

```rust
pub async fn command() -> Result<T, String> {
  operation()
    .map_err(|e| e.to_string())?
  Ok(result)
}
```

## Performance Optimizations

### Frontend
- React.memo for components
- useCallback for event handlers
- Lazy loading with Suspense (ready for implementation)
- CSS animations instead of JS
- Optimized re-renders

### Backend
- Tokio async runtime for non-blocking I/O
- WalkDir for efficient directory traversal
- Early termination in loops
- Minimal allocations
- Efficient string operations

## Security Considerations

### File System Access
- ✅ Validate paths before operations
- ✅ Restrict to user directories
- ✅ No access to system-critical files
- ✅ Safe deletion with confirmation

### Data Privacy
- ✅ No external API calls
- ✅ No data collection
- ✅ Local processing only
- ✅ No telemetry

### Code Security
- ✅ Type-safe Rust
- ✅ Memory-safe operations
- ✅ No unsafe blocks (except where necessary)
- ✅ Input validation

## Extensibility

### Adding a New Feature

1. **Define Data Model** in `models.rs`
   ```rust
   #[derive(Serialize, Deserialize)]
   pub struct NewFeatureData { ... }
   ```

2. **Create Command Module** in `commands/`
   ```rust
   pub async fn new_feature() -> Result<NewFeatureData, String> { ... }
   ```

3. **Add Tauri Handler** in `main.rs`
   ```rust
   #[tauri::command]
   async fn new_feature_cmd() -> Result<NewFeatureData, String> { ... }
   ```

4. **Create React Component** in `components/`
   ```typescript
   const NewFeature = () => {
     const [data, setData] = useState(null)
     useEffect(() => {
       invoke('new_feature_cmd').then(setData)
     }, [])
     return <div>...</div>
   }
   ```

5. **Add Navigation** in `App.tsx`
   ```typescript
   { id: 'feature', label: 'Feature', icon: '🎯' }
   ```

## Testing Strategy

### Unit Tests (Rust)
```bash
cargo test
```

### Component Tests (React)
```bash
npm test
```

### Integration Tests
- Manual testing with `npm run dev`
- Test all command flows
- Verify UI updates

### Performance Tests
- Measure startup time
- Profile memory usage
- Monitor disk scan performance

## Deployment

### macOS
```bash
npm run build
# Creates: Z-Cleaner.dmg
```

### Windows
```bash
npm run build
# Creates: Z-Cleaner.msi
```

### Code Signing
- Configure in `tauri.conf.json`
- Set up certificates
- Sign before distribution

## Monitoring & Logging

### Development
```bash
RUST_LOG=debug npm run dev
```

### Production
- Log to file in app data directory
- Rotate logs automatically
- No sensitive data in logs

---

**This architecture ensures scalability, maintainability, and security.**
