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
import { useGoogleAuth, useMicrosoftAuth, GoogleUser } from '../../infrastructure/auth/GoogleAuth';

// Localization imports
import { useTranslation } from '../../localization/hooks/useTranslation';
import { LanguageSelector } from '../../localization/components/LanguageSelector';

// Theme imports
import { useTheme } from '../../theme';

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
        {
          position: 'absolute',
          zIndex: 1,
          opacity: fadeAnim,
          transform: [{ translateY }],
          left: Math.random() * width,
          top: height * 0.2 + Math.random() * 200,
        },
      ]}
    >
      <Text style={{
        fontSize: 20,
        color: '#6C5CE7',
        opacity: 0.7,
      }}>♪</Text>
    </Animated.View>
  );
};

// Función para crear estilos usando el tema
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
  keyboardContainer: {
    flex: 1,
    justifyContent: 'center',
    padding: theme.spacing.lg,
  },
  header: {
    alignItems: 'center',
    marginBottom: theme.spacing.xxl + theme.spacing.md,
    zIndex: 2,
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
  title: {
    ...theme.styles.titleLarge,
    textShadowColor: `${theme.primary}80`,
    textShadowOffset: { width: 0, height: 2 },
    textShadowRadius: 10,
    letterSpacing: 1,
  },
  subtitle: {
    ...theme.styles.titleSmall,
    color: theme.textSecondary,
    fontWeight: '300',
    marginBottom: theme.spacing.lg,
  },
  languageButton: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: theme.colors.glassLight,
    paddingHorizontal: theme.spacing.md,
    paddingVertical: theme.spacing.sm,
    borderRadius: theme.borderRadius.xl,
    marginTop: theme.spacing.sm,
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
  },
  languageFlag: {
    fontSize: 16,
    marginRight: theme.spacing.sm,
  },
  languageText: {
    color: theme.textSecondary,
    fontSize: 12,
    fontWeight: '500',
  },
  musicWave: {
    flexDirection: 'row',
    alignItems: 'flex-end',
    height: 20,
    gap: 3,
  },
  waveBar: {
    width: 3,
    backgroundColor: theme.primary,
    borderRadius: 2,
    opacity: 0.8,
  },
  form: {
    width: '100%',
    zIndex: 2,
  },
  glassContainer: {
    backgroundColor: theme.colors.glassLight,
    borderRadius: theme.borderRadius.xl,
    padding: theme.spacing.xl,
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
    ...theme.shadows.lg,
  },
  input: {
    backgroundColor: theme.colors.glassLight,
    borderRadius: theme.borderRadius.md,
    padding: theme.spacing.lg,
    marginBottom: theme.spacing.lg,
    color: theme.text,
    fontSize: 16,
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
    fontWeight: '500',
    ...theme.shadows.sm,
  },
  buttonContainer: {
    borderRadius: theme.borderRadius.md,
    marginTop: theme.spacing.sm,
    ...theme.shadows.lg,
    shadowColor: theme.primary,
  },
  button: {
    borderRadius: theme.borderRadius.md,
    padding: theme.spacing.lg,
    alignItems: 'center',
  },
  buttonDisabled: {
    shadowOpacity: 0.2,
  },
  buttonText: {
    color: theme.text,
    fontSize: 18,
    fontWeight: '700',
    letterSpacing: 0.5,
  },
  toggleButton: {
    marginTop: theme.spacing.lg + theme.spacing.sm,
    alignItems: 'center',
  },
  toggleText: {
    color: theme.colors.primaryLight,
    fontSize: 16,
    fontWeight: '500',
  },
  footer: {
    alignItems: 'center',
    marginTop: theme.spacing.xl + theme.spacing.sm,
    zIndex: 2,
  },
  footerGradient: {
    paddingHorizontal: theme.spacing.lg,
    paddingVertical: theme.spacing.sm,
    borderRadius: theme.borderRadius.xl,
  },
  footerText: {
    color: theme.text,
    fontSize: 14,
    fontWeight: '600',
    opacity: 0.9,
  },
  oauthSection: {
    marginBottom: theme.spacing.lg,
  },
  oauthTitle: {
    ...theme.styles.titleSmall,
    color: theme.textSecondary,
    fontWeight: '300',
    marginBottom: theme.spacing.md,
    textAlign: 'center',
  },
  oauthButton: {
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.sm,
    ...theme.shadows.lg,
  },
  oauthIcon: {
    fontSize: 20,
  },
  divider: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: theme.spacing.md,
  },
  dividerLine: {
    flex: 1,
    height: 1,
    backgroundColor: theme.textMuted,
  },
  dividerText: {
    marginHorizontal: theme.spacing.md,
    color: theme.textMuted,
    fontSize: 14,
    fontWeight: '500',
  },
  oauthText: {
    color: theme.text,
    fontSize: 16,
    fontWeight: '500',
  },
  googleButton: {
    borderRadius: theme.borderRadius.lg,
    overflow: 'hidden',
    ...theme.shadows.md,
  },
  microsoftButton: {
    borderRadius: theme.borderRadius.lg,
    overflow: 'hidden',
    ...theme.shadows.md,
  },
  oauthGradient: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.lg,
  },
  googleIcon: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#db4437',
    marginRight: theme.spacing.sm,
  },
  microsoftIcon: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#fff',
    marginRight: theme.spacing.sm,
  },
  oauthButtonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: '600',
  },
});

