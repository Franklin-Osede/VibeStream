import { useColorScheme as _useColorScheme } from 'react-native';

// Fix para error de RNLocalize usando una implementación simple
export function useColorScheme(): 'dark' | 'light' {
  // Retorna directamente 'light' para simplificar
  return 'light';
} 