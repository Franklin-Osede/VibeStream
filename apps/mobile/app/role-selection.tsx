import React, { useState } from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  Alert,
  ActivityIndicator,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { StatusBar } from 'expo-status-bar';

export default function RoleSelectionScreen({ navigation, route }: any) {
  const [selectedRole, setSelectedRole] = useState<'artist' | 'fan' | null>(null);
  const [loading, setLoading] = useState(false);
  
  const { user, token } = route.params;

  const handleRoleSelection = async (role: 'artist' | 'fan') => {
    setLoading(true);
    
    try {
      // Aquí podrías actualizar el rol del usuario en la base de datos
      // await userRepository.updateRole(user.id, role);
      
      // Navegar a la pantalla correspondiente
      if (role === 'artist') {
        navigation.navigate('ArtistDashboard', { 
          user: user,
          token: token
        });
      } else {
        navigation.navigate('FanDashboard', { 
          user: user,
          token: token
        });
      }
    } catch (error) {
      Alert.alert('Error', 'No se pudo establecer tu rol. Inténtalo de nuevo.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <View style={styles.container}>
      <StatusBar style="light" />
      
      <LinearGradient
        colors={['#6366f1', '#8b5cf6', '#ec4899']}
        style={styles.background}
        start={{ x: 0, y: 0 }}
        end={{ x: 1, y: 1 }}
      />

      <View style={styles.content}>
        {/* Header */}
        <View style={styles.header}>
          <View style={styles.logoContainer}>
            <LinearGradient
              colors={['#6366f1', '#8b5cf6', '#ec4899']}
              style={styles.logoGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <Text style={styles.logoIcon}>🎵</Text>
            </LinearGradient>
          </View>
          
          <Text style={styles.welcomeText}>
            ¡Bienvenido, {user.username}!
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
                  ? ['#6366f1', '#8b5cf6']
                  : ['rgba(255, 255, 255, 0.1)', 'rgba(255, 255, 255, 0.05)']
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
                  ? ['#ec4899', '#8b5cf6']
                  : ['rgba(255, 255, 255, 0.1)', 'rgba(255, 255, 255, 0.05)']
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
          <View style={styles.buttonContainer}>
            <TouchableOpacity
              style={styles.continueButton}
              onPress={() => handleRoleSelection(selectedRole)}
              disabled={loading}
            >
              <LinearGradient
                colors={['#6366f1', '#8b5cf6']}
                style={styles.buttonGradient}
                start={{ x: 0, y: 0 }}
                end={{ x: 1, y: 0 }}
              >
                {loading ? (
                  <ActivityIndicator color="#fff" size="small" />
                ) : (
                  <Text style={styles.buttonText}>
                    {loading ? 'Configurando...' : 'Continuar'}
                  </Text>
                )}
              </LinearGradient>
            </TouchableOpacity>
          </View>
        )}
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
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
    padding: 20,
    justifyContent: 'center',
  },
  header: {
    alignItems: 'center',
    marginBottom: 40,
  },
  logoContainer: {
    marginBottom: 16,
  },
  logoGradient: {
    width: 80,
    height: 80,
    borderRadius: 40,
    justifyContent: 'center',
    alignItems: 'center',
  },
  logoIcon: {
    fontSize: 40,
  },
  welcomeText: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#fff',
    marginBottom: 8,
    textAlign: 'center',
  },
  subtitle: {
    fontSize: 16,
    color: '#e2e8f0',
    textAlign: 'center',
  },
  roleContainer: {
    marginBottom: 30,
  },
  roleCard: {
    marginBottom: 16,
    borderRadius: 16,
    overflow: 'hidden',
  },
  selectedCard: {
    transform: [{ scale: 1.02 }],
  },
  cardGradient: {
    padding: 20,
  },
  cardContent: {
    alignItems: 'center',
  },
  roleIcon: {
    fontSize: 48,
    marginBottom: 12,
  },
  roleTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#e2e8f0',
    marginBottom: 8,
    textAlign: 'center',
  },
  selectedRoleTitle: {
    color: '#fff',
  },
  roleDescription: {
    fontSize: 14,
    color: '#94a3b8',
    textAlign: 'center',
    lineHeight: 20,
  },
  selectedRoleDescription: {
    color: '#e2e8f0',
  },
  buttonContainer: {
    alignItems: 'center',
  },
  continueButton: {
    width: '100%',
    borderRadius: 12,
    overflow: 'hidden',
  },
  buttonGradient: {
    padding: 16,
    alignItems: 'center',
    justifyContent: 'center',
  },
  buttonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: '600',
  },
}); 