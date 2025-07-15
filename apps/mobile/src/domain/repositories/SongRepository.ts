// Domain Repository: SongRepository
// Siguiendo DDD - Define la interfaz para el acceso a datos de canciones

import { Song, SongId, PurchaseResult } from '../../types';

export interface SongRepository {
  // Basic CRUD operations
  findById(id: SongId): Promise<Song | null>;
  save(song: Song): Promise<void>;
  delete(id: SongId): Promise<void>;

  // Query operations
  findAll(): Promise<Song[]>;
  findByArtist(artistId: string): Promise<Song[]>;
  findByGenre(genre: string): Promise<Song[]>;
  findByMood(mood: string): Promise<Song[]>;
  findTrending(limit: number): Promise<Song[]>;
  findPopular(limit: number): Promise<Song[]>;
  findRecentlyAdded(limit: number): Promise<Song[]>;

  // User-specific operations
  findLikedByUser(userId: string): Promise<Song[]>;
  findInUserLibrary(userId: string): Promise<Song[]>;
  findUserHistory(userId: string, limit: number): Promise<Song[]>;

  // Business operations
  likeSong(songId: SongId, userId: string): Promise<void>;
  unlikeSong(songId: SongId, userId: string): Promise<void>;
  repostSong(songId: SongId, userId: string): Promise<void>;
  unRepostSong(songId: SongId, userId: string): Promise<void>;
  addToLibrary(songId: SongId, userId: string): Promise<void>;
  removeFromLibrary(songId: SongId, userId: string): Promise<void>;
  playSong(songId: SongId): Promise<void>;

  // Search operations
  searchSongs(query: string, limit: number): Promise<Song[]>;
  searchByArtist(artistName: string, limit: number): Promise<Song[]>;

  // Trading operations
  findTradeableSongs(): Promise<Song[]>;
  updateSharePrice(songId: SongId, newPrice: number): Promise<void>;
  purchaseSong(songId: SongId, userId: string, amount: number): Promise<PurchaseResult>;
  getUserPurchasedSongs(userId: string): Promise<Song[]>;
}

// Mock implementation for testing
export class MockSongRepository implements SongRepository {
  private songs: Map<string, Song> = new Map();

  constructor() {
    this.initializeMockData();
  }

  private initializeMockData() {
    const mockSongs: Song[] = [
      {
        id: '1',
        title: 'Midnight Vibes',
        artist: 'Luna Echo',
        artistId: '1',
        duration: 180,
        genre: 'Electronic',
        mood: 'Chill',
        imageUrl: 'https://via.placeholder.com/300',
        audioUrl: 'https://example.com/midnight-vibes.mp3',
        stats: {
          plays: 1250000,
          likes: 8900,
          reposts: 1200
        },
        interactions: {
          isLiked: false,
          isReposted: false,
          isInLibrary: false
        },
        blockchain: {
          tokenId: '1',
          contractAddress: '0x1234567890abcdef',
          royalties: 10,
          fractionalOwnership: true,
          totalShares: 1000,
          availableShares: 500,
          sharePrice: 0.05
        },
        createdAt: new Date('2024-01-15'),
        updatedAt: new Date()
      },
      {
        id: '2',
        title: 'Neon Dreams',
        artist: 'Cyber Collective',
        artistId: '2',
        duration: 210,
        genre: 'Synthwave',
        mood: 'Energetic',
        imageUrl: 'https://via.placeholder.com/300',
        audioUrl: 'https://example.com/neon-dreams.mp3',
        stats: {
          plays: 890000,
          likes: 5600,
          reposts: 890
        },
        interactions: {
          isLiked: true,
          isReposted: false,
          isInLibrary: true
        },
        blockchain: {
          tokenId: '2',
          contractAddress: '0xabcdef1234567890',
          royalties: 15,
          fractionalOwnership: true,
          totalShares: 800,
          availableShares: 300,
          sharePrice: 0.08
        },
        createdAt: new Date('2024-02-01'),
        updatedAt: new Date()
      },
      {
        id: '3',
        title: 'Cosmic Journey',
        artist: 'Stellar Sound',
        artistId: '3',
        duration: 240,
        genre: 'Ambient',
        mood: 'Relaxing',
        imageUrl: 'https://via.placeholder.com/300',
        audioUrl: 'https://example.com/cosmic-journey.mp3',
        stats: {
          plays: 450000,
          likes: 3200,
          reposts: 450
        },
        interactions: {
          isLiked: false,
          isReposted: false,
          isInLibrary: false
        },
        blockchain: {
          tokenId: '3',
          contractAddress: '0x9876543210fedcba',
          royalties: 12,
          fractionalOwnership: false,
          totalShares: 0,
          availableShares: 0,
          sharePrice: 0
        },
        createdAt: new Date('2024-01-20'),
        updatedAt: new Date()
      }
    ];

    mockSongs.forEach(song => {
      this.songs.set(song.id, song);
    });
  }

  async findById(id: SongId): Promise<Song | null> {
    return this.songs.get(id.value) || null;
  }

  async save(song: Song): Promise<void> {
    this.songs.set(song.id, song);
  }

  async delete(id: SongId): Promise<void> {
    this.songs.delete(id.value);
  }

  async findAll(): Promise<Song[]> {
    return Array.from(this.songs.values());
  }

