import { UserRepository, LoginCredentials, AuthResult } from '../../domain/repositories/UserRepository';
import { User } from '../../domain/entities/User';

export class AuthenticateUser {
  constructor(private userRepository: UserRepository) {}

  async execute(credentials: LoginCredentials): Promise<AuthResult> {
    try {
      // Validate input
      if (!credentials.email || !credentials.password) {
        throw new Error('Email and password are required');
      }

      // Authenticate through repository
      const authResult = await this.userRepository.login(credentials);
      
      // Validate domain entity
      const user = User.create(authResult.user.toJSON());
      
      if (!user) {
        throw new Error('Authentication failed');
      }

      return {
        user,
        token: authResult.token
      };

    } catch (error: any) {
      throw new Error(`Authentication failed: ${error.message}`);
    }
  }
}

export class RegisterUser {
  constructor(private userRepository: UserRepository) {}

  async execute(userData: {
    email: string;
    username: string;
    password: string;
    role?: 'user' | 'artist';
  }): Promise<AuthResult> {
    try {
      // Validate input
      if (!userData.email || !userData.username || !userData.password) {
        throw new Error('Email, username, and password are required');
      }

      if (userData.password.length < 6) {
        throw new Error('Password must be at least 6 characters');
      }

      // Create user through repository
      const authResult = await this.userRepository.register(userData);
      
      // Validate domain entity
      const user = User.create(authResult.user.toJSON());

      return {
        user,
        token: authResult.token
      };

    } catch (error: any) {
      throw new Error(`Registration failed: ${error.message}`);
    }
  }
} 