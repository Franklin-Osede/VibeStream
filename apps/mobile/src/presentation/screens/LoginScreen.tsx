import React, { useState, useRef, useEffect } from 'react';
import {
  View,
  Text,
  TextInput,
  TouchableOpacity,
  StyleSheet,
  Alert,
  ActivityIndicator,
  KeyboardAvoidingView,
  Platform,
  Animated,
  Dimensions,
} from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { LinearGradient } from 'expo-linear-gradient';

// Domain layer imports
import { AuthenticateUser, RegisterUser } from '../../application/usecases/AuthenticateUser';
import { UserRepositoryImpl } from '../../infrastructure/api/UserRepositoryImpl';
import { ApiClient } from '../../infrastructure/api/ApiClient';

// Localization imports
import { useTranslation } from '../../localization/hooks/useTranslation';
import { LanguageSelector } from '../../localization/components/LanguageSelector';

const { width, height } = Dimensions.get('window');

// Componente de partículas musicales animadas
const MusicParticle = ({ index }: { index: number }) => {
  const fadeAnim = useRef(new Animated.Value(0)).current;
  const translateY = useRef(new Animated.Value(0)).current;

  useEffect(() => {
    const startAnimation = () => {
      Animated.loop(
        Animated.sequence([
          Animated.timing(fadeAnim, {
            toValue: 1,
            duration: 2000 + index * 300,
            useNativeDriver: true,
          }),
          Animated.timing(translateY, {
            toValue: -50,
            duration: 3000,
            useNativeDriver: true,
          }),
          Animated.timing(fadeAnim, {
            toValue: 0,
            duration: 1000,
            useNativeDriver: true,
          }),
        ])
      ).start();
    };

    const timer = setTimeout(startAnimation, index * 1000);
    return () => clearTimeout(timer);
  }, []);

  return (
    <Animated.View
      style={[
        styles.musicParticle,
        {
          opacity: fadeAnim,
          transform: [{ translateY }],
          left: Math.random() * width,
          top: height * 0.2 + Math.random() * 200,
        },
      ]}
    >
      <Text style={styles.particleText}>♪</Text>
    </Animated.View>
  );
};

