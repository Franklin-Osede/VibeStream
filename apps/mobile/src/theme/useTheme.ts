import { useMemo } from 'react';
import { COLORS, GRADIENTS, SEMANTIC_COLORS, getColor, getSemanticColor, getGradient } from './colors';
import type { ColorKey, SemanticColorKey, GradientKey } from './colors';

// Hook personalizado para el tema
export const useTheme = () => {
  const theme = useMemo(() => ({
    // Colores directos
    colors: COLORS,
    gradients: GRADIENTS,
    semanticColors: SEMANTIC_COLORS,
    
    // Funciones helper
    getColor,
    getSemanticColor,
    getGradient,
    
    // Acceso rápido a colores más usados
    primary: COLORS.primary,
    secondary: COLORS.secondary,
    accent: COLORS.accent,
    background: COLORS.background,
    surface: COLORS.surface,
    text: COLORS.textPrimary,
    textSecondary: COLORS.textSecondary,
    textMuted: COLORS.textMuted,
    success: COLORS.success,
    error: COLORS.error,
    warning: COLORS.warning,
    
    // Utilidades para estilos comunes
    styles: {
      // Container principal
      container: {
        flex: 1,
        backgroundColor: COLORS.background,
      },
      
      // Superficie elevada (cards, modales)
      surface: {
        backgroundColor: COLORS.surface,
        borderRadius: 16,
        shadowColor: COLORS.glassDark,
        shadowOffset: { width: 0, height: 4 },
        shadowOpacity: 0.3,
        shadowRadius: 8,
        elevation: 8,
      },
      
      // Efecto glass
      glass: {
        backgroundColor: COLORS.glassLight,
        borderWidth: 1,
        borderColor: COLORS.glassMedium,
        borderRadius: 16,
      },
      
      // Input común
      input: {
        backgroundColor: COLORS.glassLight,
        borderWidth: 1,
        borderColor: COLORS.glassMedium,
        borderRadius: 12,
        padding: 16,
        color: COLORS.textPrimary,
        fontSize: 16,
      },
      
      // Botón primario
      buttonPrimary: {
        backgroundColor: COLORS.primary,
        borderRadius: 12,
        paddingVertical: 16,
        paddingHorizontal: 24,
        alignItems: 'center' as const,
        justifyContent: 'center' as const,
      },
      
      // Texto de botón primario
      buttonPrimaryText: {
        color: COLORS.textPrimary,
        fontSize: 16,
        fontWeight: '600' as const,
      },
      
      // Texto principal
      textPrimary: {
        color: COLORS.textPrimary,
        fontSize: 16,
        fontWeight: '400' as const,
      },
      
      // Texto secundario
      textSecondary: {
        color: COLORS.textSecondary,
        fontSize: 14,
        fontWeight: '400' as const,
      },
      
      // Texto muted
      textMuted: {
        color: COLORS.textMuted,
        fontSize: 12,
        fontWeight: '400' as const,
      },
      
      // Título grande
      titleLarge: {
        color: COLORS.textPrimary,
        fontSize: 32,
        fontWeight: '700' as const,
      },
      
      // Título mediano
      titleMedium: {
        color: COLORS.textPrimary,
        fontSize: 24,
        fontWeight: '600' as const,
      },
      
      // Título pequeño
      titleSmall: {
        color: COLORS.textPrimary,
        fontSize: 18,
        fontWeight: '600' as const,
      },
    },
    
    // Espaciado consistente
    spacing: {
      xs: 4,
      sm: 8,
      md: 16,
      lg: 24,
      xl: 32,
      xxl: 48,
    },
    
    // Border radius consistente
    borderRadius: {
      xs: 4,
      sm: 8,
      md: 12,
      lg: 16,
      xl: 24,
      full: 9999,
    },
    
    // Sombras consistentes
    shadows: {
      sm: {
        shadowColor: COLORS.glassDark,
        shadowOffset: { width: 0, height: 2 },
        shadowOpacity: 0.1,
        shadowRadius: 4,
        elevation: 2,
      },
      md: {
        shadowColor: COLORS.glassDark,
        shadowOffset: { width: 0, height: 4 },
        shadowOpacity: 0.15,
        shadowRadius: 8,
        elevation: 4,
      },
      lg: {
        shadowColor: COLORS.glassDark,
        shadowOffset: { width: 0, height: 8 },
        shadowOpacity: 0.2,
        shadowRadius: 12,
        elevation: 8,
      },
    },
  }), []);

  // Función para crear gradientes fácilmente
  const createGradient = (colors: string[], start = { x: 0, y: 0 }, end = { x: 1, y: 0 }) => ({
    colors,
    start,
    end,
  });

  // Función para obtener colores de estado
  const getStatusColor = (status: 'success' | 'error' | 'warning' | 'info') => {
    switch (status) {
      case 'success': return COLORS.success;
      case 'error': return COLORS.error;
      case 'warning': return COLORS.warning;
      case 'info': return COLORS.info;
      default: return COLORS.textMuted;
    }
  };

  return {
    ...theme,
    createGradient,
    getStatusColor,
  };
};

// Hook para obtener solo los colores (más ligero)
export const useColors = () => {
  return useMemo(() => COLORS, []);
};

// Hook para obtener solo colores semánticos
export const useSemanticColors = () => {
  return useMemo(() => SEMANTIC_COLORS, []);
}; 