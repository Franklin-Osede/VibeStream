import { User } from '../entities/User';

export interface CreateUserData {
  email: string;
  username: string;
  password: string;
  role?: 'user' | 'artist';
}

export interface LoginCredentials {
  email: string;
  password: string;
}

export interface AuthResult {
  user: User;
  token: string;
}

export interface UserRepository {
  // Authentication
  register(userData: CreateUserData): Promise<AuthResult>;
  login(credentials: LoginCredentials): Promise<AuthResult>;
  logout(): Promise<void>;
  
  // User management
  findById(id: string): Promise<User | null>;
  getCurrentUser(): Promise<User | null>;
  updateProfile(userId: string, updates: Partial<CreateUserData>): Promise<User>;
  
  // Wallet operations
  connectWallet(userId: string, walletAddress: string): Promise<User>;
  getWalletBalance(walletAddress: string): Promise<number>;
  
  // Transactions
  getUserTransactions(userId: string): Promise<any[]>;
} 