  async findByArtist(artistId: string): Promise<Song[]> {
    return Array.from(this.songs.values())
      .filter(song => song.artistId === artistId);
  }

  async findByGenre(genre: string): Promise<Song[]> {
    return Array.from(this.songs.values())
      .filter(song => song.genre.toLowerCase() === genre.toLowerCase());
  }

  async findByMood(mood: string): Promise<Song[]> {
    return Array.from(this.songs.values())
      .filter(song => song.mood.toLowerCase() === mood.toLowerCase());
  }

  async findTrending(limit: number): Promise<Song[]> {
    return Array.from(this.songs.values())
      .filter(song => song.stats.plays > 100000) // Mock trending condition
      .sort((a, b) => b.stats.plays - a.stats.plays)
      .slice(0, limit);
  }

  async findPopular(limit: number): Promise<Song[]> {
    return Array.from(this.songs.values())
      .filter(song => song.stats.likes > 1000) // Mock popular condition
      .sort((a, b) => b.stats.likes - a.stats.likes)
      .slice(0, limit);
  }

  async findRecentlyAdded(limit: number): Promise<Song[]> {
    return Array.from(this.songs.values())
      .sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime())
      .slice(0, limit);
  }

  async findLikedByUser(userId: string): Promise<Song[]> {
    // Mock implementation - in real app this would query user-song relationships
    return Array.from(this.songs.values())
      .filter(song => song.interactions.isLiked);
  }

  async findInUserLibrary(userId: string): Promise<Song[]> {
    // Mock implementation - in real app this would query user-library relationships
    return Array.from(this.songs.values())
      .filter(song => song.interactions.isInLibrary);
  }

  async findUserHistory(userId: string, limit: number): Promise<Song[]> {
    // Mock implementation - in real app this would query user history
    return Array.from(this.songs.values()).slice(0, limit);
  }

  async likeSong(songId: SongId, userId: string): Promise<void> {
    const song = await this.findById(songId);
    if (song) {
      song.interactions.isLiked = true;
      song.stats.likes += 1;
      await this.save(song);
    }
  }

  async unlikeSong(songId: SongId, userId: string): Promise<void> {
    const song = await this.findById(songId);
    if (song) {
      song.interactions.isLiked = false;
      song.stats.likes = Math.max(0, song.stats.likes - 1);
      await this.save(song);
    }
  }

  async repostSong(songId: SongId, userId: string): Promise<void> {
    const song = await this.findById(songId);
    if (song) {
      song.interactions.isReposted = true;
      song.stats.reposts += 1;
      await this.save(song);
    }
  }

  async unRepostSong(songId: SongId, userId: string): Promise<void> {
    const song = await this.findById(songId);
    if (song) {
      song.interactions.isReposted = false;
      song.stats.reposts = Math.max(0, song.stats.reposts - 1);
      await this.save(song);
    }
  }

  async addToLibrary(songId: SongId, userId: string): Promise<void> {
    const song = await this.findById(songId);
    if (song) {
      song.interactions.isInLibrary = true;
      await this.save(song);
    }
  }

  async removeFromLibrary(songId: SongId, userId: string): Promise<void> {
    const song = await this.findById(songId);
    if (song) {
      song.interactions.isInLibrary = false;
      await this.save(song);
    }
  }

  async playSong(songId: SongId): Promise<void> {
    const song = await this.findById(songId);
    if (song) {
      song.stats.plays += 1;
      await this.save(song);
    }
  }

  async searchSongs(query: string, limit: number): Promise<Song[]> {
    const lowercaseQuery = query.toLowerCase();
    return Array.from(this.songs.values())
      .filter(song => 
        song.title.toLowerCase().includes(lowercaseQuery) ||
        song.artist.toLowerCase().includes(lowercaseQuery) ||
        song.genre.toLowerCase().includes(lowercaseQuery)
      )
      .slice(0, limit);
  }

  async searchByArtist(artistName: string, limit: number): Promise<Song[]> {
    const lowercaseArtistName = artistName.toLowerCase();
    return Array.from(this.songs.values())
      .filter(song => song.artist.toLowerCase().includes(lowercaseArtistName))
      .slice(0, limit);
  }

  async findTradeableSongs(): Promise<Song[]> {
    return Array.from(this.songs.values())
      .filter(song => song.blockchain?.fractionalOwnership === true);
  }

  async updateSharePrice(songId: SongId, newPrice: number): Promise<void> {
    const song = await this.findById(songId);
    if (song && song.blockchain) {
      song.blockchain.sharePrice = newPrice;
      await this.save(song);
    }
  }

  async purchaseSong(songId: SongId, userId: string, amount: number): Promise<PurchaseResult> {
    const song = await this.findById(songId);
    if (!song) {
      throw new Error('Song not found');
    }

    // Mock purchase logic
    const totalPaid = amount * (song.blockchain?.sharePrice || 0);
    const royalties = totalPaid * ((song.blockchain?.royalties || 0) / 100);
    const platformFee = totalPaid * 0.05; // 5% platform fee

    return {
      success: true,
      transactionId: 'tx-' + Date.now(),
      totalPaid,
      royalties,
      platformFee
    };
  }

  async getUserPurchasedSongs(userId: string): Promise<Song[]> {
    // Mock implementation - in real app this would query user purchases
    return Array.from(this.songs.values())
      .filter(song => song.interactions.isInLibrary);
  }
} 