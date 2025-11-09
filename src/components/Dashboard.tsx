import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { motion } from 'framer-motion'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faHardDrive, faBolt, faTriangleExclamation, faBroom, faGlobe, faMagnifyingGlass, faCog } from '@fortawesome/free-solid-svg-icons'
import { useTranslation } from '../hooks/useTranslation'

interface DiskStats {
  total_size: number
  used_size: number
  free_size: number
  percentage_used: number
  large_files_count: number
  large_files_size: number
}

interface DashboardProps {
  onNavigate: (page: 'dashboard' | 'cleaner' | 'analyzer' | 'optimizer' | 'settings') => void
}

export default function Dashboard({ onNavigate }: DashboardProps) {
  const { t } = useTranslation()
  const [stats, setStats] = useState<DiskStats | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadStats()
  }, [])

  const loadStats = async () => {
    try {
      setLoading(true)
      const result = await invoke<DiskStats>('analyze_disk_cmd')
      setStats(result)
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load stats')
    } finally {
      setLoading(false)
    }
  }

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i]
  }

  const handleQuickAction = (actionId: string) => {
    switch (actionId) {
      case 'cleanTemp':
        onNavigate('cleaner')
        break
      case 'browserCache':
        onNavigate('cleaner')
        break
      case 'analyze':
        onNavigate('analyzer')
        break
      case 'optimize':
        onNavigate('optimizer')
        break
      default:
        break
    }
  }

  return (
    <div className="p-8 space-y-8 animate-fade-in">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-slate-900 dark:text-white">{t('dashboard.title')}</h1>
          <p className="text-slate-600 dark:text-slate-400 mt-2">{t('dashboard.subtitle')}</p>
        </div>
        <button
          onClick={loadStats}
          className="px-6 py-3 bg-gradient-to-r from-blue-500 to-purple-500 text-white rounded-lg hover:shadow-lg transition-all hover:scale-105"
        >
          {t('dashboard.refresh')}
        </button>
      </div>

      {error && (
        <div className="p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg flex items-center gap-3">
          <FontAwesomeIcon icon={faTriangleExclamation} className="text-red-600 dark:text-red-400" />
          <span className="text-red-700 dark:text-red-300">{error}</span>
        </div>
      )}

      {loading ? (
        <div className="flex items-center justify-center h-64">
          <div className="animate-spin-slow">
            <FontAwesomeIcon icon={faBolt} className="text-blue-500" size="2x" />
          </div>
        </div>
      ) : stats ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          {/* Disk Usage Card */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700"
          >
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-sm font-semibold text-slate-600 dark:text-slate-400">{t('dashboard.diskUsage')}</h3>
              <FontAwesomeIcon icon={faHardDrive} className="text-blue-500" size="lg" />
            </div>
            <div className="space-y-3">
              <div className="text-2xl font-bold text-slate-900 dark:text-white">
                {stats.percentage_used.toFixed(1)}%
              </div>
              <div className="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                <div
                  className="bg-gradient-to-r from-blue-500 to-purple-500 h-2 rounded-full transition-all"
                  style={{ width: `${stats.percentage_used}%` }}
                />
              </div>
              <p className="text-xs text-slate-500 dark:text-slate-400">
                {formatBytes(stats.used_size)} / {formatBytes(stats.total_size)}
              </p>
            </div>
          </motion.div>

          {/* Free Space Card */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
            className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700"
          >
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-sm font-semibold text-slate-600 dark:text-slate-400">{t('dashboard.freeSpace')}</h3>
              <FontAwesomeIcon icon={faHardDrive} className="text-green-500" size="lg" />
            </div>
            <div className="space-y-3">
              <div className="text-2xl font-bold text-slate-900 dark:text-white">
                {formatBytes(stats.free_size)}
              </div>
              <p className="text-xs text-slate-500 dark:text-slate-400">
                {t('dashboard.availableOnYourSystem')}
              </p>
            </div>
          </motion.div>

          {/* Large Files Card */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
            className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700"
          >
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-sm font-semibold text-slate-600 dark:text-slate-400">{t('dashboard.largeFiles')}</h3>
              <FontAwesomeIcon icon={faTriangleExclamation} className="text-orange-500" size="lg" />
            </div>
            <div className="space-y-3">
              <div className="text-2xl font-bold text-slate-900 dark:text-white">
                {stats.large_files_count}
              </div>
              <p className="text-xs text-slate-500 dark:text-slate-400">
                {formatBytes(stats.large_files_size)} {t('dashboard.total')}
              </p>
            </div>
          </motion.div>

          {/* System Status Card */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3 }}
            className="bg-gradient-to-br from-blue-500 to-purple-500 rounded-xl p-6 shadow-soft text-white"
          >
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-sm font-semibold opacity-90">{t('dashboard.systemStatus')}</h3>
              <FontAwesomeIcon icon={faBolt} size="lg" />
            </div>
            <div className="space-y-3">
              <div className="text-2xl font-bold">{t('dashboard.healthy')}</div>
              <p className="text-xs opacity-75">{t('dashboard.allSystemsOperational')}</p>
            </div>
          </motion.div>
        </div>
      ) : null}

      {/* Quick Actions */}
      <div className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700">
        <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-4">{t('dashboard.quickActions')}</h3>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          {[
            { id: 'cleanTemp', label: t('dashboard.cleanTemp'), icon: faBroom, color: 'blue' },
            { id: 'browserCache', label: t('dashboard.browserCache'), icon: faGlobe, color: 'purple' },
            { id: 'analyze', label: t('dashboard.analyze'), icon: faMagnifyingGlass, color: 'green' },
            { id: 'optimize', label: t('dashboard.optimize'), icon: faCog, color: 'orange' },
          ].map((action) => (
            <button
              key={action.id}
              onClick={() => handleQuickAction(action.id)}
              className={`p-4 rounded-lg bg-slate-50 dark:bg-slate-800 hover:shadow-md transition-all hover:scale-105 text-center`}
            >
              <div className="text-2xl mb-2 text-slate-900 dark:text-white"><FontAwesomeIcon icon={action.icon} size="2x" /></div>
              <p className="text-sm font-medium text-slate-900 dark:text-white">{action.label}</p>
            </button>
          ))}
        </div>
      </div>
    </div>
  )
}
