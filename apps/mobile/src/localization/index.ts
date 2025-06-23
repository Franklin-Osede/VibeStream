// Main configuration
export { default as i18n } from './config/i18n';

// Custom hook
export { useTranslation } from './hooks/useTranslation';

// Translations
export { default as enTranslations } from './translations/en';

// Simple types
export interface TranslationKey {
  key: string;
} 