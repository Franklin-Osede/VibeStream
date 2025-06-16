import React, { useState } from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  StyleSheet,
  SafeAreaView,
  Animated,
  Dimensions,
} from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { LinearGradient } from 'expo-linear-gradient';
import { useTheme } from '../../theme';
import { useTranslation } from '../../localization/hooks/useTranslation';

const { width } = Dimensions.get('window');

interface OnboardingScreenProps {
  navigation: any;
  route: any;
}

export default function OnboardingScreen({ navigation, route }: OnboardingScreenProps) {
  const { user } = route.params;
  const { t } = useTranslation();
  const theme = useTheme();
  const [selectedRole, setSelectedRole] = useState<'fan' | 'artist' | null>(null);
  const [fadeAnim] = useState(new Animated.Value(0));
  const [slideAnim] = useState(new Animated.Value(50));

  const styles = createStyles(theme);

  React.useEffect(() => {
    Animated.parallel([
      Animated.timing(fadeAnim, {
        toValue: 1,
        duration: 800,
        useNativeDriver: true,
      }),
      Animated.timing(slideAnim, {
        toValue: 0,
        duration: 800,
        useNativeDriver: true,
      }),
    ]).start();
  }, []);

  const handleRoleSelection = (role: 'fan' | 'artist') => {
    setSelectedRole(role);
  };

  const handleContinue = async () => {
    if (!selectedRole) return;

    // TODO: Actualizar el rol del usuario en el backend
    const updatedUser = { ...user, role: selectedRole };
    
    // Navegar según el rol seleccionado
    if (selectedRole === 'artist') {
      navigation.replace('ArtistMain', { user: updatedUser });
    } else {
      navigation.replace('Main', { user: updatedUser });
    }
  };

  return (
    <SafeAreaView style={styles.container}>
      <StatusBar style="light" />
      
      {/* Fondo con gradiente */}
      <LinearGradient
        colors={theme.gradients.background}
        style={styles.background}
        start={{ x: 0, y: 0 }}
        end={{ x: 1, y: 1 }}
      />

      <Animated.View 
        style={[
          styles.content,
          {
            opacity: fadeAnim,
            transform: [{ translateY: slideAnim }]
          }
        ]}
      >
        {/* Header */}
        <View style={styles.header}>
          <LinearGradient
            colors={[theme.primary, theme.colors.primaryLight]}
            style={styles.logoContainer}
            start={{ x: 0, y: 0 }}
            end={{ x: 1, y: 1 }}
          >
            <Text style={styles.logoIcon}>🎵</Text>
          </LinearGradient>
          
          <Text style={styles.welcomeTitle}>
            ¡Bienvenido a VibeStream!
          </Text>
          <Text style={styles.welcomeSubtitle}>
            @{user.username}, cuéntanos más sobre ti
          </Text>
        </View>

        {/* Role Selection */}
        <View style={styles.roleSection}>
          <Text style={styles.sectionTitle}>¿Cómo quieres usar VibeStream?</Text>
          
          {/* Fan Option */}
          <TouchableOpacity
            style={[
              styles.roleCard,
              selectedRole === 'fan' && styles.roleCardSelected
            ]}
            onPress={() => handleRoleSelection('fan')}
          >
            <LinearGradient
              colors={
                selectedRole === 'fan' 
                  ? [theme.primary + '30', theme.accent + '20']
                  : [theme.colors.glassLight, theme.colors.glassLight]
              }
              style={styles.roleGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <View style={styles.roleIcon}>
                <Text style={styles.roleEmoji}>🎧</Text>
              </View>
              
              <Text style={styles.roleTitle}>Soy Fan de la Música</Text>
              <Text style={styles.roleDescription}>
                • Escucha música y gana dinero{'\n'}
                • Descubre nuevos artistas{'\n'}
                • Invierte en canciones{'\n'}
                • Participa en campañas NFT
              </Text>
              
              <View style={styles.earningsBadge}>
                <Text style={styles.earningsText}>💰 Gana hasta $50/mes</Text>
              </View>
            </LinearGradient>
          </TouchableOpacity>

          {/* Artist Option */}
          <TouchableOpacity
            style={[
              styles.roleCard,
              selectedRole === 'artist' && styles.roleCardSelected
            ]}
            onPress={() => handleRoleSelection('artist')}
          >
            <LinearGradient
              colors={
                selectedRole === 'artist' 
                  ? [theme.accent + '30', theme.colors.accentPink + '20']
                  : [theme.colors.glassLight, theme.colors.glassLight]
              }
              style={styles.roleGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <View style={styles.roleIcon}>
                <Text style={styles.roleEmoji}>🎤</Text>
              </View>
              
              <Text style={styles.roleTitle}>Soy Artista/Creador</Text>
              <Text style={styles.roleDescription}>
                • Sube tu música y monetízala{'\n'}
                • Crea campañas NFT promocionales{'\n'}
                • Vende participaciones en royalties{'\n'}
                • Conecta directamente con fans
              </Text>
              
              <View style={styles.earningsBadge}>
                <Text style={styles.earningsText}>🎯 Gana más que en Spotify</Text>
              </View>
            </LinearGradient>
          </TouchableOpacity>
        </View>

        {/* Continue Button */}
        <View style={styles.footer}>
          <TouchableOpacity
            style={[
              styles.continueButton,
              !selectedRole && styles.continueButtonDisabled
            ]}
            onPress={handleContinue}
            disabled={!selectedRole}
          >
            <LinearGradient
              colors={
                selectedRole 
                  ? [theme.primary, theme.colors.primaryLight]
                  : [theme.colors.textDisabled, theme.colors.textMuted]
              }
              style={styles.continueGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 0 }}
            >
              <Text style={styles.continueText}>
                {selectedRole ? 'Comenzar a Ganar 🚀' : 'Selecciona una opción'}
              </Text>
            </LinearGradient>
          </TouchableOpacity>
          
          <Text style={styles.footnote}>
            Podrás cambiar esto más tarde en configuración
          </Text>
        </View>
      </Animated.View>
    </SafeAreaView>
  );
}

const createStyles = (theme: ReturnType<typeof useTheme>) => StyleSheet.create({
  container: {
    flex: 1,
  },
  background: {
    position: 'absolute',
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
  },
  content: {
    flex: 1,
    paddingHorizontal: theme.spacing.lg,
    paddingVertical: theme.spacing.xl,
  },
  header: {
    alignItems: 'center',
    marginBottom: theme.spacing.xxl,
  },
  logoContainer: {
    width: 80,
    height: 80,
    borderRadius: 40,
    justifyContent: 'center',
    alignItems: 'center',
    marginBottom: theme.spacing.lg,
    ...theme.shadows.lg,
  },
  logoIcon: {
    fontSize: 40,
    color: theme.text,
  },
  welcomeTitle: {
    ...theme.styles.titleLarge,
    textAlign: 'center',
    marginBottom: theme.spacing.sm,
  },
  welcomeSubtitle: {
    ...theme.styles.textSecondary,
    textAlign: 'center',
    fontSize: 16,
  },
  roleSection: {
    flex: 1,
    marginBottom: theme.spacing.xl,
  },
  sectionTitle: {
    ...theme.styles.titleMedium,
    textAlign: 'center',
    marginBottom: theme.spacing.xl,
  },
  roleCard: {
    marginBottom: theme.spacing.lg,
    borderRadius: theme.borderRadius.xl,
    overflow: 'hidden',
    ...theme.shadows.md,
  },
  roleCardSelected: {
    ...theme.shadows.lg,
    shadowColor: theme.primary,
  },
  roleGradient: {
    padding: theme.spacing.xl,
    borderWidth: 2,
    borderColor: 'transparent',
  },
  roleIcon: {
    alignItems: 'center',
    marginBottom: theme.spacing.md,
  },
  roleEmoji: {
    fontSize: 48,
  },
  roleTitle: {
    ...theme.styles.titleMedium,
    textAlign: 'center',
    marginBottom: theme.spacing.md,
  },
  roleDescription: {
    ...theme.styles.textSecondary,
    textAlign: 'center',
    lineHeight: 20,
    marginBottom: theme.spacing.md,
  },
  earningsBadge: {
    backgroundColor: theme.colors.success + '20',
    paddingVertical: theme.spacing.sm,
    paddingHorizontal: theme.spacing.md,
    borderRadius: theme.borderRadius.lg,
    alignSelf: 'center',
  },
  earningsText: {
    color: theme.colors.success,
    fontSize: 14,
    fontWeight: '600',
    textAlign: 'center',
  },
  footer: {
    alignItems: 'center',
  },
  continueButton: {
    width: '100%',
    borderRadius: theme.borderRadius.xl,
    overflow: 'hidden',
    ...theme.shadows.lg,
    marginBottom: theme.spacing.md,
  },
  continueButtonDisabled: {
    opacity: 0.6,
  },
  continueGradient: {
    paddingVertical: theme.spacing.lg,
    paddingHorizontal: theme.spacing.xl,
  },
  continueText: {
    ...theme.styles.titleSmall,
    textAlign: 'center',
    color: theme.text,
  },
  footnote: {
    ...theme.styles.textMuted,
    fontSize: 12,
    textAlign: 'center',
  },
}); 