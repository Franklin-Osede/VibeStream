// Domain Repository: UserRepository
// Siguiendo principios DDD - Repository Pattern

import { User, UserId, LoginCredentials, AuthResult, CreateUserData } from '../../types';

export interface UserRepository {
  findById(id: UserId): Promise<User | null>;
  findByEmail(email: string): Promise<User | null>;
  findByUsername(username: string): Promise<User | null>;
  save(user: User): Promise<void>;
  update(user: User): Promise<void>;
  delete(id: UserId): Promise<void>;
  findAll(): Promise<User[]>;
  findArtists(): Promise<User[]>;
  findFans(): Promise<User[]>;
  searchUsers(query: string): Promise<User[]>;
  followUser(userId: string, targetUserId: UserId): Promise<void>;
  unfollowUser(userId: string, targetUserId: UserId): Promise<void>;
  findFollowing(userId: string): Promise<User[]>;
  login(credentials: LoginCredentials): Promise<AuthResult>;
  register(userData: CreateUserData): Promise<AuthResult>;
  getWalletBalance(walletAddress: string): Promise<number>;
}

// Repository Implementation for Mock Data
export class MockUserRepository implements UserRepository {
  private users: Map<string, User> = new Map();

  constructor() {
    // Initialize with mock data
    this.initializeMockData();
  }

  private initializeMockData(): void {
    const mockUsers: User[] = [
      {
        id: '1',
        username: 'Luna Echo',
        email: 'luna@vibestream.com',
        profile: {
          avatar: 'https://via.placeholder.com/100',
          bio: 'Electronic music producer',
          location: 'Los Angeles, CA',
          website: 'https://lunaecho.com'
        },
        wallet: {
          address: '0x1234567890abcdef',
          balance: { eth: 2.5, vibers: 50000 }
        },
        stats: {
          followers: 12500,
          following: 89,
          totalPlays: 2500000,
          totalLikes: 8900
        },
        role: 'artist',
        isVerified: true,
        createdAt: new Date('2023-01-01'),
        updatedAt: new Date()
      },
      {
        id: '2',
        username: 'Cyber Collective',
        email: 'cyber@vibestream.com',
        profile: {
          avatar: 'https://via.placeholder.com/100',
          bio: 'Synthwave collective',
          location: 'Berlin, Germany',
          website: 'https://cybercollective.com'
        },
        wallet: {
          address: '0xabcdef1234567890',
          balance: { eth: 1.8, vibers: 32000 }
        },
        stats: {
          followers: 8900,
          following: 156,
          totalPlays: 1800000,
          totalLikes: 5600
        },
        role: 'artist',
        isVerified: true,
        createdAt: new Date('2023-02-01'),
        updatedAt: new Date()
      },
      {
        id: '3',
        username: 'Music Fan',
        email: 'fan@vibestream.com',
        profile: {
          avatar: 'https://via.placeholder.com/100',
          bio: 'Music enthusiast',
          location: 'New York, NY'
        },
        wallet: {
          address: '0x9876543210fedcba',
          balance: { eth: 0.5, vibers: 5000 }
        },
        stats: {
          followers: 0,
          following: 25,
          totalPlays: 50000,
          totalLikes: 1200
        },
        role: 'fan',
        isVerified: false,
        createdAt: new Date('2023-03-01'),
        updatedAt: new Date()
      }
    ];

    mockUsers.forEach(user => {
      this.users.set(user.id, user);
    });
  }

  async findById(id: UserId): Promise<User | null> {
    return this.users.get(id.value) || null;
  }

  async findByEmail(email: string): Promise<User | null> {
    for (const user of this.users.values()) {
      if (user.email === email) {
        return user;
      }
    }
    return null;
  }

  async findByUsername(username: string): Promise<User | null> {
    for (const user of this.users.values()) {
      if (user.username === username) {
        return user;
      }
    }
    return null;
  }

  async save(user: User): Promise<void> {
    this.users.set(user.id, user);
  }

  async update(user: User): Promise<void> {
    if (this.users.has(user.id)) {
      this.users.set(user.id, user);
    }
  }

  async delete(id: UserId): Promise<void> {
    this.users.delete(id.value);
  }

  async findAll(): Promise<User[]> {
    return Array.from(this.users.values());
  }

  async findArtists(): Promise<User[]> {
    return Array.from(this.users.values()).filter(user => user.role === 'artist');
  }

  async findFans(): Promise<User[]> {
    return Array.from(this.users.values()).filter(user => user.role === 'fan');
  }

  async searchUsers(query: string): Promise<User[]> {
    const lowercaseQuery = query.toLowerCase();
    return Array.from(this.users.values()).filter(user => 
      user.username.toLowerCase().includes(lowercaseQuery) ||
      user.email.toLowerCase().includes(lowercaseQuery)
    );
  }

  async followUser(userId: string, targetUserId: UserId): Promise<void> {
    const user = this.users.get(userId);
    const targetUser = this.users.get(targetUserId.value);
    
    if (user && targetUser) {
      // Update user's following count
      user.stats.following += 1;
      // Update target user's followers count
      targetUser.stats.followers += 1;
      
      this.users.set(userId, user);
      this.users.set(targetUserId.value, targetUser);
    }
  }

  async unfollowUser(userId: string, targetUserId: UserId): Promise<void> {
    const user = this.users.get(userId);
    const targetUser = this.users.get(targetUserId.value);
    
    if (user && targetUser) {
      // Update user's following count
      user.stats.following = Math.max(0, user.stats.following - 1);
      // Update target user's followers count
      targetUser.stats.followers = Math.max(0, targetUser.stats.followers - 1);
      
      this.users.set(userId, user);
      this.users.set(targetUserId.value, targetUser);
    }
  }

  async findFollowing(userId: string): Promise<User[]> {
    // Mock implementation - in real app this would query relationships
    return Array.from(this.users.values()).filter(user => user.role === 'artist');
  }

  async login(credentials: LoginCredentials): Promise<AuthResult> {
    const user = await this.findByEmail(credentials.email);
    if (!user) {
      throw new Error('User not found');
    }
    
    // Mock authentication - in real app this would verify password
    return {
      user,
      token: 'mock-jwt-token-' + user.id
    };
  }

  async register(userData: CreateUserData): Promise<AuthResult> {
    const existingUser = await this.findByEmail(userData.email);
    if (existingUser) {
      throw new Error('User already exists');
    }

    const newUser: User = {
      id: Date.now().toString(),
      username: userData.username,
      email: userData.email,
      profile: {
        avatar: 'https://via.placeholder.com/100'
      },
      wallet: {
        address: '0x' + Math.random().toString(16).substr(2, 40),
        balance: { eth: 0, vibers: 1000 }
      },
      stats: {
        followers: 0,
        following: 0,
        totalPlays: 0,
        totalLikes: 0
      },
      role: userData.role,
      isVerified: false,
      createdAt: new Date(),
      updatedAt: new Date()
    };

    await this.save(newUser);
    
    return {
      user: newUser,
      token: 'mock-jwt-token-' + newUser.id
    };
  }

  async getWalletBalance(walletAddress: string): Promise<number> {
    for (const user of this.users.values()) {
      if (user.wallet.address === walletAddress) {
        return user.wallet.balance.eth;
      }
    }
    return 0;
  }
} 