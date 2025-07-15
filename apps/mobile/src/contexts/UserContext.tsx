// Context: UserContext
// Maneja el estado del usuario usando la arquitectura DDD

import React, { createContext, useContext, useState, ReactNode, useEffect } from 'react';
import { User, UserContextType } from '../types';
import { MockUserRepository } from '../domain/repositories/UserRepository';

const UserContext = createContext<UserContextType | undefined>(undefined);

interface UserProviderProps {
  children: ReactNode;
}

export function UserProvider({ children }: UserProviderProps) {
  const [user, setUser] = useState<User | null>(null);
  const [token, setToken] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const userRepository = new MockUserRepository();

  const logout = () => {
    setUser(null);
    setToken(null);
    setError(null);
  };

  const isAuthenticated = !!user && !!token;

  const updateUserProfile = async (profile: Partial<User['profile']>) => {
    if (!user) return;

    try {
      setLoading(true);
      setError(null);

      // Update user profile
      const updatedUser = {
        ...user,
        profile: {
          ...user.profile,
          ...profile
        }
      };
      
      // Save to repository
      await userRepository.save(updatedUser);
      
      // Update local state
      setUser(updatedUser);

    } catch (error) {
      setError(error instanceof Error ? error.message : 'Failed to update profile');
      throw error;
    } finally {
      setLoading(false);
    }
  };

  const followUser = async (userId: string) => {
    if (!user) return;

    try {
      setLoading(true);
      setError(null);

      const targetUserId = { value: userId };
      await userRepository.followUser(user.id, targetUserId);

      // Update local user stats
      const updatedUser = {
        ...user,
        stats: {
          ...user.stats,
          following: user.stats.following + 1
        }
      };
      setUser(updatedUser);

    } catch (error) {
      setError(error instanceof Error ? error.message : 'Failed to follow user');
      throw error;
    } finally {
      setLoading(false);
    }
  };

  const unfollowUser = async (userId: string) => {
    if (!user) return;

    try {
      setLoading(true);
      setError(null);

      const targetUserId = { value: userId };
      await userRepository.unfollowUser(user.id, targetUserId);

      // Update local user stats
      const updatedUser = {
        ...user,
        stats: {
          ...user.stats,
          following: Math.max(0, user.stats.following - 1)
        }
      };
      setUser(updatedUser);

    } catch (error) {
      setError(error instanceof Error ? error.message : 'Failed to unfollow user');
      throw error;
    } finally {
      setLoading(false);
    }
  };

  const addVibers = async (amount: number) => {
    if (!user) return;

    try {
      setLoading(true);
      setError(null);

      const updatedUser = {
        ...user,
        wallet: {
          ...user.wallet,
          balance: {
            ...user.wallet.balance,
            vibers: user.wallet.balance.vibers + amount
          }
        }
      };
      
      await userRepository.save(updatedUser);
      setUser(updatedUser);

    } catch (error) {
      setError(error instanceof Error ? error.message : 'Failed to add vibers');
      throw error;
    } finally {
      setLoading(false);
    }
  };

  const spendVibers = async (amount: number): Promise<boolean> => {
    if (!user) return false;

    try {
      setLoading(true);
      setError(null);

      if (user.wallet.balance.vibers >= amount) {
        const updatedUser = {
          ...user,
          wallet: {
            ...user.wallet,
            balance: {
              ...user.wallet.balance,
              vibers: user.wallet.balance.vibers - amount
            }
          }
        };
        
        await userRepository.save(updatedUser);
        setUser(updatedUser);
        return true;
      }
      return false;

    } catch (error) {
      setError(error instanceof Error ? error.message : 'Failed to spend vibers');
      throw error;
    } finally {
      setLoading(false);
    }
  };

  const addEth = async (amount: number) => {
    if (!user) return;

    try {
      setLoading(true);
      setError(null);

      const updatedUser = {
        ...user,
        wallet: {
          ...user.wallet,
          balance: {
            ...user.wallet.balance,
            eth: user.wallet.balance.eth + amount
          }
        }
      };
      
      await userRepository.save(updatedUser);
      setUser(updatedUser);

    } catch (error) {
      setError(error instanceof Error ? error.message : 'Failed to add ETH');
      throw error;
    } finally {
      setLoading(false);
    }
  };

  const spendEth = async (amount: number): Promise<boolean> => {
    if (!user) return false;

    try {
      setLoading(true);
      setError(null);

      if (user.wallet.balance.eth >= amount) {
        const updatedUser = {
          ...user,
          wallet: {
            ...user.wallet,
            balance: {
              ...user.wallet.balance,
              eth: user.wallet.balance.eth - amount
            }
          }
        };
        
        await userRepository.save(updatedUser);
        setUser(updatedUser);
        return true;
      }
      return false;

    } catch (error) {
      setError(error instanceof Error ? error.message : 'Failed to spend ETH');
      throw error;
    } finally {
      setLoading(false);
    }
  };

  const value: UserContextType = {
    user,
    token,
    setUser,
    setToken,
    logout,
    isAuthenticated,
    loading,
    error,
    updateUserProfile,
    followUser,
    unfollowUser,
    addVibers,
    spendVibers,
    addEth,
    spendEth,
  };

  return (
    <UserContext.Provider value={value}>
      {children}
    </UserContext.Provider>
  );
}

export function useUser() {
  const context = useContext(UserContext);
  if (context === undefined) {
    throw new Error('useUser must be used within a UserProvider');
  }
  return context;
} 