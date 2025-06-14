import { Song } from '../entities/Song';

export interface SongFilters {
  genre?: string;
  artistId?: string;
  priceRange?: { min: number; max: number };
  isMinted?: boolean;
}

export interface PurchaseResult {
  transactionHash: string;
  artistRoyalty: number;
  platformFee: number;
  totalPaid: number;
  status: 'pending' | 'completed' | 'failed';
}

export interface SongRepository {
  // Song discovery
  findAll(filters?: SongFilters): Promise<Song[]>;
  findById(id: string): Promise<Song | null>;
  findByArtist(artistId: string): Promise<Song[]>;
  searchByTitle(query: string): Promise<Song[]>;
  
  // Featured content
  getFeaturedSongs(): Promise<Song[]>;
  getTrendingSongs(): Promise<Song[]>;
  getRecommendedSongs(userId: string): Promise<Song[]>;
  
  // Purchase operations
  purchaseSong(songId: string, userId: string, walletAddress: string): Promise<PurchaseResult>;
  getUserPurchasedSongs(userId: string): Promise<Song[]>;
  
  // Artist operations (if user is artist)
  createSong(songData: Omit<Song, 'id' | 'createdAt'>): Promise<Song>;
  updateSong(songId: string, updates: Partial<Song>): Promise<Song>;
  mintSongAsNFT(songId: string): Promise<Song>;
} 