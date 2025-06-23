import React, { createContext } from 'react';
import en from '../translations/en';

// Interface para el contexto
export interface I18nContextType {
  t: (key: string, params?: Record<string, string | number>) => string;
  language: string;
  setLanguage: (lang: string) => void;
  availableLanguages: string[];
}

// Crear el contexto
export const I18nContext = createContext<I18nContextType>({
  t: (key: string) => key,
  language: 'es',
  setLanguage: () => {},
  availableLanguages: ['en', 'es']
});

// Simple mock para i18n sin dependencias externas
const i18n = {
  translations: { en },
  locale: 'es',
  defaultLocale: 'en',
  t: (key: string): string => key
};

export default i18n; 