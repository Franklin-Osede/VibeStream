import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { secureStorage } from '../security/SecureStorage';
import { secureAPIClient } from '../security/SecureAPIClient';

// =============================================================================
// TIPOS
// =============================================================================

export interface User {
  id: string;
  email: string;
  username: string;
  user_type: 'artist' | 'fan' | 'investor';
  profile_image_url?: string;
  verified: boolean;
  created_at: string;
}

export interface LoginCredentials {
  email: string;
  password: string;
}

export interface RegisterData {
  email: string;
  password: string;
  username: string;
  user_type: 'artist' | 'fan' | 'investor';
}

export interface AuthContextType {
  // Estado
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  isInitialized: boolean;
  
  // Acciones
  login: (credentials: LoginCredentials) => Promise<void>;
  register: (data: RegisterData) => Promise<void>;
  logout: () => Promise<void>;
  
  // Utilidades
  checkAuthStatus: () => Promise<void>;
}

// =============================================================================
// CONTEXTO
// =============================================================================

const AuthContext = createContext<AuthContextType | undefined>(undefined);

// =============================================================================
// PROVIDER
// =============================================================================

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [isInitialized, setIsInitialized] = useState(false);

  // =============================================================================
  // INICIALIZACIÓN
  // =============================================================================

  useEffect(() => {
    initializeAuth();
  }, []);

  const initializeAuth = async () => {
    try {
      setIsLoading(true);
      
      // Verificar si hay un token almacenado
      const token = await secureStorage.getAccessToken();
      
      if (token) {
        // Intentar obtener el perfil del usuario
        await checkAuthStatus();
      }
    } catch (error) {
      console.error('Auth initialization error:', error);
      // Limpiar datos corruptos
      await secureStorage.clearSecureData();
    } finally {
      setIsLoading(false);
      setIsInitialized(true);
    }
  };

  // =============================================================================
  // MÉTODOS DE AUTENTICACIÓN
  // =============================================================================

  const login = async (credentials: LoginCredentials): Promise<void> => {
    try {
      setIsLoading(true);
      
      const { user: userData, token } = await secureAPIClient.login(
        credentials.email,
        credentials.password
      );

      setUser(userData);
      setIsAuthenticated(true);
      
      // Guardar credenciales para "Remember me"
      await secureStorage.storeUserCredentials(credentials.email, credentials.password);
      
    } catch (error) {
      console.error('Login error:', error);
      throw new Error('Login failed. Please check your credentials.');
    } finally {
      setIsLoading(false);
    }
  };

  const register = async (data: RegisterData): Promise<void> => {
    try {
      setIsLoading(true);
      
      const { user: userData, token } = await secureAPIClient.register(data);

      setUser(userData);
      setIsAuthenticated(true);
      
      // Guardar credenciales
      await secureStorage.storeUserCredentials(data.email, data.password);
      
    } catch (error) {
      console.error('Register error:', error);
      throw new Error('Registration failed. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const logout = async (): Promise<void> => {
    try {
      setIsLoading(true);
      
      await secureAPIClient.logout();
      
      setUser(null);
      setIsAuthenticated(false);
      
    } catch (error) {
      console.error('Logout error:', error);
      // Aún así, limpiar el estado local
      setUser(null);
      setIsAuthenticated(false);
    } finally {
      setIsLoading(false);
    }
  };

  const checkAuthStatus = async (): Promise<void> => {
    try {
      const userProfile = await secureAPIClient.getUserProfile();
      setUser(userProfile);
      setIsAuthenticated(true);
    } catch (error) {
      console.error('Auth status check error:', error);
      // Si falla, limpiar el estado
      setUser(null);
      setIsAuthenticated(false);
      await secureStorage.clearSecureData();
    }
  };

  // =============================================================================
  // VALOR DEL CONTEXTO
  // =============================================================================

  const value: AuthContextType = {
    user,
    isAuthenticated,
    isLoading,
    isInitialized,
    login,
    register,
    logout,
    checkAuthStatus,
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
};

// =============================================================================
// HOOK PERSONALIZADO
// =============================================================================

export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext);
  
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  
  return context;
};

// =============================================================================
// UTILIDADES
// =============================================================================

/**
 * Hook para verificar si el usuario está autenticado
 */
export const useIsAuthenticated = (): boolean => {
  const { isAuthenticated } = useAuth();
  return isAuthenticated;
};

/**
 * Hook para obtener el usuario actual
 */
export const useCurrentUser = (): User | null => {
  const { user } = useAuth();
  return user;
};

/**
 * Hook para verificar si la autenticación está cargando
 */
export const useAuthLoading = (): boolean => {
  const { isLoading } = useAuth();
  return isLoading;
};
