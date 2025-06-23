import { useContext } from 'react';
import { I18nContext } from '../config/i18n';

export interface UseTranslationReturn {
  t: (key: string, params?: Record<string, string | number>) => string;
  language: string;
  setLanguage: (lang: string) => void;
  availableLanguages: string[];
}

export const useTranslation = (): UseTranslationReturn => {
  const context = useContext(I18nContext);
  
  if (!context) {
    throw new Error('useTranslation must be used within an I18nProvider');
  }

  const { t, language, setLanguage, availableLanguages } = context;

  return {
    t,
    language,
    setLanguage,
    availableLanguages
  };
}; 