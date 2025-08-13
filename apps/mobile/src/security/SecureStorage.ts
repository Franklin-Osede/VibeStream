import AsyncStorage from '@react-native-async-storage/async-storage';
import * as Keychain from 'react-native-keychain';

/**
 * Sistema de almacenamiento seguro básico
 * - AsyncStorage para datos no sensibles
 * - Keychain para datos sensibles (tokens, credenciales)
 */
export class SecureStorage {
  private static instance: SecureStorage;
  
  private constructor() {}
  
  static getInstance(): SecureStorage {
    if (!SecureStorage.instance) {
      SecureStorage.instance = new SecureStorage();
    }
    return SecureStorage.instance;
  }

  // =============================================================================
  // ALMACENAMIENTO SEGURO (Keychain)
  // =============================================================================

  /**
   * Guardar token de acceso de forma segura
   */
  async storeAccessToken(token: string): Promise<void> {
    try {
      await Keychain.setInternetCredentials(
        'vibestream_access_token',
        'user',
        token
      );
    } catch (error) {
      console.error('Error storing access token:', error);
      throw new Error('Failed to store access token securely');
    }
  }

  /**
   * Obtener token de acceso de forma segura
   */
  async getAccessToken(): Promise<string | null> {
    try {
      const credentials = await Keychain.getInternetCredentials('vibestream_access_token');
      return credentials ? credentials.password : null;
    } catch (error) {
      console.error('Error getting access token:', error);
      return null;
    }
  }

  /**
   * Guardar credenciales de usuario de forma segura
   */
  async storeUserCredentials(email: string, password: string): Promise<void> {
    try {
      await Keychain.setInternetCredentials(
        'vibestream_user_credentials',
        email,
        password
      );
    } catch (error) {
      console.error('Error storing user credentials:', error);
      throw new Error('Failed to store user credentials securely');
    }
  }

  /**
   * Obtener credenciales de usuario de forma segura
   */
  async getUserCredentials(): Promise<{ email: string; password: string } | null> {
    try {
      const credentials = await Keychain.getInternetCredentials('vibestream_user_credentials');
      return credentials ? { email: credentials.username, password: credentials.password } : null;
    } catch (error) {
      console.error('Error getting user credentials:', error);
      return null;
    }
  }

  /**
   * Limpiar todos los datos sensibles
   */
  async clearSecureData(): Promise<void> {
    try {
      // Usar setInternetCredentials con valores vacíos para "limpiar"
      try {
        await Keychain.setInternetCredentials('vibestream_access_token', 'user', '');
      } catch (e) {
        // Ignorar errores si no existen
      }
      
      try {
        await Keychain.setInternetCredentials('vibestream_user_credentials', 'user', '');
      } catch (e) {
        // Ignorar errores si no existen
      }
    } catch (error) {
      console.error('Error clearing secure data:', error);
    }
  }

  // =============================================================================
  // ALMACENAMIENTO LOCAL (AsyncStorage)
  // =============================================================================

  /**
   * Guardar datos no sensibles
   */
  async storeData(key: string, value: any): Promise<void> {
    try {
      const jsonValue = JSON.stringify(value);
      await AsyncStorage.setItem(key, jsonValue);
    } catch (error) {
      console.error('Error storing data:', error);
      throw new Error('Failed to store data');
    }
  }

  /**
   * Obtener datos no sensibles
   */
  async getData<T>(key: string): Promise<T | null> {
    try {
      const jsonValue = await AsyncStorage.getItem(key);
      return jsonValue != null ? JSON.parse(jsonValue) : null;
    } catch (error) {
      console.error('Error getting data:', error);
      return null;
    }
  }

  /**
   * Eliminar datos específicos
   */
  async removeData(key: string): Promise<void> {
    try {
      await AsyncStorage.removeItem(key);
    } catch (error) {
      console.error('Error removing data:', error);
    }
  }

  /**
   * Limpiar todos los datos locales
   */
  async clearAllData(): Promise<void> {
    try {
      await AsyncStorage.clear();
    } catch (error) {
      console.error('Error clearing all data:', error);
    }
  }

  // =============================================================================
  // UTILIDADES DE SEGURIDAD
  // =============================================================================

  /**
   * Verificar si el dispositivo es seguro
   */
  async isDeviceSecure(): Promise<boolean> {
    try {
      const biometryType = await Keychain.getSupportedBiometryType();
      return biometryType !== null;
    } catch (error) {
      console.error('Error checking device security:', error);
      return false;
    }
  }

  /**
   * Obtener tipo de biometría disponible
   */
  async getBiometryType(): Promise<string | null> {
    try {
      const biometryType = await Keychain.getSupportedBiometryType();
      return biometryType;
    } catch (error) {
      console.error('Error getting biometry type:', error);
      return null;
    }
  }
}

// Exportar instancia singleton
export const secureStorage = SecureStorage.getInstance();
