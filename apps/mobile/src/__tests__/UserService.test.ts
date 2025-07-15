// Tests for UserService - Following TDD principles
// These tests would run with Jest in a real environment

import { UserService, UserServiceImpl } from '../application/services/UserService';
import { MockUserRepository } from '../domain/repositories/UserRepository';
import { User } from '../domain/entities/User';

describe('UserService', () => {
  let userService: UserService;
  let userRepository: MockUserRepository;

  beforeEach(() => {
    userRepository = new MockUserRepository();
    userService = new UserServiceImpl(userRepository);
  });

  describe('registerUser', () => {
    it('should register a new user successfully', async () => {
      // Arrange
      const username = 'testuser';
      const email = 'test@example.com';
      const role = 'artist' as const;

      // Act
      const result = await userService.registerUser(username, email, role);

      // Assert
      expect(result).toBeDefined();
      expect(result.profile.username).toBe(username);
      expect(result.profile.email).toBe(email);
      expect(result.profile.role).toBe(role);
      expect(result.isValid()).toBe(true);
    });

    it('should throw error for invalid email', async () => {
      // Arrange
      const username = 'testuser';
      const email = 'invalid-email';
      const role = 'artist' as const;

      // Act & Assert
      await expect(userService.registerUser(username, email, role))
        .rejects.toThrow('Invalid email format');
    });

    it('should throw error for missing required fields', async () => {
      // Act & Assert
      await expect(userService.registerUser('', 'test@example.com', 'artist'))
        .rejects.toThrow('Username, email, and role are required');
    });

    it('should throw error for duplicate email', async () => {
      // Arrange
      await userService.registerUser('user1', 'test@example.com', 'artist');

      // Act & Assert
      await expect(userService.registerUser('user2', 'test@example.com', 'fan'))
        .rejects.toThrow('User with this email already exists');
    });
  });

  describe('authenticateUser', () => {
    it('should authenticate existing user', async () => {
      // Arrange
      const email = 'luna@vibestream.com';

      // Act
      const result = await userService.authenticateUser(email, 'password');

      // Assert
      expect(result).toBeDefined();
      expect(result?.profile.email).toBe(email);
    });

    it('should return null for non-existing user', async () => {
      // Arrange
      const email = 'nonexistent@example.com';

      // Act
      const result = await userService.authenticateUser(email, 'password');

      // Assert
      expect(result).toBeNull();
    });
  });

  describe('getUserProfile', () => {
    it('should return user profile for valid ID', async () => {
      // Arrange
      const userId = '1';

      // Act
      const result = await userService.getUserProfile(userId);

      // Assert
      expect(result).toBeDefined();
      expect(result?.id.value).toBe(userId);
    });

    it('should return null for invalid ID', async () => {
      // Arrange
      const userId = 'invalid-id';

      // Act
      const result = await userService.getUserProfile(userId);

      // Assert
      expect(result).toBeNull();
    });
  });

  describe('followUser', () => {
    it('should follow user successfully', async () => {
      // Arrange
      const followerId = '4'; // Music Fan
      const followedId = '1'; // Luna Echo

      // Act
      await userService.followUser(followerId, followedId);

      // Assert
      const follower = await userService.getUserProfile(followerId);
      expect(follower?.profile.following).toBeGreaterThan(0);
    });

    it('should throw error when trying to follow yourself', async () => {
      // Arrange
      const userId = '1';

      // Act & Assert
      await expect(userService.followUser(userId, userId))
        .rejects.toThrow('Cannot follow yourself');
    });

    it('should throw error for non-existing users', async () => {
      // Act & Assert
      await expect(userService.followUser('invalid', 'also-invalid'))
        .rejects.toThrow('User not found');
    });
  });

  describe('searchUsers', () => {
    it('should return users matching search query', async () => {
      // Arrange
      const query = 'Luna';

      // Act
      const result = await userService.searchUsers(query);

      // Assert
      expect(result).toHaveLength(1);
      expect(result[0].profile.username).toContain('Luna');
    });

    it('should return empty array for empty query', async () => {
      // Arrange
      const query = '';

      // Act
      const result = await userService.searchUsers(query);

      // Assert
      expect(result).toHaveLength(0);
    });

    it('should return empty array for no matches', async () => {
      // Arrange
      const query = 'NonexistentUser';

      // Act
      const result = await userService.searchUsers(query);

      // Assert
      expect(result).toHaveLength(0);
    });
  });

  describe('getTrendingArtists', () => {
    it('should return artists sorted by followers', async () => {
      // Act
      const result = await userService.getTrendingArtists();

      // Assert
      expect(result.length).toBeGreaterThan(0);
      expect(result.every(user => user.profile.isArtist())).toBe(true);
      
      // Check if sorted by followers (descending)
      for (let i = 0; i < result.length - 1; i++) {
        expect(result[i].profile.followers).toBeGreaterThanOrEqual(result[i + 1].profile.followers);
      }
    });
  });

  describe('balance management', () => {
    it('should add vibers to user balance', async () => {
      // Arrange
      const userId = '1';
      const initialBalance = await userService.getUserBalance(userId);
      const amount = 1000;

      // Act
      await userService.addVibers(userId, amount);

      // Assert
      const newBalance = await userService.getUserBalance(userId);
      expect(newBalance.vibers).toBe(initialBalance.vibers + amount);
    });

    it('should add eth to user balance', async () => {
      // Arrange
      const userId = '1';
      const initialBalance = await userService.getUserBalance(userId);
      const amount = 0.5;

      // Act
      await userService.addEth(userId, amount);

      // Assert
      const newBalance = await userService.getUserBalance(userId);
      expect(newBalance.eth).toBe(initialBalance.eth + amount);
    });

    it('should purchase with vibers when sufficient balance', async () => {
      // Arrange
      const userId = '1';
      await userService.addVibers(userId, 1000);
      const amount = 500;

      // Act
      const result = await userService.purchaseWithVibers(userId, amount);

      // Assert
      expect(result).toBe(true);
      const balance = await userService.getUserBalance(userId);
      expect(balance.vibers).toBe(500); // 1000 - 500
    });

    it('should fail purchase when insufficient vibers', async () => {
      // Arrange
      const userId = '1';
      const amount = 10000; // More than user has

      // Act
      const result = await userService.purchaseWithVibers(userId, amount);

      // Assert
      expect(result).toBe(false);
    });

    it('should throw error for negative amounts', async () => {
      // Arrange
      const userId = '1';
      const amount = -100;

      // Act & Assert
      await expect(userService.addVibers(userId, amount))
        .rejects.toThrow('Amount must be positive');
    });
  });

  describe('getRecommendedUsers', () => {
    it('should return recommended users excluding current user', async () => {
      // Arrange
      const userId = '1';

      // Act
      const result = await userService.getRecommendedUsers(userId);

      // Assert
      expect(result.length).toBeGreaterThan(0);
      expect(result.every(user => user.id.value !== userId)).toBe(true);
    });

    it('should return users when current user not found', async () => {
      // Arrange
      const userId = 'invalid-id';

      // Act
      const result = await userService.getRecommendedUsers(userId);

      // Assert
      expect(result.length).toBeGreaterThan(0);
    });
  });
}); 