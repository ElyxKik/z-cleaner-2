import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { motion } from 'framer-motion'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faBolt, faToggleOn, faMicrochip, faHardDrive, faHeartPulse, faTriangleExclamation } from '@fortawesome/free-solid-svg-icons'
import { useTranslation } from '../hooks/useTranslation'

interface StartupProgram {
  name: string
  path: string
  enabled: boolean
  size: number
}

interface MemoryHog {
  name: string
  pid: number
  memory_mb: number
  memory_percent: number
}

interface SystemPerformance {
  cpu_usage: number
  memory_usage: number
  disk_usage: number
}

export default function Optimizer() {
  const { t } = useTranslation()
  const [programs, setPrograms] = useState<StartupProgram[]>([])
  const [memoryHogs, setMemoryHogs] = useState<MemoryHog[]>([])
  const [performance, setPerformance] = useState<SystemPerformance>({
    cpu_usage: 0,
    memory_usage: 0,
    disk_usage: 0,
  })
  const [loading, setLoading] = useState(false)
  const [hogsLoading, setHogsLoading] = useState(false)

  useEffect(() => {
    loadPrograms()
    loadPerformance()
    loadMemoryHogs()
    
    // Rafraîchir les performances toutes les 3 secondes
    const perfInterval = setInterval(() => {
      loadPerformance()
    }, 3000)
    
    return () => {
      clearInterval(perfInterval)
    }
  }, [])

  const loadPrograms = async () => {
    try {
      setLoading(true)
      const result = await invoke<StartupProgram[]>('get_startup_programs_cmd')
      setPrograms(result)
    } catch (err) {
      console.error('Error loading programs:', err)
    } finally {
      setLoading(false)
    }
  }

  const loadPerformance = async () => {
    try {
      const result = await invoke<SystemPerformance>('get_system_performance_cmd')
      console.log('Performance data received:', result)
      setPerformance(result)
    } catch (err) {
      console.error('Error loading performance:', err)
    }
  }

  const loadMemoryHogs = async () => {
    try {
      setHogsLoading(true)
      const result = await invoke<MemoryHog[]>('get_memory_hogs_cmd')
      setMemoryHogs(result)
    } catch (err) {
      console.error('Error loading memory hogs:', err)
    } finally {
      setHogsLoading(false)
    }
  }

  const handleDisable = async (name: string) => {
    try {
      await invoke('disable_startup_program_cmd', { name })
      setPrograms(programs.map((p) => (p.name === name ? { ...p, enabled: false } : p)))
    } catch (err) {
      console.error('Error disabling program:', err)
    }
  }

  const handleKillProcess = async (pid: number, name: string) => {
    try {
      const confirmed = window.confirm(`Are you sure you want to kill "${name}" (PID: ${pid})?`)
      if (!confirmed) return

      await invoke('kill_process_cmd', { pid })
      // Rafraîchir la liste après avoir tué le processus
      await loadMemoryHogs()
    } catch (err) {
      console.error('Error killing process:', err)
      alert(`Failed to kill process: ${err}`)
    }
  }

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i]
  }

  const getUsageColor = (usage: number) => {
    if (usage < 50) return 'from-green-500 to-emerald-500'
    if (usage < 75) return 'from-yellow-500 to-orange-500'
    return 'from-red-500 to-pink-500'
  }

  return (
    <div className="p-8 space-y-8 animate-fade-in">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-slate-900 dark:text-white">{t('optimizer.title')}</h1>
          <p className="text-slate-600 dark:text-slate-400 mt-2">{t('optimizer.subtitle')}</p>
        </div>
        <button
          onClick={loadPrograms}
          disabled={loading}
          className="px-6 py-3 bg-gradient-to-r from-blue-500 to-purple-500 text-white rounded-lg hover:shadow-lg transition-all disabled:opacity-50"
        >
          {loading ? 'Loading...' : t('optimizer.refresh')}
        </button>
      </div>

      {/* System Performance */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700"
          >
            <div className="flex items-center gap-3 mb-4">
              <FontAwesomeIcon icon={faMicrochip} className="text-blue-500" size="lg" />
              <h3 className="text-lg font-semibold text-slate-900 dark:text-white">{t('optimizer.cpuUsage')}</h3>
            </div>
            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <span className="text-3xl font-bold text-slate-900 dark:text-white">
                  {performance.cpu_usage.toFixed(1)}%
                </span>
              </div>
              <div className="h-2 bg-slate-200 dark:bg-slate-700 rounded-full overflow-hidden">
                <motion.div
                  className={`h-full bg-gradient-to-r ${getUsageColor(performance.cpu_usage)}`}
                  initial={{ width: 0 }}
                  animate={{ width: `${performance.cpu_usage}%` }}
                  transition={{ duration: 0.5 }}
                />
              </div>
            </div>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
            className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700"
          >
            <div className="flex items-center gap-3 mb-4">
              <FontAwesomeIcon icon={faHeartPulse} className="text-purple-500" size="lg" />
              <h3 className="text-lg font-semibold text-slate-900 dark:text-white">{t('optimizer.memoryUsage')}</h3>
            </div>
            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <span className="text-3xl font-bold text-slate-900 dark:text-white">
                  {performance.memory_usage.toFixed(1)}%
                </span>
              </div>
              <div className="h-2 bg-slate-200 dark:bg-slate-700 rounded-full overflow-hidden">
                <motion.div
                  className={`h-full bg-gradient-to-r ${getUsageColor(performance.memory_usage)}`}
                  initial={{ width: 0 }}
                  animate={{ width: `${performance.memory_usage}%` }}
                  transition={{ duration: 0.5 }}
                />
              </div>
            </div>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
            className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700"
          >
            <div className="flex items-center gap-3 mb-4">
              <FontAwesomeIcon icon={faHardDrive} className="text-orange-500" size="lg" />
              <h3 className="text-lg font-semibold text-slate-900 dark:text-white">{t('optimizer.diskUsage')}</h3>
            </div>
            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <span className="text-3xl font-bold text-slate-900 dark:text-white">
                  {performance.disk_usage.toFixed(1)}%
                </span>
              </div>
              <div className="h-2 bg-slate-200 dark:bg-slate-700 rounded-full overflow-hidden">
                <motion.div
                  className={`h-full bg-gradient-to-r ${getUsageColor(performance.disk_usage)}`}
                  initial={{ width: 0 }}
                  animate={{ width: `${performance.disk_usage}%` }}
                  transition={{ duration: 0.5 }}
                />
              </div>
            </div>
          </motion.div>
      </div>

      {/* Memory Hogs */}
      <div className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-2 mb-4">
          <FontAwesomeIcon icon={faTriangleExclamation} className="text-red-500" size="lg" />
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white">{t('optimizer.memoryHogs')}</h3>
        </div>
          <button
            onClick={loadMemoryHogs}
            disabled={hogsLoading}
            className="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-all disabled:opacity-50"
          >
            {hogsLoading ? 'Loading...' : 'Refresh'}
          </button>
        </div>

        {hogsLoading ? (
          <div className="flex justify-center py-12">
            <div className="animate-spin-slow">
              <FontAwesomeIcon icon={faBolt} className="text-blue-500" size="2x" />
            </div>
          </div>
        ) : memoryHogs.length > 0 ? (
          <div className="space-y-3">
            {memoryHogs.map((hog) => (
              <div
                key={hog.pid}
                className="flex items-center justify-between p-4 bg-slate-50 dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-700 hover:shadow-md transition-all"
              >
                <div className="flex-1 min-w-0">
                  <p className="font-medium text-slate-900 dark:text-white truncate">{hog.name}</p>
                  <p className="text-xs text-slate-500 dark:text-slate-400">PID: {hog.pid}</p>
                </div>
                <div className="flex items-center gap-3 ml-4 flex-shrink-0">
                  <div className="text-right">
                    <p className="font-semibold text-slate-900 dark:text-white whitespace-nowrap">{hog.memory_mb.toFixed(1)} MB</p>
                    <p className="text-xs text-slate-500 dark:text-slate-400">{hog.memory_percent.toFixed(1)}%</p>
                  </div>
                  <button
                    onClick={() => handleKillProcess(hog.pid, hog.name)}
                    className="px-3 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg font-medium transition-all text-sm flex-shrink-0"
                  >
                    Kill
                  </button>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <div className="text-center py-12">
            <p className="text-slate-500 dark:text-slate-400">{t('optimizer.noMemoryHogs')}</p>
          </div>
        )}
      </div>

      {/* Startup Programs */}
      <div className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700">
        <div className="flex items-center gap-2 mb-4">
          <FontAwesomeIcon icon={faToggleOn} className="text-orange-500" size="lg" />
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white">{t('optimizer.startupPrograms')}</h3>
        </div>

        {loading ? (
          <div className="flex justify-center py-12">
            <div className="animate-spin-slow">
              <FontAwesomeIcon icon={faBolt} className="text-blue-500" size="2x" />
            </div>
          </div>
        ) : programs.length > 0 ? (
          <div className="space-y-3">
            {programs.map((program) => (
              <div
                key={program.name}
                className="flex items-center justify-between p-4 bg-slate-50 dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-700 hover:shadow-md transition-all"
              >
                <div className="flex-1">
                  <p className="font-medium text-slate-900 dark:text-white">{program.name}</p>
                  <p className="text-xs text-slate-500 dark:text-slate-400 truncate">{program.path}</p>
                  <p className="text-xs text-slate-500 dark:text-slate-400 mt-1">
                    Size: {formatBytes(program.size)}
                  </p>
                </div>
                <button
                  onClick={() => handleDisable(program.name)}
                  disabled={!program.enabled}
                  className={`px-4 py-2 rounded-lg font-medium transition-all ${
                    program.enabled
                      ? 'bg-red-500 text-white hover:bg-red-600'
                      : 'bg-slate-300 dark:bg-slate-600 text-slate-500 dark:text-slate-400 cursor-not-allowed'
                  }`}
                >
                  {program.enabled ? t('optimizer.disable') : t('optimizer.disabled')}
                </button>
              </div>
            ))}
          </div>
        ) : (
          <div className="text-center py-12">
            <p className="text-slate-500 dark:text-slate-400">{t('optimizer.noStartupPrograms')}</p>
          </div>
        )}
      </div>

      {/* Performance Tips */}
      <div className="bg-gradient-to-br from-blue-50 to-purple-50 dark:from-slate-800 dark:to-slate-900 rounded-xl p-6 border border-blue-200 dark:border-slate-700">
        <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-4">{t('optimizer.optimizationTips')}</h3>
        <ul className="space-y-3">
          {[
            t('optimizer.tip1'),
            t('optimizer.tip2'),
            t('optimizer.tip3'),
            t('optimizer.tip4'),
            t('optimizer.tip5'),
            t('optimizer.tip6'),
          ].map((tip, idx) => (
            <li key={idx} className="flex items-start gap-3">
              <span className="text-blue-500 font-bold mt-0.5">✓</span>
              <span className="text-slate-700 dark:text-slate-300">{tip}</span>
            </li>
          ))}
        </ul>
      </div>
    </div>
  )
}