export default function LoginScreen({ navigation }: any) {
  const [isLogin, setIsLogin] = useState(true);
  const [email, setEmail] = useState('');
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [showLanguageSelector, setShowLanguageSelector] = useState(false);

  // Traducciones
  const { t, getCurrentLanguageInfo } = useTranslation();
  
  // Tema
  const theme = useTheme();
  const styles = createStyles(theme);

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
            onPress: () => navigation.replace('Main', { user: result.user })
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
            onPress: () => navigation.replace('Main', { user: result.user })
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

  const handleMockOAuth = async (provider: 'google' | 'microsoft') => {
    setLoading(true);
    try {
      // Simular delay de OAuth
      await new Promise(resolve => setTimeout(resolve, 1500));
      
      // Datos mock del usuario según provider
      const mockUser = {
        id: `${provider}_${Date.now()}`,
        email: provider === 'google' ? 'usuario@gmail.com' : 'usuario@outlook.com',
        username: provider === 'google' ? 'GoogleUser' : 'MicrosoftUser',
        name: provider === 'google' ? 'Usuario Google' : 'Usuario Microsoft',
        provider: provider,
        profilePicture: provider === 'google' 
          ? 'https://lh3.googleusercontent.com/a/default-user=s96-c' 
          : 'https://graph.microsoft.com/v1.0/me/photo/$value',
        role: 'user'
      };

      Alert.alert(
        'Bienvenido',
        `¡Hola ${mockUser.name}! Te registraste con ${provider === 'google' ? 'Google' : 'Microsoft'}`,
        [
          {
            text: 'Continuar',
            onPress: () => navigation.replace('Onboarding', { user: mockUser })
          }
        ]
      );
    } catch (error) {
      Alert.alert('Error', `No se pudo iniciar sesión con ${provider}`);
    } finally {
      setLoading(false);
    }
  };

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
              colors={[theme.primary, theme.colors.primaryLight, theme.colors.accentPink]}
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
              placeholderTextColor={theme.textMuted}
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
                placeholderTextColor={theme.textMuted}
                value={username}
                onChangeText={setUsername}
                autoCapitalize="none"
                autoCorrect={false}
              />
            )}

            <TextInput
              style={styles.input}
              placeholder={t('auth.login.password')}
              placeholderTextColor={theme.textMuted}
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
                colors={loading ? [theme.colors.textDisabled, theme.colors.textMuted] : [theme.primary, theme.colors.primaryLight]}
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

            {/* OAuth Buttons - Mock por ahora */}
            <View style={styles.oauthSection}>
              <View style={styles.divider}>
                <View style={styles.dividerLine} />
                <Text style={styles.dividerText}>o continúa con</Text>
                <View style={styles.dividerLine} />
              </View>

                             <TouchableOpacity 
                 style={[styles.oauthButton, styles.googleButton]}
                 onPress={() => handleMockOAuth('google')}
                 disabled={loading}
               >
                 <LinearGradient
                   colors={['#db4437', '#f4b400']}
                   style={styles.oauthGradient}
                   start={{ x: 0, y: 0 }}
                   end={{ x: 1, y: 0 }}
                 >
                   <Text style={styles.googleIcon}>G</Text>
                   <Text style={styles.oauthButtonText}>Continuar con Google</Text>
                 </LinearGradient>
               </TouchableOpacity>

               <TouchableOpacity 
                 style={[styles.oauthButton, styles.microsoftButton]}
                 onPress={() => handleMockOAuth('microsoft')}
                 disabled={loading}
               >
                 <LinearGradient
                   colors={['#0078d4', '#40e0d0']}
                   style={styles.oauthGradient}
                   start={{ x: 0, y: 0 }}
                   end={{ x: 1, y: 0 }}
                 >
                   <Text style={styles.microsoftIcon}>⊞</Text>
                   <Text style={styles.oauthButtonText}>Continuar con Microsoft</Text>
                 </LinearGradient>
               </TouchableOpacity>
            </View>
          </View>
        </Animated.View>

        {/* Footer mejorado */}
        <View style={styles.footer}>
          <LinearGradient
            colors={[theme.primary, theme.colors.primaryLight]}
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

 