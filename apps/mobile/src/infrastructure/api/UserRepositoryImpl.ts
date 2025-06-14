import { UserRepository, CreateUserData, LoginCredentials, AuthResult } from '../../domain/repositories/UserRepository';
import { User, UserProps } from '../../domain/entities/User';
import { ApiClient } from './ApiClient';

// Interfaces que coinciden EXACTAMENTE con el backend Rust
interface BackendUserInfo {
  id: string;
  username: string;
  email: string;
  role: string;
}

interface BackendLoginResponse {
  token: string;
  user: BackendUserInfo;
}

interface BackendUserResponse {
  id: string;
  username: string;
  email: string;
  role: string;
  created_at: string;
}

export class UserRepositoryImpl implements UserRepository {
  constructor(private apiClient: ApiClient) {}

  async register(userData: CreateUserData): Promise<AuthResult> {
    try {
      const response = await this.apiClient.post<BackendLoginResponse>('/auth/register', {
        email: userData.email,
        username: userData.username,
        password: userData.password,
        role: userData.role || 'user'
      });

      const userProps: UserProps = {
        id: response.user.id,
        email: response.user.email,
        username: response.user.username,
        role: response.user.role as 'user' | 'artist' | 'admin',
        walletAddress: undefined, // Backend no devuelve wallet_address en register
        isVerified: false,
        createdAt: new Date() // Backend no devuelve created_at en register
      };

      const user = User.create(userProps);
      
      // Save token
      await this.apiClient.setAuthToken(response.token);

      return {
        user,
        token: response.token
      };
    } catch (error: any) {
      throw new Error(`Registration failed: ${error.message}`);
    }
  }

  async login(credentials: LoginCredentials): Promise<AuthResult> {
    try {
      const response = await this.apiClient.post<BackendLoginResponse>('/auth/login', {
        email: credentials.email,
        password: credentials.password
      });

      const userProps: UserProps = {
        id: response.user.id,
        email: response.user.email,
        username: response.user.username,
        role: response.user.role as 'user' | 'artist' | 'admin',
        walletAddress: undefined, // Backend no devuelve wallet_address en login
        isVerified: false,
        createdAt: new Date() // Backend no devuelve created_at en login
      };

      const user = User.create(userProps);
      
      // Save token
      await this.apiClient.setAuthToken(response.token);

      return {
        user,
        token: response.token
      };
    } catch (error: any) {
      throw new Error(`Login failed: ${error.message}`);
    }
  }

  async logout(): Promise<void> {
    await this.apiClient.clearAuthToken();
  }

  async findById(id: string): Promise<User | null> {
    try {
      const response = await this.apiClient.get<BackendUserResponse>(`/users/${id}`);

      const userProps: UserProps = {
        id: response.id,
        email: response.email,
        username: response.username,
        role: response.role as 'user' | 'artist' | 'admin',
        walletAddress: undefined, // Backend no devuelve wallet_address aquí
        isVerified: false,
        createdAt: new Date(response.created_at)
      };

      return User.create(userProps);
    } catch (error: any) {
      if (error.status === 404) {
        return null;
      }
      throw error;
    }
  }

  async getCurrentUser(): Promise<User | null> {
    if (!this.apiClient.isAuthenticated()) {
      return null;
    }

    try {
      const response = await this.apiClient.get<BackendUserInfo>('/auth/profile');

      const userProps: UserProps = {
        id: response.id,
        email: response.email,
        username: response.username,
        role: response.role as 'user' | 'artist' | 'admin',
        walletAddress: undefined,
        isVerified: false,
        createdAt: new Date()
      };

      return User.create(userProps);
    } catch (error: any) {
      // Token might be expired
      if (error.status === 401) {
        await this.logout();
        return null;
      }
      throw error;
    }
  }

  async updateProfile(userId: string, updates: Partial<CreateUserData>): Promise<User> {
    const response = await this.apiClient.put<BackendUserResponse>(`/users/${userId}`, updates);

    const userProps: UserProps = {
      id: response.id,
      email: response.email,
      username: response.username,
      role: response.role as 'user' | 'artist' | 'admin',
      walletAddress: undefined,
      isVerified: false,
      createdAt: new Date(response.created_at)
    };

    return User.create(userProps);
  }

  async connectWallet(userId: string, walletAddress: string): Promise<User> {
    // Este endpoint no existe en el backend actual, simulation
    const response = await this.apiClient.post<BackendUserResponse>(`/users/${userId}/wallet`, { 
      wallet_address: walletAddress 
    });

    const userProps: UserProps = {
      id: response.id,
      email: response.email,
      username: response.username,
      role: response.role as 'user' | 'artist' | 'admin',
      walletAddress: walletAddress,
      isVerified: false,
      createdAt: new Date(response.created_at)
    };

    return User.create(userProps);
  }

  async getWalletBalance(walletAddress: string): Promise<number> {
    // Endpoint real: GET /api/v1/wallet/balance/:blockchain/:address
    const response = await this.apiClient.get<{
      blockchain: string;
      address: string;
      balance: number;
      symbol: string;
      timestamp: string;
      status: string;
    }>(`/wallet/balance/ethereum/${walletAddress}`);

    return response.balance;
  }

  async getUserTransactions(userId: string): Promise<any[]> {
    // Endpoint real: GET /api/v1/user/transactions
    return this.apiClient.get<any[]>(`/user/transactions`);
  }
} 