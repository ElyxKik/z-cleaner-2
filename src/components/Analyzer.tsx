import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { BarChart3, Zap, Trash2, AlertCircle, Shield, AlertTriangle, HardDrive, FileText, Cpu, Database } from 'lucide-react'
import { motion } from 'framer-motion'
import { useTranslation } from '../hooks/useTranslation'

interface LargeFile {
  path: string
  size: number
  size_mb: number
  modified: string
}

interface MalwareFile {
  path: string
  threat_level: string
  threat_type: string
  reason: string
  size: number
  hash?: string
  is_autostart: boolean
}

interface JunkFile {
  path: string
  size: number
  category: string
  description: string
  safe_to_delete: boolean
}

interface JunkCategory {
  name: string
  count: number
  total_size: number
}

interface JunkScanResult {
  total_junk_size: number
  junk_files: JunkFile[]
  categories: JunkCategory[]
}

interface DiskAnalysis {
  total_space: number
  used_space: number
  free_space: number
  usage_percentage: number
  partitions: PartitionInfo[]
  file_type_distribution: Record<string, FileTypeStats>
  largest_folders: FolderStats[]
  hidden_files_count: number
  hidden_files_size: number
}

interface PartitionInfo {
  name: string
  mount_point: string
  total_space: number
  free_space: number
  file_system: string
  disk_type: string
}

interface FileTypeStats {
  extension: string
  count: number
  total_size: number
  percentage: number
}

interface FolderStats {
  path: string
  size: number
  file_count: number
  subfolder_count: number
}

interface ScanProgress {
  filesScanned: number
  filesFound: number
}

interface MalwareScanProgress {
  filesScanned: number
  threatsFound: number
}