export default function LoginScreen({ navigation }: any) {
  const [isLogin, setIsLogin] = useState(true);
  const [email, setEmail] = useState('');
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [showLanguageSelector, setShowLanguageSelector] = useState(false);

  // Traducciones
  const { t, getCurrentLanguageInfo } = useTranslation();

  // Animaciones
  const logoScale = useRef(new Animated.Value(0.8)).current;
  const formTranslateY = useRef(new Animated.Value(50)).current;
  const formOpacity = useRef(new Animated.Value(0)).current;

  useEffect(() => {
    // Animación de entrada
    Animated.sequence([
      Animated.timing(logoScale, {
        toValue: 1,
        duration: 800,
        useNativeDriver: true,
      }),
      Animated.parallel([
        Animated.timing(formTranslateY, {
          toValue: 0,
          duration: 600,
          useNativeDriver: true,
        }),
        Animated.timing(formOpacity, {
          toValue: 1,
          duration: 600,
          useNativeDriver: true,
        }),
      ]),
    ]).start();
  }, []);

  // Initialize DDD layers
  const apiClient = new ApiClient();
  const userRepository = new UserRepositoryImpl(apiClient);
  const authenticateUser = new AuthenticateUser(userRepository);
  const registerUser = new RegisterUser(userRepository);

  const handleLogin = async () => {
    if (!email || !password) {
      Alert.alert(t('common.error'), t('auth.login.errors.emptyFields'));
      return;
    }

    setLoading(true);
    try {
      const result = await authenticateUser.execute({
        email: email.toLowerCase(),
        password
      });

      Alert.alert(
        t('auth.login.success'),
        t('auth.login.welcomeBack', { username: result.user.username }),
        [
          {
            text: 'OK',
            onPress: () => navigation.replace('Home', { user: result.user })
          }
        ]
      );
    } catch (error: any) {
      Alert.alert(t('common.error'), t('auth.login.errors.invalidCredentials'));
    } finally {
      setLoading(false);
    }
  };

  const handleRegister = async () => {
    if (!email || !username || !password) {
      Alert.alert(t('common.error'), t('auth.register.errors.emptyFields'));
      return;
    }

    if (password.length < 6) {
      Alert.alert(t('common.error'), t('auth.register.errors.passwordTooShort'));
      return;
    }

    setLoading(true);
    try {
      const result = await registerUser.execute({
        email: email.toLowerCase(),
        username,
        password,
        role: 'user'
      });

      Alert.alert(
        t('auth.register.success'),
        t('auth.register.accountCreated', { username: result.user.username }),
        [
          {
            text: 'OK',
            onPress: () => navigation.replace('Home', { user: result.user })
          }
        ]
      );
    } catch (error: any) {
      Alert.alert(t('common.error'), t('auth.register.errors.serverError'));
    } finally {
      setLoading(false);
    }
  };

  const toggleMode = () => {
    setIsLogin(!isLogin);
    setEmail('');
    setUsername('');
    setPassword('');
  };

  return (
    <View style={styles.container}>
      <StatusBar style="light" />
      
      {/* Fondo con gradiente */}
      <LinearGradient
        colors={['#0f0f23', '#16213e', '#1a1a2e', '#0f3460']}
        style={styles.background}
        start={{ x: 0, y: 0 }}
        end={{ x: 1, y: 1 }}
      />

      {/* Partículas musicales animadas */}
      {[...Array(6)].map((_, index) => (
        <MusicParticle key={index} index={index} />
      ))}

      <KeyboardAvoidingView 
        style={styles.keyboardContainer}
        behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
      >
        {/* Header con logo animado */}
        <Animated.View 
          style={[
            styles.header,
            {
              transform: [{ scale: logoScale }]
            }
          ]}
        >
          <View style={styles.logoContainer}>
            <LinearGradient
              colors={['#6c5ce7', '#a29bfe', '#fd79a8']}
              style={styles.logoGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <Text style={styles.logoIcon}>🎵</Text>
            </LinearGradient>
            <Text style={styles.title}>{t('app.name')}</Text>
          </View>
          <Text style={styles.subtitle}>
            {isLogin ? t('auth.login.subtitle') : t('auth.register.subtitle')}
          </Text>
          
          {/* Selector de idioma */}
          <TouchableOpacity 
            style={styles.languageButton}
            onPress={() => setShowLanguageSelector(true)}
          >
            <Text style={styles.languageFlag}>{getCurrentLanguageInfo().flag}</Text>
            <Text style={styles.languageText}>{getCurrentLanguageInfo().name}</Text>
          </TouchableOpacity>
          <View style={styles.musicWave}>
            <View style={[styles.waveBar, { height: 4 }]} />
            <View style={[styles.waveBar, { height: 12 }]} />
            <View style={[styles.waveBar, { height: 8 }]} />
            <View style={[styles.waveBar, { height: 16 }]} />
            <View style={[styles.waveBar, { height: 6 }]} />
            <View style={[styles.waveBar, { height: 14 }]} />
            <View style={[styles.waveBar, { height: 10 }]} />
          </View>
        </Animated.View>

        {/* Formulario con efectos glassmorphism */}
        <Animated.View 
          style={[
            styles.form,
            {
              opacity: formOpacity,
              transform: [{ translateY: formTranslateY }]
            }
          ]}
        >
          <View style={styles.glassContainer}>
            <TextInput
              style={styles.input}
              placeholder={t('auth.login.email')}
              placeholderTextColor="#9ca3af"
              value={email}
              onChangeText={setEmail}
              keyboardType="email-address"
              autoCapitalize="none"
              autoCorrect={false}
            />

            {!isLogin && (
              <TextInput
                style={styles.input}
                placeholder={t('auth.register.username')}
                placeholderTextColor="#9ca3af"
                value={username}
                onChangeText={setUsername}
                autoCapitalize="none"
                autoCorrect={false}
              />
            )}

            <TextInput
              style={styles.input}
              placeholder={t('auth.login.password')}
              placeholderTextColor="#9ca3af"
              value={password}
              onChangeText={setPassword}
              secureTextEntry
              autoCapitalize="none"
              autoCorrect={false}
            />

            <TouchableOpacity
              style={[styles.buttonContainer, loading && styles.buttonDisabled]}
              onPress={isLogin ? handleLogin : handleRegister}
              disabled={loading}
            >
              <LinearGradient
                colors={loading ? ['#666', '#888'] : ['#6c5ce7', '#a29bfe']}
                style={styles.button}
                start={{ x: 0, y: 0 }}
                end={{ x: 1, y: 0 }}
              >
                {loading ? (
                  <ActivityIndicator color="#fff" size="small" />
                ) : (
                  <Text style={styles.buttonText}>
                    {isLogin ? t('auth.login.button') : t('auth.register.button')}
                  </Text>
                )}
              </LinearGradient>
            </TouchableOpacity>

            <TouchableOpacity onPress={toggleMode} style={styles.toggleButton}>
              <Text style={styles.toggleText}>
                {isLogin 
                  ? t('auth.login.switchToRegister')
                  : t('auth.register.switchToLogin')
                }
              </Text>
            </TouchableOpacity>
          </View>
        </Animated.View>

        {/* Footer mejorado */}
        <View style={styles.footer}>
          <LinearGradient
            colors={['#6c5ce7', '#a29bfe']}
            style={styles.footerGradient}
            start={{ x: 0, y: 0 }}
            end={{ x: 1, y: 0 }}
          >
            <Text style={styles.footerText}>
              {t('app.tagline')}
            </Text>
          </LinearGradient>
        </View>
      </KeyboardAvoidingView>

      {/* Selector de idioma */}
      <LanguageSelector
        visible={showLanguageSelector}
        onClose={() => setShowLanguageSelector(false)}
        onLanguageChange={() => {
          // El cambio de idioma se maneja automáticamente
          console.log('Language changed');
        }}
      />
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
  keyboardContainer: {
    flex: 1,
    justifyContent: 'center',
    padding: 20,
  },
  musicParticle: {
    position: 'absolute',
    zIndex: 1,
  },
  particleText: {
    fontSize: 20,
    color: '#6c5ce7',
    opacity: 0.7,
  },
  header: {
    alignItems: 'center',
    marginBottom: 50,
    zIndex: 2,
  },
  logoContainer: {
    alignItems: 'center',
    marginBottom: 20,
  },
  logoGradient: {
    width: 80,
    height: 80,
    borderRadius: 40,
    justifyContent: 'center',
    alignItems: 'center',
    marginBottom: 15,
    shadowColor: '#6c5ce7',
    shadowOffset: {
      width: 0,
      height: 8,
    },
    shadowOpacity: 0.44,
    shadowRadius: 10.32,
    elevation: 16,
  },
  logoIcon: {
    fontSize: 40,
    color: '#fff',
  },
  title: {
    fontSize: 36,
    fontWeight: '800',
    color: '#fff',
    textShadowColor: 'rgba(108, 92, 231, 0.5)',
    textShadowOffset: { width: 0, height: 2 },
    textShadowRadius: 10,
    letterSpacing: 1,
  },
  subtitle: {
    fontSize: 20,
    color: '#d1d5db',
    fontWeight: '300',
    marginBottom: 20,
  },
  musicWave: {
    flexDirection: 'row',
    alignItems: 'flex-end',
    height: 20,
    gap: 3,
  },
  waveBar: {
    width: 3,
    backgroundColor: '#6c5ce7',
    borderRadius: 2,
    opacity: 0.8,
  },
  form: {
    width: '100%',
    zIndex: 2,
  },
  glassContainer: {
    backgroundColor: 'rgba(255, 255, 255, 0.08)',
    borderRadius: 24,
    padding: 30,
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.1)',
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 8,
    },
    shadowOpacity: 0.44,
    shadowRadius: 10.32,
    elevation: 16,
  },
  input: {
    backgroundColor: 'rgba(255, 255, 255, 0.1)',
    borderRadius: 16,
    padding: 18,
    marginBottom: 20,
    color: '#fff',
    fontSize: 16,
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.1)',
    fontWeight: '500',
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.25,
    shadowRadius: 3.84,
    elevation: 5,
  },
  buttonContainer: {
    borderRadius: 16,
    marginTop: 10,
    shadowColor: '#6c5ce7',
    shadowOffset: {
      width: 0,
      height: 8,
    },
    shadowOpacity: 0.44,
    shadowRadius: 10.32,
    elevation: 16,
  },
  button: {
    borderRadius: 16,
    padding: 18,
    alignItems: 'center',
  },
  buttonDisabled: {
    shadowOpacity: 0.2,
  },
  buttonText: {
    color: '#fff',
    fontSize: 18,
    fontWeight: '700',
    letterSpacing: 0.5,
  },
  toggleButton: {
    marginTop: 25,
    alignItems: 'center',
  },
  toggleText: {
    color: '#a29bfe',
    fontSize: 16,
    fontWeight: '500',
  },
  footer: {
    alignItems: 'center',
    marginTop: 40,
    zIndex: 2,
  },
  footerGradient: {
    paddingHorizontal: 20,
    paddingVertical: 8,
    borderRadius: 20,
  },
  footerText: {
    color: '#fff',
    fontSize: 14,
    fontWeight: '600',
    opacity: 0.9,
  },
  languageButton: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: 'rgba(255, 255, 255, 0.1)',
    paddingHorizontal: 12,
    paddingVertical: 6,
    borderRadius: 20,
    marginTop: 10,
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.2)',
  },
  languageFlag: {
    fontSize: 16,
    marginRight: 6,
  },
  languageText: {
    color: '#d1d5db',
    fontSize: 12,
    fontWeight: '500',
  },
}); 