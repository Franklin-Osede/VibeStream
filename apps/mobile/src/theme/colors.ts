// VibeStream Professional Color Palette
// Optimized for music streaming app with blockchain integration

export const COLORS = {
  // === PRIMARY COLORS (Music & Creativity) ===
  primary: '#6C5CE7',           // Violeta vibrante - Botones principales
  primaryDark: '#5A4BD4',       // Violeta oscuro - Hover states
  primaryLight: '#A29BFE',      // Violeta claro - Elementos secundarios
  
  // === SECONDARY COLORS (Sound Waves & Energy) ===
  secondary: '#00D4FF',         // Azul cian - Ondas sonoras
  secondaryDark: '#0099CC',     // Azul cian oscuro
  secondaryLight: '#66E5FF',    // Azul cian claro
  
  // === ACCENT COLORS (Energy & Premium) ===
  accent: '#FF6B6B',            // Rojo coral - Acciones importantes
  accentGold: '#FFD93D',        // Dorado - Funciones premium
  accentPink: '#FD79A8',        // Rosa - Elementos de amor/favoritos
  
  // === BACKGROUND COLORS (Depth & Immersion) ===
  background: '#0F0F1E',        // Azul muy oscuro - Fondo principal
  backgroundSecondary: '#1A1A2E', // Azul grisáceo - Fondo secundario
  backgroundTertiary: '#16213E', // Azul medio - Fondo terciario
  surface: '#2D2D3A',           // Superficie elevada - Cards, modales
  surfaceLight: '#3A3A4A',      // Superficie clara - Elementos interactivos
  
  // === TEXT COLORS (Hierarchy & Readability) ===
  textPrimary: '#FFFFFF',       // Blanco puro - Títulos principales
  textSecondary: '#E2E8F0',     // Gris muy claro - Texto secundario
  textMuted: '#94A3B8',         // Gris medio - Texto auxiliar
  textDisabled: '#64748B',      // Gris oscuro - Texto deshabilitado
  
  // === STATE COLORS (Feedback & Status) ===
  success: '#4ECDC4',           // Verde aqua - Éxito
  successDark: '#26A69A',       // Verde aqua oscuro
  warning: '#FFB347',           // Naranja suave - Advertencias
  warningDark: '#FF9800',       // Naranja oscuro
  error: '#FF5757',             // Rojo brillante - Errores
  errorDark: '#E53E3E',         // Rojo oscuro
  info: '#3182CE',              // Azul informativo
  
  // === BLOCKCHAIN/CRYPTO COLORS ===
  bitcoin: '#F7931A',           // Naranja Bitcoin
  ethereum: '#627EEA',          // Azul Ethereum
  crypto: '#8B5CF6',            // Violeta crypto genérico
  
  // === TRANSPARENCY LEVELS ===
  overlay: 'rgba(0, 0, 0, 0.5)',           // Overlay oscuro
  overlayLight: 'rgba(0, 0, 0, 0.3)',      // Overlay ligero
  glassLight: 'rgba(255, 255, 255, 0.08)',  // Efecto glass claro
  glassMedium: 'rgba(255, 255, 255, 0.12)', // Efecto glass medio
  glassDark: 'rgba(0, 0, 0, 0.2)',         // Efecto glass oscuro
  
  // === MUSIC WAVE COLORS (For visualizers) ===
  wave: {
    low: '#4ECDC4',      // Frecuencias bajas - Verde aqua
    mid: '#FFD93D',      // Frecuencias medias - Dorado
    high: '#FF6B6B',     // Frecuencias altas - Rojo coral
  },
} as const;

// === GRADIENTS (Separate from COLORS to avoid type conflicts) ===
export const GRADIENTS = {
  primary: ['#6C5CE7', '#A29BFE'],
  secondary: ['#00D4FF', '#66E5FF'],
  accent: ['#FF6B6B', '#FD79A8'],
  gold: ['#FFD93D', '#FFA726'],
  background: ['#0F0F1E', '#1A1A2E', '#16213E'],
  surface: ['#2D2D3A', '#3A3A4A'],
  crypto: ['#F7931A', '#627EEA'],
} as const;

// === SEMANTIC COLOR MAPPING ===
export const SEMANTIC_COLORS = {
  // Buttons
  buttonPrimary: COLORS.primary,
  buttonSecondary: COLORS.surface,
  buttonDanger: COLORS.error,
  buttonSuccess: COLORS.success,
  
  // Inputs
  inputBackground: COLORS.glassLight,
  inputBorder: COLORS.glassMedium,
  inputBorderFocused: COLORS.primary,
  inputText: COLORS.textPrimary,
  inputPlaceholder: COLORS.textMuted,
  
  // Navigation
  tabActive: COLORS.primary,
  tabInactive: COLORS.textMuted,
  navigationBackground: COLORS.surface,
  
  // Cards
  cardBackground: COLORS.glassLight,
  cardBorder: COLORS.glassMedium,
  cardShadow: COLORS.glassDark,
  
  // Music Player
  playerBackground: COLORS.surface,
  playerProgress: COLORS.primary,
  playerProgressBackground: COLORS.textMuted,
  playButton: COLORS.accent,
  
  // Wallet/Crypto
  walletBalance: COLORS.accentGold,
  cryptoPositive: COLORS.success,
  cryptoNegative: COLORS.error,
} as const;

// === THEME TYPES ===
export type ColorKey = keyof Omit<typeof COLORS, 'wave'>;
export type SemanticColorKey = keyof typeof SEMANTIC_COLORS;
export type GradientKey = keyof typeof GRADIENTS;

// === HELPER FUNCTIONS ===
export const getColor = (key: ColorKey): string => COLORS[key] as string;
export const getSemanticColor = (key: SemanticColorKey): string => SEMANTIC_COLORS[key];
export const getGradient = (key: GradientKey): readonly string[] => GRADIENTS[key];

// === ACCESSIBILITY HELPERS ===
export const ACCESSIBILITY = {
  // High contrast alternatives
  highContrast: {
    text: '#FFFFFF',
    background: '#000000',
    border: '#FFFFFF',
  },
  
  // Focus indicators
  focusRing: COLORS.primary,
  focusRingWidth: 2,
  
  // Minimum contrast ratios met
  contrastRatio: {
    normal: 4.5,  // WCAG AA
    large: 3,     // WCAG AA Large
  },
} as const; 