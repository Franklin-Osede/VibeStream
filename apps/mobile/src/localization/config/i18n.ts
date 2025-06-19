// Simple mock para i18n sin dependencias externas
import en from '../translations/en';

// Tipo para las traducciones
type Translations = {
  [locale: string]: any;
};

// Implementación simple de i18n
const i18n = {
  translations: { en } as Translations,
  locale: 'en',
  defaultLocale: 'en',
  
  // Método simple para obtener traducciones
  t: (key: string): string => {
    const keys = key.split('.');
    let result: any = i18n.translations[i18n.locale];
    
    for (const k of keys) {
      if (result && typeof result === 'object' && k in result) {
        result = result[k];
      } else {
        // Fallback al inglés si no se encuentra
        let fallback: any = i18n.translations[i18n.defaultLocale];
        for (const fbk of keys) {
          if (fallback && typeof fallback === 'object' && fbk in fallback) {
            fallback = fallback[fbk];
          } else {
            return key; // Retorna la clave si no se encuentra
          }
        }
        return typeof fallback === 'string' ? fallback : key;
      }
    }
    
    return typeof result === 'string' ? result : key;
  }
};

export default i18n; 