import { motion } from 'framer-motion'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { useState, useEffect } from 'react'
import { useTranslation } from '../hooks/useTranslation'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faMagnifyingGlass, faBolt, faCheckCircle, faTrash, faExclamationCircle } from '@fortawesome/free-solid-svg-icons'

interface CleanResult {
  id: string
  operation: string
  files_deleted: number
  space_freed: number
  timestamp: string
  status: string
}

interface CleanScanResult {
  category: string
  files_count: number
  space_to_free: number
  description: string
  icon: string
}

interface CleanableFile {
  path: string
  size: number
  category: string
  modified: string
}

interface CleanScanResults {
  total_files: number
  total_space: number
  categories: CleanScanResult[]
  files: CleanableFile[]
}

interface ScanProgress {
  category: string
  status: 'scanning' | 'done'
  files?: number
  space?: number
}

export default function Cleaner() {
  const { t } = useTranslation()
  const [results, setResults] = useState<CleanResult[]>([])
  const [scanResults, setScanResults] = useState<CleanScanResults | null>(null)
  const [loading, setLoading] = useState(false)
  const [scanning, setScanning] = useState(false)
  const [error, setError] = useState<string>('')
  const [scanProgress, setScanProgress] = useState<Record<string, ScanProgress>>({})
  const [showDetails, setShowDetails] = useState(false)
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null)
  const [selectedOps, setSelectedOps] = useState({
    temp: true,
    browser: true,
    logs: true,
    cache: true,
  })

  // Setup event listener for scan progress
  useEffect(() => {
    let unlisten: (() => void) | null = null

    const setupListener = async () => {
      unlisten = await listen<ScanProgress>('clean-scan-progress', (event) => {
        setScanProgress((prev) => ({
          ...prev,
          [event.payload.category]: event.payload,
        }))
      })
    }

    setupListener()

    return () => {
      if (unlisten) {
        unlisten()
      }
    }
  }, [])

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i]
  }

  const handleScan = async () => {
    setScanning(true)
    setError('')
    try {
      console.log('Starting scan for cleaning...')
      const result = await invoke<CleanScanResults>('scan_for_cleaning_cmd')
      console.log('Scan results:', result)
      setScanResults(result)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      console.error('Scan error:', errorMsg)
      setError(`Scan error: ${errorMsg}`)
    } finally {
      setScanning(false)
    }
  }

  const handleClean = async (operation: string) => {
    setError('')
    try {
      console.log(`Starting clean operation: ${operation}`)
      let result: CleanResult
      switch (operation) {
        case 'temp':
          result = await invoke<CleanResult>('clean_temp_files_cmd')
          break
        case 'browser':
          result = await invoke<CleanResult>('clean_browser_cache_cmd')
          break
        case 'logs':
          result = await invoke<CleanResult>('clean_log_files_cmd')
          break
        case 'cache':
          result = await invoke<CleanResult>('clean_app_cache_cmd')
          break
        default:
          setError(`Unknown operation: ${operation}`)
          return
      }
      console.log(`Clean operation completed:`, result)
      setResults((prevResults) => [result, ...prevResults])
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      console.error('Clean error:', errorMsg)
      setError(`Error: ${errorMsg}`)
    }
  }

  const cleanAll = async () => {
    setLoading(true)
    setError('')
    
    const ops = Object.entries(selectedOps)
      .filter(([_, selected]) => selected)
      .map(([op, _]) => op)

    console.log('Starting cleaning for operations:', ops)
    
    if (ops.length === 0) {
      setError('Please select at least one category to clean')
      setLoading(false)
      return
    }

    for (const op of ops) {
      console.log(`Cleaning ${op}...`)
      await handleClean(op)
    }
    
    setLoading(false)
  }

  return (
    <div className="p-8 space-y-8 animate-fade-in">
      <div>
        <h1 className="text-3xl font-bold text-slate-900 dark:text-white">{t('cleaner.title')}</h1>
        <p className="text-slate-600 dark:text-slate-400 mt-2">{t('cleaner.subtitle')}</p>
      </div>

      {/* Error Message */}
      {error && (
        <motion.div
          initial={{ opacity: 0, y: -10 }}
          animate={{ opacity: 1, y: 0 }}
          className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4 flex items-start gap-3"
        >
          <FontAwesomeIcon icon={faExclamationCircle} className="text-red-500 flex-shrink-0 mt-0.5" size="lg" />
          <div>
            <p className="font-medium text-red-900 dark:text-red-200">{error}</p>
          </div>
        </motion.div>
      )}

      {/* Scanning Progress */}
      {scanning && (
        <motion.div
          initial={{ opacity: 0, y: -10 }}
          animate={{ opacity: 1, y: 0 }}
          exit={{ opacity: 0, y: -10 }}
          className="bg-gradient-to-r from-blue-50 to-purple-50 dark:from-slate-800 dark:to-slate-700 rounded-xl p-8 border border-blue-200 dark:border-slate-600"
        >
          <div className="space-y-6">
            <div className="flex items-center gap-3">
              <motion.div
                animate={{ rotate: 360 }}
                transition={{ duration: 2, repeat: Infinity, ease: 'linear' }}
              >
                <FontAwesomeIcon icon={faBolt} className="text-blue-500" size="2x" />
              </motion.div>
              <div>
                <h3 className="text-lg font-semibold text-slate-900 dark:text-white">Scanning your computer...</h3>
                <p className="text-sm text-slate-600 dark:text-slate-400">Deep scan in progress</p>
              </div>
            </div>

            <div className="space-y-4">
              {['temp', 'browser', 'logs', 'cache'].map((category) => {
                const progress = scanProgress[category]
                const categoryNames = {
                  temp: '🗑️ Temporary Files',
                  browser: '🌐 Browser Cache',
                  logs: '📝 Log Files',
                  cache: '💾 App Cache',
                }
                return (
                  <div key={category} className="space-y-2">
                    <div className="flex justify-between items-center">
                      <span className="text-sm font-medium text-slate-700 dark:text-slate-300">
                        {categoryNames[category as keyof typeof categoryNames]}
                      </span>
                      {progress?.status === 'done' ? (
                        <span className="text-sm font-bold text-green-600 dark:text-green-400">
                          {progress.files} files • {formatBytes(progress.space || 0)}
                        </span>
                      ) : progress?.status === 'scanning' ? (
                        <span className="text-sm font-bold text-blue-600 dark:text-blue-400">Scanning...</span>
                      ) : (
                        <span className="text-sm font-bold text-slate-400">Pending...</span>
                      )}
                    </div>
                    <motion.div
                      className="h-2 bg-slate-200 dark:bg-slate-600 rounded-full overflow-hidden"
                      initial={{ opacity: 0 }}
                      animate={{ opacity: 1 }}
                    >
                      <motion.div
                        className={`h-full ${
                          progress?.status === 'done'
                            ? 'bg-gradient-to-r from-green-500 to-emerald-500'
                            : 'bg-gradient-to-r from-blue-500 to-purple-500'
                        }`}
                        animate={{
                          width: progress?.status === 'done' ? '100%' : '60%',
                        }}
                        transition={{ duration: 0.5 }}
                      />
                    </motion.div>
                  </div>
                )
              })}
            </div>

            <div className="flex gap-2 pt-2">
              {[0, 1, 2].map((i) => (
                <motion.div
                  key={i}
                  className="h-1 flex-1 bg-gradient-to-r from-blue-400 to-purple-400 rounded-full"
                  animate={{ opacity: [0.3, 1, 0.3] }}
                  transition={{ duration: 1.5, delay: i * 0.2, repeat: Infinity }}
                />
              ))}
            </div>
          </div>
        </motion.div>
      )}

      {/* Scan or Results */}
      {!scanning && !scanResults && (
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700"
        >
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-4">{t('cleaner.scanYourComputer')}</h3>
          <p className="text-slate-600 dark:text-slate-400 mb-6">
            {t('cleaner.scanDescription')}
          </p>
          <button
            onClick={handleScan}
            disabled={scanning}
            className="w-full py-3 bg-gradient-to-r from-blue-500 to-purple-500 text-white rounded-lg font-semibold hover:shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
          >
            <FontAwesomeIcon icon={faMagnifyingGlass} />
            {t('cleaner.scanNow')}
          </button>
        </motion.div>
      )}

      {/* Results */}
      {!scanning && scanResults && (
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700"
        >
          <div className="flex justify-between items-center mb-6">
            <div>
              <h3 className="text-lg font-semibold text-slate-900 dark:text-white">Scan Results</h3>
              <p className="text-sm text-slate-600 dark:text-slate-400 mt-1">
                Found {scanResults.total_files} files • {formatBytes(scanResults.total_space)} to free
              </p>
            </div>
            <button
              onClick={handleScan}
              disabled={scanning}
              className="px-4 py-2 text-sm bg-slate-200 dark:bg-slate-700 text-slate-900 dark:text-white rounded-lg hover:bg-slate-300 dark:hover:bg-slate-600 transition-colors"
            >
              Re-scan
            </button>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
            {scanResults.categories.map((category) => (
              <motion.label
                key={category.category}
                initial={{ opacity: 0, scale: 0.95 }}
                animate={{ opacity: 1, scale: 1 }}
                className="flex items-center p-4 border border-slate-200 dark:border-slate-700 rounded-lg cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors"
              >
                <input
                  type="checkbox"
                  checked={selectedOps[category.category as keyof typeof selectedOps]}
                  onChange={(e) =>
                    setSelectedOps({
                      ...selectedOps,
                      [category.category]: e.target.checked,
                    })
                  }
                  className="w-4 h-4"
                />
                <div className="ml-4 flex-1">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <span className="text-xl">{category.icon}</span>
                      <div>
                        <p className="font-medium text-slate-900 dark:text-white capitalize">
                          {category.category.replace(/_/g, ' ')}
                        </p>
                        <p className="text-xs text-slate-500 dark:text-slate-400">
                          {category.files_count} files • {formatBytes(category.space_to_free)}
                        </p>
                      </div>
                    </div>
                    <button
                      onClick={(e) => {
                        e.preventDefault()
                        setSelectedCategory(category.category)
                        setShowDetails(true)
                      }}
                      className="px-3 py-1 text-xs bg-blue-500 hover:bg-blue-600 text-white rounded transition-colors"
                    >
                      Détails
                    </button>
                  </div>
                  <p className="text-sm text-slate-500 dark:text-slate-400 mt-1">{category.description}</p>
                </div>
              </motion.label>
            ))}
          </div>

          <button
            onClick={cleanAll}
            disabled={loading}
            className="w-full py-3 bg-gradient-to-r from-red-500 to-orange-500 text-white rounded-lg font-semibold hover:shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
          >
            <FontAwesomeIcon icon={faTrash} size="lg" />
            {loading ? 'Cleaning...' : 'Start Cleaning'}
          </button>
        </motion.div>
      )}

      {/* Results */}
      {results.length > 0 && (
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700"
        >
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-4">Cleaning History</h3>
          <div className="space-y-3">
            {results.map((result) => {
              console.log('Rendering result:', result)
              const timestamp = typeof result.timestamp === 'string' 
                ? new Date(result.timestamp) 
                : new Date()
              
              return (
                <motion.div
                  key={result.id}
                  initial={{ opacity: 0, x: -20 }}
                  animate={{ opacity: 1, x: 0 }}
                  className="flex items-center justify-between p-4 bg-slate-50 dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-700"
                >
                  <div className="flex items-center gap-3">
                    <FontAwesomeIcon icon={faCheckCircle} className="text-green-500" size="lg" />
                    <div>
                      <p className="font-medium text-slate-900 dark:text-white capitalize">
                        {result.operation.replace(/_/g, ' ')}
                      </p>
                      <p className="text-sm text-slate-500 dark:text-slate-400">
                        {result.files_deleted} files • {formatBytes(result.space_freed)}
                      </p>
                    </div>
                  </div>
                  <span className="text-xs text-slate-500 dark:text-slate-400">
                    {timestamp.toLocaleTimeString()}
                  </span>
                </motion.div>
              )
            })}
          </div>
        </motion.div>
      )}

      {/* Details Modal */}
      {showDetails && selectedCategory && scanResults && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          onClick={() => setShowDetails(false)}
          className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
        >
          <motion.div
            initial={{ scale: 0.95, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            exit={{ scale: 0.95, opacity: 0 }}
            onClick={(e) => e.stopPropagation()}
            className="bg-white dark:bg-slate-900 rounded-xl shadow-2xl max-w-2xl w-full max-h-[80vh] overflow-hidden flex flex-col border border-slate-200 dark:border-slate-700"
          >
            {/* Header */}
            <div className="p-6 border-b border-slate-200 dark:border-slate-700 flex justify-between items-center">
              <h2 className="text-xl font-bold text-slate-900 dark:text-white capitalize">
                {selectedCategory.replace(/_/g, ' ')} - Fichiers détaillés
              </h2>
              <button
                onClick={() => setShowDetails(false)}
                className="text-slate-500 hover:text-slate-700 dark:hover:text-slate-300"
              >
                ✕
              </button>
            </div>

            {/* Files List */}
            <div className="flex-1 overflow-y-auto p-6 space-y-2">
              {scanResults.files
                .filter((file) => file.category === selectedCategory)
                .map((file, idx) => (
                  <motion.div
                    key={idx}
                    initial={{ opacity: 0, x: -10 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: idx * 0.01 }}
                    className="p-3 bg-slate-50 dark:bg-slate-800 rounded border border-slate-200 dark:border-slate-700 text-sm"
                  >
                    <p className="font-mono text-slate-700 dark:text-slate-300 break-all">
                      {file.path}
                    </p>
                    <p className="text-xs text-slate-500 dark:text-slate-400 mt-1">
                      {formatBytes(file.size)} • {new Date(file.modified).toLocaleDateString()}
                    </p>
                  </motion.div>
                ))}
            </div>

            {/* Footer */}
            <div className="p-6 border-t border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800">
              <button
                onClick={() => setShowDetails(false)}
                className="w-full px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-colors"
              >
                Fermer
              </button>
            </div>
          </motion.div>
        </motion.div>
      )}
    </div>
  )
}
