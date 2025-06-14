import React, { useState } from 'react';
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
} from 'react-native';
import { StatusBar } from 'expo-status-bar';

// Domain layer imports
import { AuthenticateUser, RegisterUser } from '../../application/usecases/AuthenticateUser';
import { UserRepositoryImpl } from '../../infrastructure/api/UserRepositoryImpl';
import { ApiClient } from '../../infrastructure/api/ApiClient';

export default function LoginScreen({ navigation }: any) {
  const [isLogin, setIsLogin] = useState(true);
  const [email, setEmail] = useState('');
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);

  // Initialize DDD layers
  const apiClient = new ApiClient();
  const userRepository = new UserRepositoryImpl(apiClient);
  const authenticateUser = new AuthenticateUser(userRepository);
  const registerUser = new RegisterUser(userRepository);

  const handleLogin = async () => {
    if (!email || !password) {
      Alert.alert('Error', 'Por favor ingresa email y contraseña');
      return;
    }

    setLoading(true);
    try {
      const result = await authenticateUser.execute({
        email: email.toLowerCase(),
        password
      });

      Alert.alert(
        'Éxito',
        `¡Bienvenido ${result.user.username}!`,
        [
          {
            text: 'OK',
            onPress: () => navigation.replace('Home', { user: result.user })
          }
        ]
      );
    } catch (error: any) {
      Alert.alert('Error de Login', error.message);
    } finally {
      setLoading(false);
    }
  };

  const handleRegister = async () => {
    if (!email || !username || !password) {
      Alert.alert('Error', 'Por favor completa todos los campos');
      return;
    }

    if (password.length < 6) {
      Alert.alert('Error', 'La contraseña debe tener al menos 6 caracteres');
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
        'Registro Exitoso',
        `¡Cuenta creada para ${result.user.username}!`,
        [
          {
            text: 'OK',
            onPress: () => navigation.replace('Home', { user: result.user })
          }
        ]
      );
    } catch (error: any) {
      Alert.alert('Error de Registro', error.message);
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
    <KeyboardAvoidingView 
      style={styles.container}
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
    >
      <StatusBar style="light" />
      
      <View style={styles.header}>
        <Text style={styles.title}>🎵 VibeStream</Text>
        <Text style={styles.subtitle}>
          {isLogin ? 'Inicia sesión' : 'Crea tu cuenta'}
        </Text>
      </View>

      <View style={styles.form}>
        <TextInput
          style={styles.input}
          placeholder="Email"
          placeholderTextColor="#666"
          value={email}
          onChangeText={setEmail}
          keyboardType="email-address"
          autoCapitalize="none"
          autoCorrect={false}
        />

        {!isLogin && (
          <TextInput
            style={styles.input}
            placeholder="Nombre de usuario"
            placeholderTextColor="#666"
            value={username}
            onChangeText={setUsername}
            autoCapitalize="none"
            autoCorrect={false}
          />
        )}

        <TextInput
          style={styles.input}
          placeholder="Contraseña"
          placeholderTextColor="#666"
          value={password}
          onChangeText={setPassword}
          secureTextEntry
          autoCapitalize="none"
          autoCorrect={false}
        />

        <TouchableOpacity
          style={[styles.button, loading && styles.buttonDisabled]}
          onPress={isLogin ? handleLogin : handleRegister}
          disabled={loading}
        >
          {loading ? (
            <ActivityIndicator color="#fff" />
          ) : (
            <Text style={styles.buttonText}>
              {isLogin ? 'Iniciar Sesión' : 'Registrarse'}
            </Text>
          )}
        </TouchableOpacity>

        <TouchableOpacity onPress={toggleMode} style={styles.toggleButton}>
          <Text style={styles.toggleText}>
            {isLogin 
              ? '¿No tienes cuenta? Regístrate' 
              : '¿Ya tienes cuenta? Inicia sesión'
            }
          </Text>
        </TouchableOpacity>
      </View>

      <View style={styles.footer}>
        <Text style={styles.footerText}>
          Música descentralizada con blockchain
        </Text>
      </View>
    </KeyboardAvoidingView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#1a1a2e',
    justifyContent: 'center',
    padding: 20,
  },
  header: {
    alignItems: 'center',
    marginBottom: 50,
  },
  title: {
    fontSize: 32,
    fontWeight: 'bold',
    color: '#fff',
    marginBottom: 10,
  },
  subtitle: {
    fontSize: 18,
    color: '#aaa',
  },
  form: {
    width: '100%',
  },
  input: {
    backgroundColor: '#2d2d2d',
    borderRadius: 10,
    padding: 15,
    marginBottom: 15,
    color: '#fff',
    fontSize: 16,
    borderWidth: 1,
    borderColor: '#444',
  },
  button: {
    backgroundColor: '#6c5ce7',
    borderRadius: 10,
    padding: 15,
    alignItems: 'center',
    marginTop: 10,
  },
  buttonDisabled: {
    backgroundColor: '#666',
  },
  buttonText: {
    color: '#fff',
    fontSize: 18,
    fontWeight: 'bold',
  },
  toggleButton: {
    marginTop: 20,
    alignItems: 'center',
  },
  toggleText: {
    color: '#6c5ce7',
    fontSize: 16,
  },
  footer: {
    alignItems: 'center',
    marginTop: 50,
  },
  footerText: {
    color: '#666',
    fontSize: 14,
  },
}); 