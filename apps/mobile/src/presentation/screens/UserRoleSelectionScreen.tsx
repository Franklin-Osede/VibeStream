import React, { useState } from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  StyleSheet,
  Animated,
  Dimensions,
  Alert,
} from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { LinearGradient } from 'expo-linear-gradient';
// Removed translation import - using simple text for demo
// import { useTranslation } from '../../localization/hooks/useTranslation';
import { useTheme } from '../../theme';

const { width, height } = Dimensions.get('window');

interface UserRoleSelectionScreenProps {
  navigation: any;
  route: {
    params: {
      user: any;
      token: string;
    };
  };
}

const UserRoleSelectionScreen: React.FC<UserRoleSelectionScreenProps> = ({ 
  navigation, 
  route 
}) => {
  const [selectedRole, setSelectedRole] = useState<'artist' | 'fan' | null>(null);
  const [loading, setLoading] = useState(false);
  // Removed translation hook - using simple text for demo
  // const { t } = useTranslation();
  const theme = useTheme();

  const handleRoleSelection = async (role: 'artist' | 'fan') => {
    setLoading(true);
    
    try {
      // Aquí podrías actualizar el rol del usuario en la base de datos
      // await userRepository.updateRole(route.params.user.id, role);
      
      // Navegar a la pantalla correspondiente
      if (role === 'artist') {
        navigation.replace('ArtistDashboard', { 
          user: route.params.user,
          token: route.params.token
        });
      } else {
        navigation.replace('FanDashboard', { 
          user: route.params.user,
          token: route.params.token
        });
      }
    } catch (error) {
      Alert.alert('Error', 'No se pudo establecer tu rol. Inténtalo de nuevo.');
    } finally {
      setLoading(false);
    }
  };

  const styles = createStyles(theme);

  return (
    <View style={styles.container}>
      <StatusBar style="light" />
      
      {/* Fondo con gradiente */}
      <LinearGradient
        colors={theme.gradients.background}
        style={styles.background}
        start={{ x: 0, y: 0 }}
        end={{ x: 1, y: 1 }}
      />

      <View style={styles.content}>
        {/* Header */}
        <View style={styles.header}>
          <View style={styles.logoContainer}>
            <LinearGradient
              colors={[theme.primary, theme.colors.primaryLight, theme.colors.accentPink]}
              style={styles.logoGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <Text style={styles.logoIcon}>🎵</Text>
            </LinearGradient>
          </View>
          
          <Text style={styles.welcomeText}>
            ¡Bienvenido, {route.params.user.username}!
          </Text>
          
          <Text style={styles.subtitle}>
            Para personalizar tu experiencia, dinos qué eres:
          </Text>
        </View>

        {/* Role Selection Cards */}
        <View style={styles.roleContainer}>
          {/* Artist Card */}
          <TouchableOpacity
            style={[
              styles.roleCard,
              selectedRole === 'artist' && styles.selectedCard
            ]}
            onPress={() => setSelectedRole('artist')}
            disabled={loading}
          >
            <LinearGradient
              colors={
                selectedRole === 'artist' 
                  ? [theme.primary, theme.colors.primaryLight]
                  : [theme.colors.glassLight, theme.colors.glassMedium]
              }
              style={styles.cardGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <View style={styles.cardContent}>
                <Text style={styles.roleIcon}>🎤</Text>
                <Text style={[
                  styles.roleTitle,
                  selectedRole === 'artist' && styles.selectedRoleTitle
                ]}>
                  Soy Artista
                </Text>
                <Text style={[
                  styles.roleDescription,
                  selectedRole === 'artist' && styles.selectedRoleDescription
                ]}>
                  • Sube tu música a la blockchain{'\n'}
                  • Crea campañas NFT promocionales{'\n'}
                  • Vende participaciones de tus canciones{'\n'}
                  • Recibe royalties automáticamente
                </Text>
              </View>
            </LinearGradient>
          </TouchableOpacity>

          {/* Fan Card */}
          <TouchableOpacity
            style={[
              styles.roleCard,
              selectedRole === 'fan' && styles.selectedCard
            ]}
            onPress={() => setSelectedRole('fan')}
            disabled={loading}
          >
            <LinearGradient
              colors={
                selectedRole === 'fan' 
                  ? [theme.colors.accentPink, theme.colors.primaryLight]
                  : [theme.colors.glassLight, theme.colors.glassMedium]
              }
              style={styles.cardGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <View style={styles.cardContent}>
                <Text style={styles.roleIcon}>🎧</Text>
                <Text style={[
                  styles.roleTitle,
                  selectedRole === 'fan' && styles.selectedRoleTitle
                ]}>
                  Soy Fan
                </Text>
                <Text style={[
                  styles.roleDescription,
                  selectedRole === 'fan' && styles.selectedRoleDescription
                ]}>
                  • Escucha música y gana $VIBERS{'\n'}
                  • Invierte en participaciones de canciones{'\n'}
                  • Colecciona NFTs exclusivos{'\n'}
                  • Apoya a tus artistas favoritos
                </Text>
              </View>
            </LinearGradient>
          </TouchableOpacity>
        </View>

        {/* Continue Button */}
        {selectedRole && (
          <Animated.View style={styles.buttonContainer}>
            <TouchableOpacity
              style={styles.continueButton}
              onPress={() => handleRoleSelection(selectedRole)}
              disabled={loading}
            >
              <LinearGradient
                colors={[theme.primary, theme.colors.primaryLight]}
                style={styles.buttonGradient}
                start={{ x: 0, y: 0 }}
                end={{ x: 1, y: 0 }}
              >
                <Text style={styles.buttonText}>
                  {loading ? 'Configurando...' : 'Continuar'}
                </Text>
              </LinearGradient>
            </TouchableOpacity>
          </Animated.View>
        )}
      </View>
    </View>
  );
};

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
    padding: theme.spacing.lg,
    justifyContent: 'center',
  },
  header: {
    alignItems: 'center',
    marginBottom: theme.spacing.xxl,
  },
  logoContainer: {
    alignItems: 'center',
    marginBottom: theme.spacing.lg,
  },
  logoGradient: {
    width: 80,
    height: 80,
    borderRadius: 40,
    justifyContent: 'center',
    alignItems: 'center',
    marginBottom: theme.spacing.md,
    ...theme.shadows.lg,
    shadowColor: theme.primary,
  },
  logoIcon: {
    fontSize: 40,
    color: theme.text,
  },
  welcomeText: {
    ...theme.styles.titleLarge,
    textAlign: 'center',
    marginBottom: theme.spacing.md,
    color: theme.text,
  },
  subtitle: {
    ...theme.styles.titleSmall,
    textAlign: 'center',
    color: theme.textSecondary,
    lineHeight: 24,
  },
  roleContainer: {
    gap: theme.spacing.lg,
    marginBottom: theme.spacing.xxl,
  },
  roleCard: {
    borderRadius: theme.borderRadius.xl,
    ...theme.shadows.lg,
  },
  selectedCard: {
    transform: [{ scale: 1.02 }],
  },
  cardGradient: {
    borderRadius: theme.borderRadius.xl,
    padding: theme.spacing.xl,
    borderWidth: 2,
    borderColor: 'transparent',
  },
  cardContent: {
    alignItems: 'center',
  },
  roleIcon: {
    fontSize: 48,
    marginBottom: theme.spacing.md,
  },
  roleTitle: {
    ...theme.styles.titleMedium,
    color: theme.textSecondary,
    marginBottom: theme.spacing.sm,
    textAlign: 'center',
  },
  selectedRoleTitle: {
    color: theme.text,
    fontWeight: 'bold',
  },
  roleDescription: {
    fontSize: 14,
    color: theme.textMuted,
    textAlign: 'left',
    lineHeight: 22,
  },
  selectedRoleDescription: {
    color: theme.text,
  },
  buttonContainer: {
    alignItems: 'center',
  },
  continueButton: {
    width: '100%',
    borderRadius: theme.borderRadius.md,
    ...theme.shadows.lg,
    shadowColor: theme.primary,
  },
  buttonGradient: {
    paddingVertical: theme.spacing.lg,
    paddingHorizontal: theme.spacing.xl,
    borderRadius: theme.borderRadius.md,
    alignItems: 'center',
  },
  buttonText: {
    ...theme.styles.titleMedium,
    color: theme.text,
    fontWeight: 'bold',
  },
});

export default UserRoleSelectionScreen; 