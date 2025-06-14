import { SongRepository, PurchaseResult } from '../../domain/repositories/SongRepository';
import { UserRepository } from '../../domain/repositories/UserRepository';
import { Song } from '../../domain/entities/Song';
import { User } from '../../domain/entities/User';

export interface PurchaseSongRequest {
  songId: string;
  userId: string;
  walletAddress: string;
}

export class PurchaseSong {
  constructor(
    private songRepository: SongRepository,
    private userRepository: UserRepository
  ) {}

  async execute(request: PurchaseSongRequest): Promise<PurchaseResult> {
    try {
      // 1. Validate input
      if (!request.songId || !request.userId || !request.walletAddress) {
        throw new Error('Song ID, User ID, and Wallet Address are required');
      }

      // 2. Get and validate user
      const userData = await this.userRepository.findById(request.userId);
      if (!userData) {
        throw new Error('User not found');
      }

      const user = User.create(userData.toJSON());
      if (!user.canPurchaseMusic()) {
        throw new Error('User cannot purchase music');
      }

      if (!user.hasWalletConnected()) {
        throw new Error('User must connect wallet first');
      }

      // 3. Get and validate song
      const songData = await this.songRepository.findById(request.songId);
      if (!songData) {
        throw new Error('Song not found');
      }

      const song = Song.create(songData.toJSON());
      if (!song.isAvailableForPurchase()) {
        throw new Error('Song is not available for purchase');
      }

      // 4. Check wallet balance
      const balance = await this.userRepository.getWalletBalance(request.walletAddress);
      if (balance < song.price) {
        throw new Error('Insufficient wallet balance');
      }

      // 5. Calculate fees (Domain logic)
      const artistRoyalty = song.calculateArtistRoyalty(song.price);
      const platformFee = song.calculatePlatformFee(song.price);

      // 6. Execute purchase through repository
      const purchaseResult = await this.songRepository.purchaseSong(
        request.songId,
        request.userId,
        request.walletAddress
      );

      // 7. Validate purchase result
      if (purchaseResult.status === 'failed') {
        throw new Error('Purchase transaction failed');
      }

      return {
        ...purchaseResult,
        artistRoyalty,
        platformFee,
        totalPaid: song.price
      };

    } catch (error: any) {
      throw new Error(`Purchase failed: ${error.message}`);
    }
  }
}

export class GetUserPurchases {
  constructor(private songRepository: SongRepository) {}

  async execute(userId: string): Promise<Song[]> {
    if (!userId) {
      throw new Error('User ID is required');
    }

    const purchasedSongs = await this.songRepository.getUserPurchasedSongs(userId);
    
    // Validate all songs as domain entities
    return purchasedSongs.map(songData => Song.create(songData.toJSON()));
  }
} 