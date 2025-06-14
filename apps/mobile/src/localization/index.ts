// Main configuration
export { default as i18n, SUPPORTED_LANGUAGES } from './config/i18n';

// Custom hook
export { useTranslation } from './hooks/useTranslation';
export type { TranslationKey, TranslationParams } from './hooks/useTranslation';

// Components
export { LanguageSelector } from './components/LanguageSelector';

// Translations (if needed for direct access)
export { default as enTranslations } from './translations/en.json';
export { default as esTranslations } from './translations/es.json';
export { default as ptTranslations } from './translations/pt.json';
export { default as itTranslations } from './translations/it.json';
export { default as frTranslations } from './translations/fr.json'; 