// Application Service: UserService
// Siguiendo TDD y DDD - Application Layer

import { User, UserId } from '../../domain/entities/User';
import { UserRepository } from '../../domain/repositories/UserRepository';

export interface UserService {
  // User Management
  registerUser(username: string, email: string, role: 'artist' | 'fan'): Promise<User>;
  authenticateUser(email: string, password: string): Promise<User | null>;
  getUserProfile(userId: string): Promise<User | null>;
  updateUserProfile(userId: string, updates: Partial<User>): Promise<User>;
  
  // Social Features
  followUser(followerId: string, followedId: string): Promise<void>;
  unfollowUser(followerId: string, followedId: string): Promise<void>;
  getFollowers(userId: string): Promise<User[]>;
  getFollowing(userId: string): Promise<User[]>;
  
  // Search and Discovery
  searchUsers(query: string): Promise<User[]>;
  getTrendingArtists(): Promise<User[]>;
  getRecommendedUsers(userId: string): Promise<User[]>;
  
  // Balance Management
  getUserBalance(userId: string): Promise<{ vibers: number; eth: number }>;
  addVibers(userId: string, amount: number): Promise<void>;
  addEth(userId: string, amount: number): Promise<void>;
  purchaseWithVibers(userId: string, amount: number): Promise<boolean>;
  purchaseWithEth(userId: string, amount: number): Promise<boolean>;
}

export class UserServiceImpl implements UserService {
  constructor(private userRepository: UserRepository) {}

  // User Management
  async registerUser(username: string, email: string, role: 'artist' | 'fan'): Promise<User> {
    // Validate input
    if (!username || !email || !role) {
      throw new Error('Username, email, and role are required');
    }

    if (!email.includes('@')) {
      throw new Error('Invalid email format');
    }

    // Check if user already exists
    const existingUser = await this.userRepository.findByEmail(email);
    if (existingUser) {
      throw new Error('User with this email already exists');
    }

    const existingUsername = await this.userRepository.findByUsername(username);
    if (existingUsername) {
      throw new Error('Username already taken');
    }

    // Create new user
    const userId = Math.random().toString(36).substr(2, 9); // Simple ID generation
    const user = User.create(userId, username, email, role);

    // Save user
    await this.userRepository.save(user);

    return user;
  }

  async authenticateUser(email: string, password: string): Promise<User | null> {
    // In a real app, you would hash the password and compare
    // For now, we'll just check if the user exists
    const user = await this.userRepository.findByEmail(email);
    return user;
  }

  async getUserProfile(userId: string): Promise<User | null> {
    const user = await this.userRepository.findById({ value: userId });
    return user;
  }

  async updateUserProfile(userId: string, updates: Partial<User>): Promise<User> {
    const user = await this.userRepository.findById({ value: userId });
    if (!user) {
      throw new Error('User not found');
    }

    // Apply updates (in a real app, you'd have proper update methods)
    // For now, we'll just save the user as-is
    await this.userRepository.update(user);
    return user;
  }

  // Social Features
  async followUser(followerId: string, followedId: string): Promise<void> {
    const follower = await this.userRepository.findById({ value: followerId });
    const followed = await this.userRepository.findById({ value: followedId });

    if (!follower || !followed) {
      throw new Error('User not found');
    }

    if (followerId === followedId) {
      throw new Error('Cannot follow yourself');
    }

    follower.followUser();
    await this.userRepository.update(follower);
  }

  async unfollowUser(followerId: string, followedId: string): Promise<void> {
    const follower = await this.userRepository.findById({ value: followerId });
    const followed = await this.userRepository.findById({ value: followedId });

    if (!follower || !followed) {
      throw new Error('User not found');
    }

    follower.unfollowUser();
    await this.userRepository.update(follower);
  }

  async getFollowers(userId: string): Promise<User[]> {
    // Mock implementation - in real app this would query relationships
    const allUsers = await this.userRepository.findAll();
    return allUsers.slice(0, 5); // Return first 5 users as mock followers
  }

  async getFollowing(userId: string): Promise<User[]> {
    // Mock implementation - in real app this would query relationships
    const allUsers = await this.userRepository.findAll();
    return allUsers.slice(0, 3); // Return first 3 users as mock following
  }

  // Search and Discovery
  async searchUsers(query: string): Promise<User[]> {
    if (!query || query.trim().length === 0) {
      return [];
    }

    return await this.userRepository.searchUsers(query.trim());
  }

  async getTrendingArtists(): Promise<User[]> {
    const artists = await this.userRepository.findArtists();
    // Sort by followers (mock trending logic)
    return artists.sort((a, b) => b.profile.followers - a.profile.followers);
  }

  async getRecommendedUsers(userId: string): Promise<User[]> {
    // Mock implementation - in real app this would use ML/recommendation logic
    const allUsers = await this.userRepository.findAll();
    const currentUser = await this.userRepository.findById({ value: userId });
    
    if (!currentUser) {
      return allUsers.slice(0, 5);
    }

    // Filter out current user and return others
    return allUsers
      .filter(user => user.id.value !== userId)
      .slice(0, 5);
  }

  // Balance Management
  async getUserBalance(userId: string): Promise<{ vibers: number; eth: number }> {
    const user = await this.userRepository.findById({ value: userId });
    if (!user) {
      throw new Error('User not found');
    }

    return user.balance;
  }

  async addVibers(userId: string, amount: number): Promise<void> {
    const user = await this.userRepository.findById({ value: userId });
    if (!user) {
      throw new Error('User not found');
    }

    if (amount <= 0) {
      throw new Error('Amount must be positive');
    }

    user.earnVibers(amount);
    await this.userRepository.update(user);
  }

  async addEth(userId: string, amount: number): Promise<void> {
    const user = await this.userRepository.findById({ value: userId });
    if (!user) {
      throw new Error('User not found');
    }

    if (amount <= 0) {
      throw new Error('Amount must be positive');
    }

    user.earnEth(amount);
    await this.userRepository.update(user);
  }

  async purchaseWithVibers(userId: string, amount: number): Promise<boolean> {
    const user = await this.userRepository.findById({ value: userId });
    if (!user) {
      throw new Error('User not found');
    }

    if (amount <= 0) {
      throw new Error('Amount must be positive');
    }

    const success = user.purchaseWithVibers(amount);
    if (success) {
      await this.userRepository.update(user);
    }

    return success;
  }

  async purchaseWithEth(userId: string, amount: number): Promise<boolean> {
    const user = await this.userRepository.findById({ value: userId });
    if (!user) {
      throw new Error('User not found');
    }

    if (amount <= 0) {
      throw new Error('Amount must be positive');
    }

    const success = user.purchaseWithEth(amount);
    if (success) {
      await this.userRepository.update(user);
    }

    return success;
  }
} 