import { useState, useEffect } from 'react'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faGear, faSave } from '@fortawesome/free-solid-svg-icons'

interface AppSettings {
  theme: string
  autoScan: boolean
  scanInterval: number
  safeMode: boolean
  language: string
}

export default function Settings() {
  const [settings, setSettings] = useState<AppSettings>({
    theme: 'dark',
    autoScan: false,
    scanInterval: 24,
    safeMode: true,
    language: 'fr',
  })

  const [saved, setSaved] = useState(false)

  // Charger les paramètres depuis localStorage au démarrage
  useEffect(() => {
    const savedSettings = localStorage.getItem('appSettings')
    if (savedSettings) {
      try {
        setSettings(JSON.parse(savedSettings))
      } catch (err) {
        console.error('Error loading settings:', err)
      }
    }
  }, [])

  const handleSave = () => {
    // Sauvegarder dans localStorage
    localStorage.setItem('appSettings', JSON.stringify(settings))
    localStorage.setItem('appLanguage', settings.language)
    setSaved(true)
    setTimeout(() => setSaved(false), 3000)
    // Recharger la page pour appliquer la langue
    window.location.reload()
  }

  return (
    <div className="p-8 space-y-8 animate-fade-in">
      <div>
        <h1 className="text-3xl font-bold text-slate-900 dark:text-white">Paramètres</h1>
        <p className="text-slate-600 dark:text-slate-400 mt-2">Configurez les préférences de Z-Cleaner</p>
      </div>

      <div className="max-w-2xl space-y-6">
        {/* General Settings */}
        <div className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700">
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
            <FontAwesomeIcon icon={faGear} />
            Paramètres Généraux
          </h3>

          <div className="space-y-4">
            {/* Theme */}
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                Thème
              </label>
              <select
                value={settings.theme}
                onChange={(e) => setSettings({ ...settings, theme: e.target.value })}
                className="w-full px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white"
              >
                <option value="light">Clair</option>
                <option value="dark">Sombre</option>
                <option value="auto">Auto</option>
              </select>
            </div>

            {/* Language */}
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                Langue
              </label>
              <select
                value={settings.language}
                onChange={(e) => setSettings({ ...settings, language: e.target.value })}
                className="w-full px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white"
              >
                <option value="en">English</option>
                <option value="fr">Français</option>
              </select>
            </div>
          </div>
        </div>

        {/* Scanning Settings */}
        <div className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-soft border border-slate-200 dark:border-slate-700">
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-4">Analyse</h3>

          <div className="space-y-4">
            {/* Auto Scan */}
            <label className="flex items-center gap-3 cursor-pointer">
              <input
                type="checkbox"
                checked={settings.autoScan}
                onChange={(e) => setSettings({ ...settings, autoScan: e.target.checked })}
                className="w-4 h-4"
              />
              <span className="text-slate-700 dark:text-slate-300">Activer l'analyse automatique</span>
            </label>

            {/* Scan Interval */}
            {settings.autoScan && (
              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  Intervalle d'analyse (heures)
                </label>
                <input
                  type="number"
                  min="1"
                  max="168"
                  value={settings.scanInterval}
                  onChange={(e) => setSettings({ ...settings, scanInterval: parseInt(e.target.value) })}
                  className="w-full px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white"
                />
              </div>
            )}

            {/* Safe Mode */}
            <label className="flex items-center gap-3 cursor-pointer">
              <input
                type="checkbox"
                checked={settings.safeMode}
                onChange={(e) => setSettings({ ...settings, safeMode: e.target.checked })}
                className="w-4 h-4"
              />
              <span className="text-slate-700 dark:text-slate-300">
                Mode sûr (analyser sans supprimer)
              </span>
            </label>
          </div>
        </div>

        {/* About */}
        <div className="bg-gradient-to-br from-blue-50 to-purple-50 dark:from-slate-800 dark:to-slate-900 rounded-xl p-6 border border-blue-200 dark:border-slate-700">
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-4">À propos de Z-Cleaner</h3>
          <div className="space-y-2 text-slate-700 dark:text-slate-300">
            <p>
              <strong>Version :</strong> 1.0.0
            </p>
            <p className="text-sm pt-2">
              Z-Cleaner est un nettoyeur système rapide et sécurisé qui vous aide à libérer de l'espace disque et à optimiser
              les performances de votre ordinateur.
            </p>
          </div>
        </div>

        {/* Save Button */}
        <button
          onClick={handleSave}
          className="w-full py-3 bg-gradient-to-r from-green-500 to-emerald-500 text-white rounded-lg font-semibold hover:shadow-lg transition-all flex items-center justify-center gap-2"
        >
          <FontAwesomeIcon icon={faSave} />
          Enregistrer les paramètres
        </button>

        {saved && (
          <div className="p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg text-green-700 dark:text-green-300 text-center">
            ✓ Paramètres enregistrés avec succès
          </div>
        )}
      </div>
    </div>
  )
}