export default function Analyzer() {
  const { t } = useTranslation()
  const [activeTab, setActiveTab] = useState<'overview' | 'files' | 'junk' | 'malware' | 'autostart'>('overview')
  const [files, setFiles] = useState<LargeFile[]>([])
  const [malwareFiles, setMalwareFiles] = useState<MalwareFile[]>([])
  const [junkResult, setJunkResult] = useState<JunkScanResult | null>(null)
  const [diskAnalysis, setDiskAnalysis] = useState<DiskAnalysis | null>(null)
  const [autostartFiles, setAutostartFiles] = useState<MalwareFile[]>([])
  
  const [loading, setLoading] = useState(false)
  const [malwareLoading, setMalwareLoading] = useState(false)
  const [junkLoading, setJunkLoading] = useState(false)
  const [analysisLoading, setAnalysisLoading] = useState(false)
  const [autostartLoading, setAutostartLoading] = useState(false)
  
  const [hasScanned, setHasScanned] = useState(false)
  const [hasMalwareScan, setHasMalwareScan] = useState(false)
  const [hasJunkScan, setHasJunkScan] = useState(false)
  const [hasAnalysis, setHasAnalysis] = useState(false)
  const [hasAutostartScan, setHasAutostartScan] = useState(false)
  
  const [progress, setProgress] = useState<ScanProgress>({ filesScanned: 0, filesFound: 0 })
  const [malwareProgress, setMalwareProgress] = useState<MalwareScanProgress>({ filesScanned: 0, threatsFound: 0 })
  const [selectedFiles, setSelectedFiles] = useState<Set<string>>(new Set())
  const [selectedMalware, setSelectedMalware] = useState<Set<string>>(new Set())
  const [selectedJunk, setSelectedJunk] = useState<Set<string>>(new Set())
  const [selectedAutostart, setSelectedAutostart] = useState<Set<string>>(new Set())
  const [isDeleting, setIsDeleting] = useState(false)
  const [deleteMessage, setDeleteMessage] = useState<string>('')

  // Ne pas faire de scan automatique - l'utilisateur doit cliquer sur Scan

  useEffect(() => {
    let unlistenFiles: (() => void) | null = null
    let unlistenMalware: (() => void) | null = null

    const setupListeners = async () => {
      unlistenFiles = await listen<ScanProgress>('scan-progress', (event) => {
        setProgress(event.payload)
      })
      unlistenMalware = await listen<MalwareScanProgress>('malware-scan-progress', (event) => {
        setMalwareProgress(event.payload)
      })
    }

    setupListeners()

    return () => {
      if (unlistenFiles) {
        unlistenFiles()
      }
      if (unlistenMalware) {
        unlistenMalware()
      }
    }
  }, [])

  const loadLargeFiles = async () => {
    try {
      setLoading(true)
      setProgress({ filesScanned: 0, filesFound: 0 })
      const result = await invoke<LargeFile[]>('find_large_files_with_progress_cmd', { limit: 100 })
      setFiles(result)
      setHasScanned(true)
    } catch (err) {
      console.error('Error loading files:', err)
    } finally {
      setLoading(false)
    }
  }

  const toggleFileSelection = (path: string) => {
    const newSelected = new Set(selectedFiles)
    if (newSelected.has(path)) {
      newSelected.delete(path)
    } else {
      newSelected.add(path)
    }
    setSelectedFiles(newSelected)
  }

  const deleteSelectedFiles = async () => {
    if (selectedFiles.size === 0) return
    
    try {
      setIsDeleting(true)
      setDeleteMessage('')
      const filesToDelete = Array.from(selectedFiles)
      const result = await invoke<{ deleted: number; freed: number }>('delete_files_cmd', { 
        paths: filesToDelete 
      })
      
      setDeleteMessage(`✓ ${result.deleted} fichiers supprimés - ${(result.freed / (1024 * 1024 * 1024)).toFixed(2)} GB libérés`)
      setSelectedFiles(new Set())
      
      // Rafraîchir la liste
      await loadLargeFiles()
    } catch (err) {
      setDeleteMessage(`✗ Erreur: ${err}`)
      console.error('Error deleting files:', err)
    } finally {
      setIsDeleting(false)
    }
  }

  const selectAll = () => {
    if (selectedFiles.size === files.length) {
      setSelectedFiles(new Set())
    } else {
      setSelectedFiles(new Set(files.map(f => f.path)))
    }
  }

  const scanForMalware = async () => {
    try {
      setMalwareLoading(true)
      setDeleteMessage('')
      setMalwareProgress({ filesScanned: 0, threatsFound: 0 })
      const home = await invoke<string>('get_home_dir_cmd').catch(() => '/Users')
      const result = await invoke<MalwareFile[]>('scan_for_malware_with_progress_cmd', { 
        rootPath: home,
        limit: 100 
      })
      setMalwareFiles(result)
      setHasMalwareScan(true)
    } catch (err) {
      console.error('Error scanning for malware:', err)
      setDeleteMessage(`✗ Erreur lors du scan: ${err}`)
    } finally {
      setMalwareLoading(false)
    }
  }

  const toggleMalwareSelection = (path: string) => {
    const newSelected = new Set(selectedMalware)
    if (newSelected.has(path)) {
      newSelected.delete(path)
    } else {
      newSelected.add(path)
    }
    setSelectedMalware(newSelected)
  }

  const selectAllMalware = () => {
    if (selectedMalware.size === malwareFiles.length) {
      setSelectedMalware(new Set())
    } else {
      setSelectedMalware(new Set(malwareFiles.map(f => f.path)))
    }
  }

  const deleteQuarantinedFiles = async () => {
    if (selectedMalware.size === 0) return
    
    try {
      setIsDeleting(true)
      setDeleteMessage('')
      const filesToDelete = Array.from(selectedMalware)
      const result = await invoke<{ deleted: number; freed: number }>('delete_files_cmd', { 
        paths: filesToDelete 
      })
      
      setDeleteMessage(`✓ ${result.deleted} fichiers malveillants supprimés - ${(result.freed / (1024 * 1024 * 1024)).toFixed(2)} GB libérés`)
      setSelectedMalware(new Set())
      
      // Rafraîchir la liste
      await scanForMalware()
    } catch (err) {
      setDeleteMessage(`✗ Erreur: ${err}`)
      console.error('Error deleting malware:', err)
    } finally {
      setIsDeleting(false)
    }
  }

  const scanForJunk = async () => {
    try {
      setJunkLoading(true)
      setDeleteMessage('')
      const home = await invoke<string>('get_home_dir_cmd').catch(() => '/Users')
      const result = await invoke<JunkScanResult>('scan_for_junk_cmd', { 
        rootPath: home
      })
      setJunkResult(result)
      setHasJunkScan(true)
    } catch (err) {
      console.error('Error scanning for junk:', err)
      setDeleteMessage(`✗ Erreur lors du scan: ${err}`)
    } finally {
      setJunkLoading(false)
    }
  }

  const analyzeDiskAdvanced = async () => {
    try {
      setAnalysisLoading(true)
      setDeleteMessage('')
      const home = await invoke<string>('get_home_dir_cmd').catch(() => '/Users')
      const result = await invoke<DiskAnalysis>('analyze_disk_advanced_cmd', { 
        rootPath: home
      })
      setDiskAnalysis(result)
      setHasAnalysis(true)
    } catch (err) {
      console.error('Error analyzing disk:', err)
      setDeleteMessage(`✗ Erreur lors de l'analyse: ${err}`)
    } finally {
      setAnalysisLoading(false)
    }
  }

  const scanAutostart = async () => {
    try {
      setAutostartLoading(true)
      setDeleteMessage('')
      const result = await invoke<MalwareFile[]>('scan_autostart_entries_cmd')
      setAutostartFiles(result)
      setHasAutostartScan(true)
    } catch (err) {
      console.error('Error scanning autostart:', err)
      setDeleteMessage(`✗ Erreur lors du scan: ${err}`)
    } finally {
      setAutostartLoading(false)
    }
  }

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`
  }

  return (
    <div className="p-8 space-y-8 animate-fade-in">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-slate-900 dark:text-white">{t('analyzer.title')}</h1>
          <p className="text-slate-600 dark:text-slate-400 mt-2">{t('analyzer.subtitle')}</p>
        </div>
        <div className="flex gap-3">
          <motion.button
            onClick={loadLargeFiles}
            disabled={loading}
            whileHover={{ scale: loading ? 1 : 1.05 }}
            whileTap={{ scale: loading ? 1 : 0.95 }}
            className="px-6 py-3 bg-gradient-to-r from-blue-500 to-purple-500 text-white rounded-lg hover:shadow-lg transition-all disabled:opacity-50"
          >
            {loading ? t('analyzer.scanning') : hasScanned ? t('analyzer.rescanFiles') : t('analyzer.scanFiles')}
          </motion.button>
          <motion.button
            onClick={scanForMalware}
            disabled={malwareLoading}
            whileHover={{ scale: malwareLoading ? 1 : 1.05 }}
            whileTap={{ scale: malwareLoading ? 1 : 0.95 }}
            className="px-6 py-3 bg-gradient-to-r from-red-500 to-orange-500 text-white rounded-lg hover:shadow-lg transition-all disabled:opacity-50 flex items-center gap-2"
          >
            <Shield size={18} />
            {malwareLoading ? t('analyzer.scanning') : hasMalwareScan ? t('analyzer.rescanMalware') : t('analyzer.scanMalware')}
          </motion.button>
        </div>
      </div>

      <div className="flex gap-2 border-b border-slate-200 dark:border-slate-700">
        <button
          onClick={() => setActiveTab('files')}
          className={`px-4 py-2 font-medium transition-colors ${
            activeTab === 'files'
              ? 'text-blue-600 dark:text-blue-400 border-b-2 border-blue-600 dark:border-blue-400'
              : 'text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white'
          }`}
        >
          <BarChart3 className="inline mr-2" size={18} />
          Large Files
        </button>
        <button
          onClick={() => setActiveTab('malware')}
          className={`px-4 py-2 font-medium transition-colors ${
            activeTab === 'malware'
              ? 'text-red-600 dark:text-red-400 border-b-2 border-red-600 dark:border-red-400'
              : 'text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white'
          }`}
        >
          <AlertTriangle className="inline mr-2" size={18} />
          Malware ({malwareFiles.length})
        </button>
      </div>

      {loading && (
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
                <Zap className="text-blue-500" size={28} />
              </motion.div>
              <div>
                <h3 className="text-lg font-semibold text-slate-900 dark:text-white">Scanning your disk...</h3>
                <p className="text-sm text-slate-600 dark:text-slate-400">Finding large files</p>
              </div>
            </div>

            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <span className="text-sm font-medium text-slate-700 dark:text-slate-300">Files scanned</span>
                <span className="text-sm font-bold text-blue-600 dark:text-blue-400">{progress.filesScanned}</span>
              </div>
              <motion.div
                className="h-2 bg-slate-200 dark:bg-slate-600 rounded-full overflow-hidden"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
              >
                <motion.div
                  className="h-full bg-gradient-to-r from-blue-500 to-purple-500"
                  animate={{ width: `${Math.min((progress.filesScanned / 5000) * 100, 100)}%` }}
                  transition={{ duration: 0.3 }}
                />
              </motion.div>
            </div>

            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <span className="text-sm font-medium text-slate-700 dark:text-slate-300">Large files found</span>
                <span className="text-sm font-bold text-purple-600 dark:text-purple-400">{progress.filesFound}</span>
              </div>
              <motion.div
                className="h-2 bg-slate-200 dark:bg-slate-600 rounded-full overflow-hidden"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
              >
                <motion.div
                  className="h-full bg-gradient-to-r from-purple-500 to-pink-500"
                  animate={{ width: `${Math.min((progress.filesFound / 50) * 100, 100)}%` }}
                  transition={{ duration: 0.3 }}
                />
              </motion.div>
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

      {malwareLoading && (
        <motion.div
          initial={{ opacity: 0, y: -10 }}
          animate={{ opacity: 1, y: 0 }}
          exit={{ opacity: 0, y: -10 }}
          className="bg-gradient-to-r from-red-50 to-orange-50 dark:from-slate-800 dark:to-slate-700 rounded-xl p-8 border border-red-200 dark:border-slate-600"
        >
          <div className="space-y-6">
            <div className="flex items-center gap-3">
              <motion.div
                animate={{ rotate: 360 }}
                transition={{ duration: 2, repeat: Infinity, ease: 'linear' }}
              >
                <Shield className="text-red-500" size={28} />
              </motion.div>
              <div>
                <h3 className="text-lg font-semibold text-slate-900 dark:text-white">Scanning for malware...</h3>
                <p className="text-sm text-slate-600 dark:text-slate-400">Deep system scan in progress</p>
              </div>
            </div>

            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <span className="text-sm font-medium text-slate-700 dark:text-slate-300">Files scanned</span>
                <span className="text-sm font-bold text-red-600 dark:text-red-400">{malwareProgress.filesScanned}</span>
              </div>
              <motion.div
                className="h-2 bg-slate-200 dark:bg-slate-600 rounded-full overflow-hidden"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
              >
                <motion.div
                  className="h-full bg-gradient-to-r from-red-500 to-orange-500"
                  animate={{ width: `${Math.min((malwareProgress.filesScanned / 10000) * 100, 100)}%` }}
                  transition={{ duration: 0.3 }}
                />
              </motion.div>
            </div>

            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <span className="text-sm font-medium text-slate-700 dark:text-slate-300">Threats detected</span>
                <span className="text-sm font-bold text-orange-600 dark:text-orange-400">{malwareProgress.threatsFound}</span>
              </div>
              <motion.div
                className="h-2 bg-slate-200 dark:bg-slate-600 rounded-full overflow-hidden"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
              >
                <motion.div
                  className="h-full bg-gradient-to-r from-orange-500 to-red-500"
                  animate={{ width: `${Math.min((malwareProgress.threatsFound / 100) * 100, 100)}%` }}
                  transition={{ duration: 0.3 }}
                />
              </motion.div>
            </div>

            <div className="flex gap-2 pt-2">
              {[0, 1, 2].map((i) => (
                <motion.div
                  key={i}
                  className="h-1 flex-1 bg-gradient-to-r from-red-400 to-orange-400 rounded-full"
                  animate={{ opacity: [0.3, 1, 0.3] }}
                  transition={{ duration: 1.5, delay: i * 0.2, repeat: Infinity }}
                />
              ))}
            </div>
          </div>
        </motion.div>
      )}

      {activeTab === 'files' && (
      <div className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700">
        <div className="flex items-center gap-2 mb-4">
          <BarChart3 className="text-blue-500" size={24} />
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white">
            Top {files.length} Large Files
          </h3>
        </div>

        {!loading && files.length > 0 ? (
          <div className="space-y-4">
            {deleteMessage && (
              <motion.div
                initial={{ opacity: 0, y: -10 }}
                animate={{ opacity: 1, y: 0 }}
                className={`p-4 rounded-lg ${
                  deleteMessage.startsWith('✓')
                    ? 'bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800'
                    : 'bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800'
                }`}
              >
                <p className={deleteMessage.startsWith('✓') ? 'text-green-700 dark:text-green-400' : 'text-red-700 dark:text-red-400'}>
                  {deleteMessage}
                </p>
              </motion.div>
            )}

            <div className="flex items-center gap-3 p-4 bg-slate-50 dark:bg-slate-800 rounded-lg">
              <input
                type="checkbox"
                checked={selectedFiles.size === files.length && files.length > 0}
                onChange={selectAll}
                className="w-4 h-4 cursor-pointer"
              />
              <span className="text-sm font-medium text-slate-700 dark:text-slate-300">
                {selectedFiles.size > 0 ? `${selectedFiles.size} selected` : 'Select all'}
              </span>
              {selectedFiles.size > 0 && (
                <motion.button
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  onClick={deleteSelectedFiles}
                  disabled={isDeleting}
                  className="ml-auto px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg flex items-center gap-2 disabled:opacity-50 transition-colors"
                >
                  <Trash2 size={16} />
                  {isDeleting ? 'Deleting...' : 'Delete Selected'}
                </motion.button>
              )}
            </div>

            <div className="overflow-x-auto">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-slate-200 dark:border-slate-700">
                    <th className="text-left py-3 px-4 text-sm font-semibold text-slate-600 dark:text-slate-400 w-12">
                      <input type="checkbox" className="w-4 h-4" disabled />
                    </th>
                    <th className="text-left py-3 px-4 text-sm font-semibold text-slate-600 dark:text-slate-400">
                      File Path
                    </th>
                    <th className="text-right py-3 px-4 text-sm font-semibold text-slate-600 dark:text-slate-400">
                      Size
                    </th>
                    <th className="text-right py-3 px-4 text-sm font-semibold text-slate-600 dark:text-slate-400">
                      Modified
                    </th>
                  </tr>
                </thead>
                <tbody>
                  {files.map((file, idx) => (
                    <tr
                      key={idx}
                      className={`border-b border-slate-100 dark:border-slate-800 hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors ${
                        selectedFiles.has(file.path) ? 'bg-blue-50 dark:bg-blue-900/20' : ''
                      }`}
                    >
                      <td className="py-3 px-4">
                        <input
                          type="checkbox"
                          checked={selectedFiles.has(file.path)}
                          onChange={() => toggleFileSelection(file.path)}
                          className="w-4 h-4 cursor-pointer"
                        />
                      </td>
                      <td className="py-3 px-4 text-sm text-slate-700 dark:text-slate-300 truncate">
                        {file.path}
                      </td>
                      <td className="py-3 px-4 text-right text-sm font-medium text-slate-900 dark:text-white">
                        {file.size_mb.toFixed(2)} MB
                      </td>
                      <td className="py-3 px-4 text-right text-sm text-slate-500 dark:text-slate-400">
                        {new Date(file.modified).toLocaleDateString()}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        ) : !loading && hasScanned ? (
          <div className="text-center py-12">
            <AlertCircle className="mx-auto mb-3 text-slate-400" size={40} />
            <p className="text-slate-500 dark:text-slate-400">No large files found</p>
          </div>
        ) : (
          <div className="text-center py-12">
            <p className="text-slate-500 dark:text-slate-400">Click "Scan Files" to start analyzing your disk</p>
          </div>
        )}
      </div>
      )}

      {activeTab === 'malware' && (
      <div className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700">
        <div className="flex items-center gap-2 mb-4">
          <AlertTriangle className="text-red-500" size={24} />
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white">
            Detected Malware ({malwareFiles.length})
          </h3>
        </div>

        {!malwareLoading && malwareFiles.length > 0 ? (
          <div className="space-y-4">
            {deleteMessage && (
              <motion.div
                initial={{ opacity: 0, y: -10 }}
                animate={{ opacity: 1, y: 0 }}
                className={`p-4 rounded-lg ${
                  deleteMessage.startsWith('✓')
                    ? 'bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800'
                    : 'bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800'
                }`}
              >
                <p className={deleteMessage.startsWith('✓') ? 'text-green-700 dark:text-green-400' : 'text-red-700 dark:text-red-400'}>
                  {deleteMessage}
                </p>
              </motion.div>
            )}

            <div className="flex items-center gap-3 p-4 bg-slate-50 dark:bg-slate-800 rounded-lg">
              <input
                type="checkbox"
                checked={selectedMalware.size === malwareFiles.length && malwareFiles.length > 0}
                onChange={selectAllMalware}
                className="w-4 h-4 cursor-pointer"
              />
              <span className="text-sm font-medium text-slate-700 dark:text-slate-300">
                {selectedMalware.size > 0 ? `${selectedMalware.size} selected` : 'Select all'}
              </span>
              {selectedMalware.size > 0 && (
                <motion.button
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  onClick={deleteQuarantinedFiles}
                  disabled={isDeleting}
                  className="ml-auto px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg flex items-center gap-2 disabled:opacity-50 transition-colors"
                >
                  <Trash2 size={16} />
                  {isDeleting ? 'Quarantining...' : 'Quarantine & Delete'}
                </motion.button>
              )}
            </div>

            <div className="overflow-x-auto">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-slate-200 dark:border-slate-700">
                    <th className="text-left py-3 px-4 text-sm font-semibold text-slate-600 dark:text-slate-400 w-12">
                      <input type="checkbox" className="w-4 h-4" disabled />
                    </th>
                    <th className="text-left py-3 px-4 text-sm font-semibold text-slate-600 dark:text-slate-400">
                      File Path
                    </th>
                    <th className="text-left py-3 px-4 text-sm font-semibold text-slate-600 dark:text-slate-400">
                      Threat Type
                    </th>
                    <th className="text-left py-3 px-4 text-sm font-semibold text-slate-600 dark:text-slate-400">
                      Level
                    </th>
                    <th className="text-left py-3 px-4 text-sm font-semibold text-slate-600 dark:text-slate-400">
                      Reason
                    </th>
                  </tr>
                </thead>
                <tbody>
                  {malwareFiles.map((malware, idx) => (
                    <tr
                      key={idx}
                      className={`border-b border-slate-100 dark:border-slate-800 hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors ${
                        selectedMalware.has(malware.path) ? 'bg-red-50 dark:bg-red-900/20' : ''
                      }`}
                    >
                      <td className="py-3 px-4">
                        <input
                          type="checkbox"
                          checked={selectedMalware.has(malware.path)}
                          onChange={() => toggleMalwareSelection(malware.path)}
                          className="w-4 h-4 cursor-pointer"
                        />
                      </td>
                      <td className="py-3 px-4 text-sm text-slate-700 dark:text-slate-300 truncate">
                        {malware.path}
                      </td>
                      <td className="py-3 px-4 text-sm text-slate-700 dark:text-slate-300">
                        {malware.threat_type}
                      </td>
                      <td className="py-3 px-4 text-sm">
                        <span className={`px-2 py-1 rounded text-xs font-semibold ${
                          malware.threat_level === 'critical' ? 'bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200' :
                          malware.threat_level === 'high' ? 'bg-orange-100 dark:bg-orange-900 text-orange-800 dark:text-orange-200' :
                          malware.threat_level === 'medium' ? 'bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200' :
                          'bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200'
                        }`}>
                          {malware.threat_level.toUpperCase()}
                        </span>
                      </td>
                      <td className="py-3 px-4 text-sm text-slate-500 dark:text-slate-400">
                        {malware.reason}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        ) : !malwareLoading && hasMalwareScan ? (
          <div className="text-center py-12">
            <Shield className="mx-auto mb-3 text-green-400" size={40} />
            <p className="text-slate-500 dark:text-slate-400">No malware detected - Your system is clean!</p>
          </div>
        ) : (
          <div className="text-center py-12">
            <p className="text-slate-500 dark:text-slate-400">Click "Scan Malware" to scan for threats</p>
          </div>
        )}
      </div>
      )}
    </div>
  )
}
