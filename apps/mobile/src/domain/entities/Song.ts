// Domain Entity: Song
// Siguiendo principios DDD - Rich Domain Model

export interface SongId {
  value: string;
}

export interface ArtistId {
  value: string;
}

export interface BlockchainData {
  tokenId?: string;
  contractAddress?: string;
  royalties: number;
  fractionalOwnership: boolean;
  
  isValid(): boolean;
  canBeTraded(): boolean;
  getRoyaltyAmount(price: number): number;
}

export interface SongMetadata {
  title: string;
  artist: string;
  artistId: ArtistId;
  duration: number;
  genre: string;
  mood: string;
  releaseDate: string;
  
  isValid(): boolean;
  getDurationInMinutes(): number;
  getFormattedDuration(): string;
}

export interface SongStats {
  plays: number;
  likes: number;
  reposts: number;
  
  incrementPlays(): void;
  incrementLikes(): void;
  decrementLikes(): void;
  incrementReposts(): void;
  decrementReposts(): void;
  getEngagementRate(): number;
}

export interface SongState {
  isLiked: boolean;
  isReposted: boolean;
  isDownloaded: boolean;
  
  toggleLike(): void;
  toggleRepost(): void;
  toggleDownload(): void;
}

export class Song {
  private readonly _id: SongId;
  private _metadata: SongMetadata;
  private _stats: SongStats;
  private _state: SongState;
  private _blockchainData?: BlockchainData;
  private _imageUrl: string;
  private _audioUrl: string;

  constructor(
    id: SongId,
    metadata: SongMetadata,
    stats: SongStats,
    state: SongState,
    imageUrl: string,
    audioUrl: string,
    blockchainData?: BlockchainData
  ) {
    this._id = id;
    this._metadata = metadata;
    this._stats = stats;
    this._state = state;
    this._imageUrl = imageUrl;
    this._audioUrl = audioUrl;
    this._blockchainData = blockchainData;
  }

  // Identity
  get id(): SongId {
    return this._id;
  }

  // Metadata
  get metadata(): SongMetadata {
    return this._metadata;
  }

  // Stats
  get stats(): SongStats {
    return this._stats;
  }

  // State
  get state(): SongState {
    return this._state;
  }

  // URLs
  get imageUrl(): string {
    return this._imageUrl;
  }

  get audioUrl(): string {
    return this._audioUrl;
  }

  // Blockchain
  get blockchainData(): BlockchainData | undefined {
    return this._blockchainData;
  }

  // Business Logic
  play(): void {
    this._stats.incrementPlays();
  }

  like(): void {
    if (!this._state.isLiked) {
      this._stats.incrementLikes();
      this._state.toggleLike();
    }
  }

  unlike(): void {
    if (this._state.isLiked) {
      this._stats.decrementLikes();
      this._state.toggleLike();
    }
  }

  repost(): void {
    if (!this._state.isReposted) {
      this._stats.incrementReposts();
      this._state.toggleRepost();
    }
  }

  unRepost(): void {
    if (this._state.isReposted) {
      this._stats.decrementReposts();
      this._state.toggleRepost();
    }
  }

  download(): void {
    this._state.toggleDownload();
  }

  // Blockchain Logic
  canBeTraded(): boolean {
    return this._blockchainData?.canBeTraded() ?? false;
  }

  hasFractionalOwnership(): boolean {
    return this._blockchainData?.fractionalOwnership ?? false;
  }

  getRoyaltyAmount(price: number): number {
    return this._blockchainData?.getRoyaltyAmount(price) ?? 0;
  }

  // Validation
  isValid(): boolean {
    return (
      this._id.value.length > 0 &&
      this._metadata.isValid() &&
      this._stats.plays >= 0 &&
      this._stats.likes >= 0 &&
      this._stats.reposts >= 0 &&
      this._imageUrl.length > 0 &&
      this._audioUrl.length > 0
    );
  }

  // Factory Method
  static create(
    id: string,
    title: string,
    artist: string,
    artistId: string,
    duration: number,
    genre: string,
    mood: string,
    imageUrl: string,
    audioUrl: string,
    blockchainData?: BlockchainData
  ): Song {
    const songId: SongId = { value: id };
    const artistIdObj: ArtistId = { value: artistId };
    
    const metadata: SongMetadata = {
      title,
      artist,
      artistId: artistIdObj,
      duration,
      genre,
      mood,
      releaseDate: new Date().toISOString(),
      isValid: function() { 
        return this.title.length > 0 && this.artist.length > 0 && this.duration > 0; 
      },
      getDurationInMinutes: function() { return this.duration / 60; },
      getFormattedDuration: function() {
        const mins = Math.floor(this.duration / 60);
        const secs = this.duration % 60;
        return `${mins}:${secs.toString().padStart(2, '0')}`;
      }
    };

    const stats: SongStats = {
      plays: 0,
      likes: 0,
      reposts: 0,
      incrementPlays: function() { this.plays++; },
      incrementLikes: function() { this.likes++; },
      decrementLikes: function() { this.likes = Math.max(0, this.likes - 1); },
      incrementReposts: function() { this.reposts++; },
      decrementReposts: function() { this.reposts = Math.max(0, this.reposts - 1); },
      getEngagementRate: function() {
        return this.plays > 0 ? ((this.likes + this.reposts) / this.plays) * 100 : 0;
      }
    };

    const state: SongState = {
      isLiked: false,
      isReposted: false,
      isDownloaded: false,
      toggleLike: function() { this.isLiked = !this.isLiked; },
      toggleRepost: function() { this.isReposted = !this.isReposted; },
      toggleDownload: function() { this.isDownloaded = !this.isDownloaded; }
    };

    return new Song(songId, metadata, stats, state, imageUrl, audioUrl, blockchainData);
  }
} 