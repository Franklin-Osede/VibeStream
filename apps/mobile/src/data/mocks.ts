// Mocks de datos para VibeStream - Demostración completa

export interface User {
  id: string;
  username: string;
  email: string;
  avatar: string;
  role: 'artist' | 'fan';
  followers: number;
  following: number;
  walletAddress?: string;
  balance: {
    vibers: number;
    eth: number;
  };
}

export interface Song {
  id: string;
  title: string;
  artist: string;
  artistId: string;
  duration: number;
  imageUrl: string;
  audioUrl: string;
  genre: string;
  mood: string;
  plays: number;
  likes: number;
  reposts: number;
  isLiked: boolean;
  isReposted: boolean;
  isDownloaded: boolean;
  releaseDate: string;
  blockchainData: {
    tokenId?: string;
    contractAddress?: string;
    royalties: number;
    fractionalOwnership: boolean;
  };
}

export interface VREvent {
  id: string;
  title: string;
  artist: string;
  artistId: string;
  imageUrl: string;
  description: string;
  startTime: string;
  endTime: string;
  price: number;
  currency: string;
  attendees: number;
  maxAttendees: number;
  isLive: boolean;
  isJoined: boolean;
  vrPlatform: string;
  vrUrl?: string;
}

export interface NFT {
  id: string;
  title: string;
  artist: string;
  artistId: string;
  imageUrl: string;
  description: string;
  price: number;
  currency: string;
  rarity: 'common' | 'rare' | 'epic' | 'legendary';
  tokenId: string;
  contractAddress: string;
  isOwned: boolean;
  isForSale: boolean;
  collection: string;
  attributes: Array<{
    trait: string;
    value: string;
  }>;
}

export interface TradingPosition {
  id: string;
  songId: string;
  songTitle: string;
  artist: string;
  shares: number;
  averagePrice: number;
  currentPrice: number;
  totalValue: number;
  profitLoss: number;
  profitLossPercentage: number;
}

export interface Post {
  id: string;
  userId: string;
  username: string;
  userAvatar: string;
  content: string;
  imageUrl?: string;
  songId?: string;
  song?: Song;
  timestamp: string;
  likes: number;
  comments: number;
  reposts: number;
  isLiked: boolean;
  isReposted: boolean;
  type: 'text' | 'song' | 'vr_event' | 'nft' | 'trading';
}

export interface Notification {
  id: string;
  type: 'like' | 'comment' | 'follow' | 'vr_event' | 'nft' | 'trading' | 'system';
  title: string;
  message: string;
  imageUrl?: string;
  timestamp: string;
  isRead: boolean;
  metadata?: any;
}

// Mocks de datos
export const mockUsers: User[] = [
  {
    id: '1',
    username: 'Luna Echo',
    email: 'luna@vibestream.com',
    avatar: 'https://via.placeholder.com/100',
    role: 'artist',
    followers: 12500,
    following: 89,
    walletAddress: '0x1234567890abcdef',
    balance: { vibers: 50000, eth: 2.5 }
  },
  {
    id: '2',
    username: 'Cyber Collective',
    email: 'cyber@vibestream.com',
    avatar: 'https://via.placeholder.com/100',
    role: 'artist',
    followers: 8900,
    following: 156,
    walletAddress: '0xabcdef1234567890',
    balance: { vibers: 32000, eth: 1.8 }
  }
];

export const mockSongs: Song[] = [
  {
    id: '1',
    title: 'Midnight Vibes',
    artist: 'Luna Echo',
    artistId: '1',
    duration: 180,
    imageUrl: 'https://via.placeholder.com/300',
    audioUrl: 'https://example.com/midnight-vibes.mp3',
    genre: 'Electronic',
    mood: 'Chill',
    plays: 1250000,
    likes: 8900,
    reposts: 1200,
    isLiked: false,
    isReposted: false,
    isDownloaded: false,
    releaseDate: '2024-01-15',
    blockchainData: {
      tokenId: '1',
      contractAddress: '0x1234567890abcdef',
      royalties: 10,
      fractionalOwnership: true
    }
  },
  {
    id: '2',
    title: 'Neon Dreams',
    artist: 'Cyber Collective',
    artistId: '2',
    duration: 210,
    imageUrl: 'https://via.placeholder.com/300',
    audioUrl: 'https://example.com/neon-dreams.mp3',
    genre: 'Synthwave',
    mood: 'Energetic',
    plays: 890000,
    likes: 5600,
    reposts: 890,
    isLiked: true,
    isReposted: false,
    isDownloaded: true,
    releaseDate: '2024-02-01',
    blockchainData: {
      tokenId: '2',
      contractAddress: '0xabcdef1234567890',
      royalties: 15,
      fractionalOwnership: true
    }
  }
];

export const mockVREvents: VREvent[] = [
  {
    id: '1',
    title: 'Neon Dreams VR Concert',
    artist: 'Cyber Collective',
    artistId: '2',
    imageUrl: 'https://via.placeholder.com/400',
    description: 'Experience the future of music in virtual reality',
    startTime: '2024-03-15T20:00:00Z',
    endTime: '2024-03-15T22:00:00Z',
    price: 0.1,
    currency: 'ETH',
    attendees: 150,
    maxAttendees: 200,
    isLive: false,
    isJoined: true,
    vrPlatform: 'Meta Quest',
    vrUrl: 'https://vr.vibestream.com/neon-dreams'
  }
];

export const mockNFTs: NFT[] = [
  {
    id: '1',
    title: 'Genesis Collection #1',
    artist: 'Luna Echo',
    artistId: '1',
    imageUrl: 'https://via.placeholder.com/300',
    description: 'The first NFT from Luna Echo\'s Genesis Collection',
    price: 0.5,
    currency: 'ETH',
    rarity: 'legendary',
    tokenId: '1',
    contractAddress: '0x1234567890abcdef',
    isOwned: false,
    isForSale: true,
    collection: 'Genesis Collection',
    attributes: [
      { trait: 'Background', value: 'Neon Purple' },
      { trait: 'Instrument', value: 'Synthesizer' },
      { trait: 'Mood', value: 'Mysterious' }
    ]
  }
];

export const mockPosts: Post[] = [
  {
    id: '1',
    userId: '1',
    username: 'Luna Echo',
    userAvatar: 'https://via.placeholder.com/50',
    content: 'Just dropped my new track "Midnight Vibes"! 🎵✨ Available now on VibeStream with fractional ownership!',
    songId: '1',
    song: mockSongs[0],
    timestamp: '2024-03-10T15:30:00Z',
    likes: 234,
    comments: 45,
    reposts: 12,
    isLiked: false,
    isReposted: false,
    type: 'song'
  }
];

export const mockTradingPositions: TradingPosition[] = [
  {
    id: '1',
    songId: '1',
    songTitle: 'Midnight Vibes',
    artist: 'Luna Echo',
    shares: 100,
    averagePrice: 0.05,
    currentPrice: 0.08,
    totalValue: 8.0,
    profitLoss: 3.0,
    profitLossPercentage: 60.0
  },
  {
    id: '2',
    songId: '2',
    songTitle: 'Neon Dreams',
    artist: 'Cyber Collective',
    shares: 50,
    averagePrice: 0.06,
    currentPrice: 0.04,
    totalValue: 2.0,
    profitLoss: -1.0,
    profitLossPercentage: -33.3
  }
]; 