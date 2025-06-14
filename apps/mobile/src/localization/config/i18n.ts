import i18next from 'i18next';
import { initReactI18next } from 'react-i18next';
import * as RNLocalize from 'react-native-localize';

// Importar traducciones
import en from '../translations/en.json';
import es from '../translations/es.json';
import pt from '../translations/pt.json';
import it from '../translations/it.json';
import fr from '../translations/fr.json';

// Recursos de traducciones
const resources = {
  en: { translation: en },
  es: { translation: es },
  pt: { translation: pt },
  it: { translation: it },
  fr: { translation: fr },
};

// Idiomas soportados
export const SUPPORTED_LANGUAGES = [
  { code: 'en', name: 'English', flag: '🇺🇸' },
  { code: 'es', name: 'Español', flag: '🇪🇸' },
  { code: 'pt', name: 'Português', flag: '🇧🇷' },
  { code: 'it', name: 'Italiano', flag: '🇮🇹' },
  { code: 'fr', name: 'Français', flag: '🇫🇷' },
];

// Detectar idioma del dispositivo
const getDeviceLanguage = (): string => {
  const locales = RNLocalize.getLocales();
  if (locales.length > 0) {
    const deviceLanguage = locales[0].languageCode;
    // Verificar si el idioma del dispositivo está soportado
    const supportedCodes = SUPPORTED_LANGUAGES.map(lang => lang.code);
    return supportedCodes.includes(deviceLanguage) ? deviceLanguage : 'en';
  }
  return 'en'; // Fallback a inglés
};

// Configuración de i18next
i18next
  .use(initReactI18next)
  .init({
    resources,
    lng: getDeviceLanguage(), // Idioma inicial basado en el dispositivo
    fallbackLng: 'en', // Idioma de respaldo
    debug: __DEV__, // Solo en desarrollo
    
    interpolation: {
      escapeValue: false, // React ya escapa por seguridad
    },
    
    // Configuración de namespace
    defaultNS: 'translation',
    
    // Configuración de detección
    detection: {
      order: ['localStorage', 'navigator'],
      caches: ['localStorage'],
    },
    
    // Configuración de respaldo
    saveMissing: __DEV__, // Solo en desarrollo
    missingKeyHandler: (lng, ns, key) => {
      if (__DEV__) {
        console.warn(`Missing translation key: ${key} for language: ${lng}`);
      }
    },
  });

export default i18next; 