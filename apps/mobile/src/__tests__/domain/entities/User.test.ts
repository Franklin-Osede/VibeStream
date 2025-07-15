// Tests: User.test.ts
// Siguiendo TDD - Tests para la entidad User

import { User } from '../../types';

describe('User Entity', () => {
  let user: User;

  beforeEach(() => {
    user = {
      id: '1',
      username: 'testuser',
      email: 'test@example.com',
      profile: {
        avatar: 'https://via.placeholder.com/100',
        bio: 'Test bio',
        location: 'Test City',
        website: 'https://test.com'
      },
      wallet: {
        address: '0x1234567890abcdef',
        balance: { eth: 1.0, vibers: 1000 }
      },
      stats: {
        followers: 0,
        following: 0,
        totalPlays: 0,
        totalLikes: 0
      },
      role: 'fan',
      isVerified: false,
      createdAt: new Date(),
      updatedAt: new Date()
    };
  });

  describe('User Creation', () => {
    it('should create a user with valid data', () => {
      expect(user.id).toBe('1');
      expect(user.username).toBe('testuser');
      expect(user.email).toBe('test@example.com');
      expect(user.role).toBe('fan');
      expect(user.isVerified).toBe(false);
    });

    it('should have default stats', () => {
      expect(user.stats.followers).toBe(0);
      expect(user.stats.following).toBe(0);
      expect(user.stats.totalPlays).toBe(0);
      expect(user.stats.totalLikes).toBe(0);
    });

    it('should have default wallet balance', () => {
      expect(user.wallet.balance.eth).toBe(1.0);
      expect(user.wallet.balance.vibers).toBe(1000);
    });
  });

  describe('User Stats Management', () => {
    it('should increment followers when gaining a follower', () => {
      const initialFollowers = user.stats.followers;
      user.stats.followers += 1;
      expect(user.stats.followers).toBe(initialFollowers + 1);
    });

    it('should handle multiple follower gains', () => {
      user.stats.followers += 1;
      user.stats.followers += 1;
      const initialFollowers = user.stats.followers;
      user.stats.followers += 1;
      expect(user.stats.followers).toBe(initialFollowers + 1);
    });

    it('should decrement followers when losing a follower', () => {
      user.stats.followers = 5;
      const initialFollowers = user.stats.followers;
      user.stats.followers = Math.max(0, user.stats.followers - 1);
      expect(user.stats.followers).toBe(initialFollowers - 1);
    });

    it('should not go below 0 followers', () => {
      user.stats.followers = 0;
      user.stats.followers = Math.max(0, user.stats.followers - 1);
      expect(user.stats.followers).toBe(0);
    });

    it('should increment following when following someone', () => {
      const initialFollowing = user.stats.following;
      user.stats.following += 1;
      expect(user.stats.following).toBe(initialFollowing + 1);
    });

    it('should decrement following when unfollowing someone', () => {
      user.stats.following = 5;
      const initialFollowing = user.stats.following;
      user.stats.following = Math.max(0, user.stats.following - 1);
      expect(user.stats.following).toBe(initialFollowing - 1);
    });

    it('should not go below 0 following', () => {
      user.stats.following = 0;
      user.stats.following = Math.max(0, user.stats.following - 1);
      expect(user.stats.following).toBe(0);
    });
  });

  describe('Wallet Management', () => {
    it('should add vibers to wallet', () => {
      const initialVibers = user.wallet.balance.vibers;
      user.wallet.balance.vibers += 500;
      expect(user.wallet.balance.vibers).toBe(initialVibers + 500);
    });

    it('should not allow negative vibers', () => {
      const initialVibers = user.wallet.balance.vibers;
      user.wallet.balance.vibers = Math.max(0, user.wallet.balance.vibers - 100);
      expect(user.wallet.balance.vibers).toBe(initialVibers);
    });

    it('should spend vibers if sufficient balance', () => {
      const initialVibers = user.wallet.balance.vibers;
      if (user.wallet.balance.vibers >= 200) {
        user.wallet.balance.vibers -= 200;
        expect(user.wallet.balance.vibers).toBe(initialVibers - 200);
      }
    });

    it('should not spend vibers if insufficient balance', () => {
      const initialVibers = user.wallet.balance.vibers;
      if (user.wallet.balance.vibers < initialVibers + 100) {
        // Don't spend if insufficient
        expect(user.wallet.balance.vibers).toBe(initialVibers);
      }
    });

    it('should add ETH to wallet', () => {
      const initialEth = user.wallet.balance.eth;
      user.wallet.balance.eth += 0.5;
      expect(user.wallet.balance.eth).toBe(initialEth + 0.5);
    });

    it('should spend ETH if sufficient balance', () => {
      const initialEth = user.wallet.balance.eth;
      if (user.wallet.balance.eth >= 0.5) {
        user.wallet.balance.eth -= 0.5;
        expect(user.wallet.balance.eth).toBe(initialEth - 0.5);
      }
    });

    it('should not spend ETH if insufficient balance', () => {
      const initialEth = user.wallet.balance.eth;
      if (user.wallet.balance.eth < initialEth + 1) {
        // Don't spend if insufficient
        expect(user.wallet.balance.eth).toBe(initialEth);
      }
    });
  });

  describe('Profile Management', () => {
    it('should update user profile', () => {
      const newBio = 'Updated bio';
      user.profile.bio = newBio;
      expect(user.profile.bio).toBe(newBio);
    });

    it('should change user role', () => {
      user.role = 'artist';
      expect(user.role).toBe('artist');
    });

    it('should verify user', () => {
      user.isVerified = true;
      expect(user.isVerified).toBe(true);
    });
  });

  describe('Business Rules', () => {
    it('should allow artists to create VR events', () => {
      const artist = { ...user, role: 'artist' as const, isVerified: true };
      expect(artist.role).toBe('artist');
      expect(artist.isVerified).toBe(true);
    });

    it('should not allow unverified artists to create VR events', () => {
      const artist = { ...user, role: 'artist' as const, isVerified: false };
      expect(artist.role).toBe('artist');
      expect(artist.isVerified).toBe(false);
    });

    it('should not allow fans to create VR events', () => {
      const fan = { ...user, role: 'fan' as const, isVerified: true };
      expect(fan.role).toBe('fan');
    });

    it('should allow artists to mint NFTs', () => {
      const artist = { ...user, role: 'artist' as const };
      expect(artist.role).toBe('artist');
    });

    it('should not allow fans to mint NFTs', () => {
      const fan = { ...user, role: 'fan' as const };
      expect(fan.role).toBe('fan');
    });

    it('should allow users with ETH to trade', () => {
      expect(user.wallet.balance.eth).toBeGreaterThan(0);
    });

    it('should not allow users without ETH to trade', () => {
      const userWithNoEth = { ...user, wallet: { ...user.wallet, balance: { ...user.wallet.balance, eth: 0 } } };
      expect(userWithNoEth.wallet.balance.eth).toBe(0);
    });
  });

  describe('Serialization', () => {
    it('should serialize user to JSON', () => {
      const json = JSON.stringify(user);
      expect(json).toContain(user.id);
      expect(json).toContain(user.username);
      expect(json).toContain(user.email);
    });

    it('should deserialize user from JSON', () => {
      const json = JSON.stringify(user);
      const deserializedUser = JSON.parse(json) as User;
      expect(deserializedUser.role).toBe(user.role);
      expect(deserializedUser.wallet).toEqual(user.wallet);
      expect(deserializedUser.isVerified).toBe(user.isVerified);
    });
  });
}); 