// Hook: useHomeScreen
// Integra la arquitectura DDD con sincronización perfecta del backend

import { useState, useEffect, useCallback } from 'react';
import { User, HomeFeedData, HomeScreenState } from '../types';
import { MockUserRepository } from '../domain/repositories/UserRepository';
import { MockSongRepository } from '../domain/repositories/SongRepository';
import { GetHomeFeedUseCase } from '../application/use-cases/GetHomeFeedUseCase';
import { BackendSyncService } from '../infrastructure/services/BackendSyncService';
import { useUser } from '../contexts/UserContext';

export const useHomeScreen = () => {
  const { user, token } = useUser();
  const [state, setState] = useState<HomeScreenState>({
    loading: true,
    error: null,
    data: null,
    refreshing: false,
    syncStatus: {
      isConnected: false,
      lastSync: '',
      pendingEvents: 0
    }
  });

  // Initialize repositories and use cases
  const userRepository = new MockUserRepository();
  const songRepository = new MockSongRepository();
  const getHomeFeedUseCase = new GetHomeFeedUseCase(userRepository, songRepository);
  const backendSyncService = new BackendSyncService();

  // Load initial data
  const loadData = useCallback(async () => {
    if (!user) return;

    try {
      setState(prev => ({ ...prev, loading: true, error: null }));

      const userId = { value: user.id };
      const homeData = await getHomeFeedUseCase.execute(userId);

      setState(prev => ({
        ...prev,
        data: homeData,
        loading: false
      }));
    } catch (error) {
      console.error('Error loading home data:', error);
      setState(prev => ({
        ...prev,
        error: error instanceof Error ? error.message : 'Unknown error',
        loading: false
      }));
    }
  }, [user, getHomeFeedUseCase]);

  // Refresh data
  const refresh = useCallback(async () => {
    if (!user) return;

    try {
      setState(prev => ({ ...prev, refreshing: true }));
      await loadData();
    } finally {
      setState(prev => ({ ...prev, refreshing: false }));
    }
  }, [user, loadData]);

  // Initialize backend sync
  useEffect(() => {
    if (!user || !token) return;

    const initializeSync = async () => {
      try {
        // Connect to backend WebSocket
        await backendSyncService.connect(token);

        // Subscribe to relevant events
        backendSyncService.subscribe('SongPlayed', (event: any) => {
          console.log('Song played event received:', event);
          // Update local state if needed
        });

        backendSyncService.subscribe('SongLiked', (event: any) => {
          console.log('Song liked event received:', event);
          // Update local state if needed
        });

        backendSyncService.subscribe('UserFollowed', (event: any) => {
          console.log('User followed event received:', event);
          // Update local state if needed
        });

        // Update sync status
        setState(prev => ({
          ...prev,
          syncStatus: {
            isConnected: backendSyncService.isConnected(),
            lastSync: new Date().toISOString(),
            pendingEvents: backendSyncService.getSyncState().pendingEvents.length
          }
        }));

      } catch (error) {
        console.error('Failed to initialize backend sync:', error);
      }
    };

    initializeSync();

    // Cleanup on unmount
    return () => {
      backendSyncService.disconnect();
    };
  }, [user, token, backendSyncService]);

  // Load data on mount and user change
  useEffect(() => {
    loadData();
  }, [loadData]);

  // Song interaction handlers
  const handlePlaySong = useCallback(async (songId: string) => {
    if (!user) return;

    try {
      const songIdObj = { value: songId };
      
      // Update local state immediately for better UX
      setState(prev => {
        if (!prev.data) return prev;

        const updatedSongs = prev.data.recommendedSongs.map(song => {
          if (song.id === songId) {
            // Update song stats
            return {
              ...song,
              stats: {
                ...song.stats,
                plays: song.stats.plays + 1
              }
            };
          }
          return song;
        });

        return {
          ...prev,
          data: {
            ...prev.data,
            recommendedSongs: updatedSongs
          }
        };
      });

      // Publish event to backend
      await backendSyncService.publishEvent('SongPlayed', {
        songId,
        userId: user.id,
        timestamp: new Date().toISOString()
      });

      // Update repository
      await songRepository.playSong(songIdObj);

    } catch (error) {
      console.error('Error playing song:', error);
    }
  }, [user, songRepository, backendSyncService]);

  const handleLikeSong = useCallback(async (songId: string) => {
    if (!user) return;

    try {
      const songIdObj = { value: songId };
      
      // Update local state immediately
      setState(prev => {
        if (!prev.data) return prev;

        const updatedSongs = prev.data.recommendedSongs.map(song => {
          if (song.id === songId) {
            const isLiked = song.interactions.isLiked;
            return {
              ...song,
              interactions: {
                ...song.interactions,
                isLiked: !isLiked
              },
              stats: {
                ...song.stats,
                likes: isLiked ? song.stats.likes - 1 : song.stats.likes + 1
              }
            };
          }
          return song;
        });

        return {
          ...prev,
          data: {
            ...prev.data,
            recommendedSongs: updatedSongs
          }
        };
      });

      // Publish event to backend
      await backendSyncService.publishEvent('SongLiked', {
        songId,
        userId: user.id,
        timestamp: new Date().toISOString()
      });

      // Update repository
      const song = await songRepository.findById(songIdObj);
      if (song) {
        if (song.interactions.isLiked) {
          await songRepository.unlikeSong(songIdObj, user.id);
        } else {
          await songRepository.likeSong(songIdObj, user.id);
        }
      }

    } catch (error) {
      console.error('Error liking song:', error);
    }
  }, [user, songRepository, backendSyncService]);

  const handleRepostSong = useCallback(async (songId: string) => {
    if (!user) return;

    try {
      const songIdObj = { value: songId };
      
      // Update local state immediately
      setState(prev => {
        if (!prev.data) return prev;

        const updatedSongs = prev.data.recommendedSongs.map(song => {
          if (song.id === songId) {
            const isReposted = song.interactions.isReposted;
            return {
              ...song,
              interactions: {
                ...song.interactions,
                isReposted: !isReposted
              },
              stats: {
                ...song.stats,
                reposts: isReposted ? song.stats.reposts - 1 : song.stats.reposts + 1
              }
            };
          }
          return song;
        });

        return {
          ...prev,
          data: {
            ...prev.data,
            recommendedSongs: updatedSongs
          }
        };
      });

      // Publish event to backend
      await backendSyncService.publishEvent('SongReposted', {
        songId,
        userId: user.id,
        timestamp: new Date().toISOString()
      });

      // Update repository
      const song = await songRepository.findById(songIdObj);
      if (song) {
        if (song.interactions.isReposted) {
          await songRepository.unRepostSong(songIdObj, user.id);
        } else {
          await songRepository.repostSong(songIdObj, user.id);
        }
      }

    } catch (error) {
      console.error('Error reposting song:', error);
    }
  }, [user, songRepository, backendSyncService]);

  const handleShareSong = useCallback(async (songId: string) => {
    if (!user) return;

    try {
      // Publish event to backend
      await backendSyncService.publishEvent('SongShared', {
        songId,
        userId: user.id,
        timestamp: new Date().toISOString()
      });

      // Update local state
      setState(prev => {
        if (!prev.data) return prev;

        const updatedSongs = prev.data.recommendedSongs.map(song => {
          if (song.id === songId) {
            return {
              ...song,
              stats: {
                ...song.stats,
                plays: song.stats.plays + 1
              }
            };
          }
          return song;
        });

        return {
          ...prev,
          data: {
            ...prev.data,
            recommendedSongs: updatedSongs
          }
        };
      });

    } catch (error) {
      console.error('Error sharing song:', error);
    }
  }, [user, backendSyncService]);

  // VR Event handlers
  const handleJoinVREvent = useCallback(async (eventId: string) => {
    if (!user) return;

    try {
      await backendSyncService.publishEvent('VREventJoined', {
        eventId,
        userId: user.id,
        timestamp: new Date().toISOString()
      });

      console.log('Joined VR event:', eventId);
    } catch (error) {
      console.error('Error joining VR event:', error);
    }
  }, [user, backendSyncService]);

  // NFT handlers
  const handleBuyNFT = useCallback(async (nftId: string) => {
    if (!user) return;

    try {
      await backendSyncService.publishEvent('NFTPurchased', {
        nftId,
        userId: user.id,
        timestamp: new Date().toISOString()
      });

      console.log('Purchased NFT:', nftId);
    } catch (error) {
      console.error('Error purchasing NFT:', error);
    }
  }, [user, backendSyncService]);

  // Trading handlers
  const handleTrade = useCallback(async (songId: string, action: 'buy' | 'sell', amount: number) => {
    if (!user) return;

    try {
      await backendSyncService.publishEvent('TradeExecuted', {
        songId,
        userId: user.id,
        action,
        amount,
        timestamp: new Date().toISOString()
      });

      console.log('Trade executed:', { songId, action, amount });
    } catch (error) {
      console.error('Error executing trade:', error);
    }
  }, [user, backendSyncService]);

  return {
    // State
    ...state,
    
    // Actions
    refresh,
    handlePlaySong,
    handleLikeSong,
    handleRepostSong,
    handleShareSong,
    handleJoinVREvent,
    handleBuyNFT,
    handleTrade,
    
    // Utilities
    isConnected: backendSyncService.isConnected(),
    syncState: backendSyncService.getSyncState()
  };
}; 