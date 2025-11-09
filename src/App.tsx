import { useState, useEffect } from 'react'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faMoon, faSun, faChartPie, faMagnifyingGlass, faBroom, faCog, faGear } from '@fortawesome/free-solid-svg-icons'
import { useTranslation } from './hooks/useTranslation'
import Dashboard from './components/Dashboard'
import Cleaner from './components/Cleaner'
import Analyzer from './components/Analyzer'
import Optimizer from './components/Optimizer'
import Settings from './components/Settings'

type Page = 'dashboard' | 'cleaner' | 'analyzer' | 'optimizer' | 'settings'

const navIcons = {
  dashboard: faChartPie,
  analyzer: faMagnifyingGlass,
  cleaner: faBroom,
  optimizer: faCog,
  settings: faGear,
}

export default function App() {
  const [currentPage, setCurrentPage] = useState<Page>('dashboard')
  const [isDark, setIsDark] = useState(true)
  const { t } = useTranslation()

  useEffect(() => {
    if (isDark) {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
  }, [isDark])

  return (
    <div className={`${isDark ? 'dark' : ''} min-h-screen bg-white dark:bg-slate-950 transition-colors`}>
      <div className="flex h-screen">
        {/* Sidebar */}
        <aside className="w-64 bg-gradient-to-b from-slate-100 to-slate-50 dark:from-slate-900 dark:to-slate-950 border-r border-slate-200 dark:border-slate-700 shadow-lg">
          <div className="p-6 border-b border-slate-200 dark:border-slate-700 flex flex-col items-center">
            {/* Logo */}
            <div className="mb-4 w-20 h-20 bg-gradient-to-br from-blue-400 to-blue-600 rounded-lg flex items-center justify-center shadow-lg">
              <img 
                src="/src-tauri/icons/icon.png" 
                alt="Z-Cleaner Logo" 
                className="w-16 h-16 object-contain"
              />
            </div>
            <h1 className="text-2xl font-bold bg-gradient-to-r from-blue-500 to-purple-600 bg-clip-text text-transparent text-center">
              Z-Cleaner
            </h1>
            <p className="text-xs text-slate-500 dark:text-slate-400 mt-1 text-center">{t('sidebar.subtitle')}</p>
          </div>

          <nav className="p-4 space-y-2">
            {[
              { id: 'dashboard', label: t('sidebar.dashboard') },
              { id: 'analyzer', label: t('sidebar.analyzer') },
              { id: 'cleaner', label: t('sidebar.cleaner') },
              { id: 'optimizer', label: t('sidebar.optimizer') },
              { id: 'settings', label: t('sidebar.settings') },
            ].map((item) => (
              <button
                key={item.id}
                onClick={() => setCurrentPage(item.id as Page)}
                className={`w-full text-left px-4 py-3 rounded-lg transition-all flex items-center gap-3 ${
                  currentPage === item.id
                    ? 'bg-gradient-to-r from-blue-500 to-purple-500 text-white shadow-lg'
                    : 'text-slate-600 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-700 hover:text-slate-900 dark:hover:text-white'
                }`}
              >
                <FontAwesomeIcon icon={navIcons[item.id as Page]} size="lg" />
                {item.label}
              </button>
            ))}
          </nav>
        </aside>

        {/* Main Content */}
        <main className="flex-1 flex flex-col overflow-hidden">
          {/* Top Bar */}
          <header className="bg-white dark:bg-slate-900 border-b border-slate-200 dark:border-slate-700 px-8 py-4 flex justify-between items-center shadow-sm">
            <h2 className="text-2xl font-bold text-slate-900 dark:text-white">
              {currentPage.charAt(0).toUpperCase() + currentPage.slice(1)}
            </h2>
            <button
              onClick={() => setIsDark(!isDark)}
              className="p-2 rounded-lg bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
            >
              <FontAwesomeIcon icon={isDark ? faSun : faMoon} size="lg" />
            </button>
          </header>

          {/* Page Content */}
          <div className="flex-1 overflow-auto">
            {currentPage === 'dashboard' && <Dashboard onNavigate={setCurrentPage} />}
            {currentPage === 'analyzer' && <Analyzer />}
            {currentPage === 'cleaner' && <Cleaner />}
            {currentPage === 'optimizer' && <Optimizer />}
            {currentPage === 'settings' && <Settings />}
          </div>
        </main>
      </div>
    </div>
  )
}
