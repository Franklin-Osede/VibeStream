// HomeService - Application Service for Home Screen
// Following DDD principles and using mock data for demonstration

import { Post, Song, VREvent, NFT, TradingPosition, HomeFeedData } from '../types';

// Mock data with correct types
const mockPosts: Post[] = [
  {
    id: '1',
    userId: '1',
    username: 'Luna Echo',
    content: 'Just dropped my new track "Midnight Vibes"! 🎵✨ Available now on VibeStream with fractional ownership!',
    timestamp: new Date('2024-03-10T15:30:00Z'),
    likes: 234,
    comments: 45,
    reposts: 12,
    isLiked: false,
    isReposted: false
  }
];

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
  }
];

const mockVREvents: VREvent[] = [
  {
    id: '1',
    title: 'Neon Dreams VR Concert',
    artist: 'Cyber Collective',
    description: 'Experience the future of music in virtual reality',
    startTime: new Date('2024-03-15T20:00:00Z'),
    endTime: new Date('2024-03-15T22:00:00Z'),
    price: 0.1,
    currency: 'ETH',
    attendees: 150,
    maxAttendees: 200,
    imageUrl: 'https://via.placeholder.com/400',
    isLive: false,
    isJoined: true
  }
];

const mockNFTs: NFT[] = [
  {
    id: '1',
    title: 'Genesis Collection #1',
    artist: 'Luna Echo',
    description: 'The first NFT from Luna Echo\'s Genesis Collection',
    price: 0.5,
    currency: 'ETH',
    rarity: 'legendary',
    imageUrl: 'https://via.placeholder.com/300',
    isForSale: true,
    isOwned: false,
    tokenId: '1',
    contractAddress: '0x1234567890abcdef'
  }
];

const mockTradingPositions: TradingPosition[] = [
  {
    id: '1',
    songId: '1',
    songTitle: 'Midnight Vibes',
    artist: 'Luna Echo',
    shares: 100,
    averagePrice: 0.05,
    currentPrice: 0.06,
    totalValue: 6.0,
    profitLoss: 1.0,
    profitLossPercentage: 20.0
  }
];

export interface HomeService {
  getHomeFeed(): Promise<HomeFeedData>;
  likePost(postId: string): Promise<void>;
  repostPost(postId: string): Promise<void>;
  playSong(songId: string): Promise<void>;
  joinVREvent(eventId: string): Promise<void>;
  buyNFT(nftId: string): Promise<void>;
  tradeSong(songId: string, action: 'buy' | 'sell', amount: number): Promise<void>;
}

export class HomeServiceImpl implements HomeService {
  private posts: Post[] = [...mockPosts];
  private songs: Song[] = [...mockSongs];
  private vrEvents: VREvent[] = [...mockVREvents];
  private nfts: NFT[] = [...mockNFTs];
  private tradingPositions: TradingPosition[] = [...mockTradingPositions];

  async getHomeFeed(): Promise<HomeFeedData> {
    // Simulate API delay
    await new Promise(resolve => setTimeout(resolve, 500));

    return {
      recommendedSongs: this.songs.slice(0, 5),
      trendingSongs: this.songs.sort((a, b) => b.stats.plays - a.stats.plays).slice(0, 5),
      recentPosts: this.posts,
      liveVREvents: this.vrEvents.filter(event => event.isLive),
      featuredNFTs: this.nfts.filter(nft => nft.isForSale).slice(0, 3),
      tradingOpportunities: this.tradingPositions
        .sort((a, b) => Math.abs(b.profitLossPercentage) - Math.abs(a.profitLossPercentage))
        .slice(0, 3),
      userStats: {
        totalPlays: 1250000,
        totalLikes: 8900,
        vibersEarned: 2500,
        portfolioValue: 15000
      }
    };
  }

  async likePost(postId: string): Promise<void> {
    await new Promise(resolve => setTimeout(resolve, 200));
    
    const post = this.posts.find(p => p.id === postId);
    if (post) {
      if (post.isLiked) {
        post.likes--;
        post.isLiked = false;
      } else {
        post.likes++;
        post.isLiked = true;
      }
    }
  }

  async repostPost(postId: string): Promise<void> {
    await new Promise(resolve => setTimeout(resolve, 200));
    
    const post = this.posts.find(p => p.id === postId);
    if (post) {
      if (post.isReposted) {
        post.reposts--;
        post.isReposted = false;
      } else {
        post.reposts++;
        post.isReposted = true;
      }
    }
  }

  async playSong(songId: string): Promise<void> {
    await new Promise(resolve => setTimeout(resolve, 100));
    
    const song = this.songs.find(s => s.id === songId);
    if (song) {
      song.stats.plays++;
    }
  }

  async joinVREvent(eventId: string): Promise<void> {
    await new Promise(resolve => setTimeout(resolve, 300));
    
    const event = this.vrEvents.find(e => e.id === eventId);
    if (event && !event.isJoined) {
      event.attendees++;
      event.isJoined = true;
    }
  }

  async buyNFT(nftId: string): Promise<void> {
    await new Promise(resolve => setTimeout(resolve, 500));
    
    const nft = this.nfts.find(n => n.id === nftId);
    if (nft && nft.isForSale) {
      nft.isForSale = false;
      nft.isOwned = true;
    }
  }

  async tradeSong(songId: string, action: 'buy' | 'sell', amount: number): Promise<void> {
    await new Promise(resolve => setTimeout(resolve, 400));
    
    const position = this.tradingPositions.find(p => p.songId === songId);
    if (position) {
      if (action === 'buy') {
        position.shares += amount;
        position.averagePrice = (position.averagePrice + position.currentPrice) / 2;
      } else {
        position.shares = Math.max(0, position.shares - amount);
      }
      
      // Update position values
      position.totalValue = position.shares * position.currentPrice;
      position.profitLoss = position.totalValue - (position.shares * position.averagePrice);
      position.profitLossPercentage = (position.profitLoss / (position.shares * position.averagePrice)) * 100;
    }
  }
}

// Singleton instance
export const homeService = new HomeServiceImpl(); 