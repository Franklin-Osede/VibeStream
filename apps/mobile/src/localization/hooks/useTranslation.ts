import { useTranslation as useI18nTranslation, UseTranslationOptions } from 'react-i18next';
import { SUPPORTED_LANGUAGES } from '../config/i18n';

// Tipos para las claves de traducción
export type TranslationKey = 
  | 'app.name'
  | 'app.tagline'
  | 'auth.login.title'
  | 'auth.login.subtitle'
  | 'auth.login.email'
  | 'auth.login.password'
  | 'auth.login.button'
  | 'auth.login.switchToRegister'
  | 'auth.login.success'
  | 'auth.login.welcomeBack'
  | 'auth.login.errors.invalidCredentials'
  | 'auth.login.errors.networkError'
  | 'auth.login.errors.serverError'
  | 'auth.login.errors.emptyFields'
  | 'auth.register.title'
  | 'auth.register.subtitle'
  | 'auth.register.email'
  | 'auth.register.username'
  | 'auth.register.password'
  | 'auth.register.button'
  | 'auth.register.switchToLogin'
  | 'auth.register.success'
  | 'auth.register.accountCreated'
  | 'auth.register.errors.emailExists'
  | 'auth.register.errors.usernameExists'
  | 'auth.register.errors.passwordTooShort'
  | 'auth.register.errors.invalidEmail'
  | 'auth.register.errors.networkError'
  | 'auth.register.errors.serverError'
  | 'auth.register.errors.emptyFields'
  | 'navigation.home'
  | 'navigation.music'
  | 'navigation.wallet'
  | 'navigation.profile'
  | 'navigation.settings'
  | 'common.loading'
  | 'common.error'
  | 'common.success'
  | 'common.cancel'
  | 'common.confirm'
  | 'common.save'
  | 'common.delete'
  | 'common.edit'
  | 'common.share'
  | 'common.back'
  | 'common.next'
  | 'common.done'
  | 'common.retry';

// Interface para parámetros de interpolación
export interface TranslationParams {
  [key: string]: string | number;
}

// Hook personalizado con tipado fuerte
export const useTranslation = () => {
  const { t, i18n } = useI18nTranslation();

  // Función de traducción con tipado
  const translate = (key: TranslationKey, params?: TranslationParams): string => {
    return t(key, params);
  };

  // Cambiar idioma
  const changeLanguage = async (languageCode: string): Promise<void> => {
    try {
      await i18n.changeLanguage(languageCode);
    } catch (error) {
      console.error('Error changing language:', error);
    }
  };

  // Obtener idioma actual
  const getCurrentLanguage = () => {
    return i18n.language;
  };

  // Obtener información del idioma actual
  const getCurrentLanguageInfo = () => {
    return SUPPORTED_LANGUAGES.find(lang => lang.code === i18n.language) || SUPPORTED_LANGUAGES[0];
  };

  // Verificar si un idioma está soportado
  const isLanguageSupported = (languageCode: string): boolean => {
    return SUPPORTED_LANGUAGES.some(lang => lang.code === languageCode);
  };

  // Obtener idiomas soportados
  const getSupportedLanguages = () => {
    return SUPPORTED_LANGUAGES;
  };

  return {
    t: translate,
    i18n,
    changeLanguage,
    getCurrentLanguage,
    getCurrentLanguageInfo,
    isLanguageSupported,
    getSupportedLanguages,
    // Mantener compatibilidad con react-i18next
    ready: i18n.isInitialized,
    language: i18n.language,
  };
}; 