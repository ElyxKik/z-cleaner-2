import fr from '../i18n/fr.json'
import en from '../i18n/en.json'

type TranslationKey = string

const translations = {
  fr: fr,
  en: en,
}

export const useTranslation = () => {
  const language = localStorage.getItem('appLanguage') || 'fr'
  
  const t = (key: TranslationKey): string => {
    const keys = key.split('.')
    let value: any = translations[language as keyof typeof translations] || translations.fr
    
    for (const k of keys) {
      value = value?.[k]
    }
    
    return value || key
  }
  
  return { t }
}
