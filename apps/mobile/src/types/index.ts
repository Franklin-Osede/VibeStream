// Types: index.ts
// Tipos centralizados para toda la aplicación

// User types
export interface User {
  id: string;
  username: string;
  email: string;
  profile: {
    avatar?: string;
    bio?: string;
    location?: string;
    website?: string;
  };
  wallet: {
    address: string;
    balance: {
      eth: number;
      vibers: number;
    };
  };
  stats: {
    followers: number;
    following: number;
    totalPlays: number;
    totalLikes: number;
  };
  role: 'fan' | 'artist' | 'admin';
  isVerified: boolean;
  createdAt: Date;
  updatedAt: Date;
}

// Song types
export interface Song {
  id: string;
  title: string;
  artist: string;
  artistId: string;
  duration: number;
  genre: string;
  mood: string;
  imageUrl: string;
  audioUrl: string;
  stats: {
    plays: number;
    likes: number;
    reposts: number;
  };
  interactions: {
    isLiked: boolean;
    isReposted: boolean;
    isInLibrary: boolean;
  };
  blockchain?: {
    tokenId?: string;
    contractAddress?: string;
    royalties: number;
    fractionalOwnership: boolean;
    totalShares: number;
    availableShares: number;
    sharePrice: number;
  };
  createdAt: Date;
  updatedAt: Date;
}

// Post types
export interface Post {
  id: string;
  userId: string;
  username: string;
  content: string;
  imageUrl?: string;
  timestamp: Date;
  likes: number;
  comments: number;
  reposts: number;
  isLiked: boolean;
  isReposted: boolean;
}

// VR Event types
export interface VREvent {
  id: string;
  title: string;
  artist: string;
  description: string;
  startTime: Date;
  endTime: Date;
  attendees: number;
  maxAttendees: number;
  price: number;
  currency: string;
  imageUrl: string;
  isLive: boolean;
  isJoined: boolean;
}

// NFT types
export interface NFT {
  id: string;
  title: string;
  artist: string;
  description: string;
  price: number;
  currency: string;
  rarity: 'common' | 'rare' | 'epic' | 'legendary';
  imageUrl: string;
  isForSale: boolean;
  isOwned: boolean;
  tokenId: string;
  contractAddress: string;
}

// Trading types
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

// Navigation types
export type RootStackParamList = {
  Login: undefined;
  RoleSelection: { user: User; token: string };
  ArtistDashboard: { user: User; token: string };
  FanDashboard: { user: User; token: string };
  MusicExplore: { user: User; token: string };
  MainApp: { user: User; token: string };
};

export type TabParamList = {
  Home: undefined;
  Trending: undefined;
  Explore: undefined;
  Library: undefined;
  Notifications: undefined;
};

// API Response types
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}

export interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    totalPages: number;
  };
}

// Event types
export interface BackendEvent {
  id: string;
  type: string;
  aggregateId: string;
  aggregateType: string;
  data: any;
  timestamp: string;
  version: number;
}

export interface WebSocketMessage {
  type: 'event' | 'command' | 'query';
  payload: any;
  timestamp: string;
}

// Home Feed types
export interface HomeFeedData {
  recommendedSongs: Song[];
  trendingSongs: Song[];
  recentPosts: Post[];
  liveVREvents: VREvent[];
  featuredNFTs: NFT[];
  tradingOpportunities: TradingPosition[];
  userStats: {
    totalPlays: number;
    totalLikes: number;
    vibersEarned: number;
    portfolioValue: number;
  };
}

// Context types
export interface UserContextType {
  user: User | null;
  token: string | null;
  setUser: (user: User | null) => void;
  setToken: (token: string | null) => void;
  logout: () => void;
  isAuthenticated: boolean;
  loading: boolean;
  error: string | null;
  updateUserProfile: (profile: Partial<User['profile']>) => Promise<void>;
  followUser: (userId: string) => Promise<void>;
  unfollowUser: (userId: string) => Promise<void>;
  addVibers: (amount: number) => Promise<void>;
  spendVibers: (amount: number) => Promise<boolean>;
  addEth: (amount: number) => Promise<void>;
  spendEth: (amount: number) => Promise<boolean>;
}

// Hook types
export interface HomeScreenState {
  loading: boolean;
  error: string | null;
  data: HomeFeedData | null;
  refreshing: boolean;
  syncStatus: {
    isConnected: boolean;
    lastSync: string;
    pendingEvents: number;
  };
}

// Repository types
export interface UserId {
  value: string;
}

export interface SongId {
  value: string;
}

// Auth types
export interface LoginCredentials {
  email: string;
  password: string;
}

export interface AuthResult {
  user: User;
  token: string;
}

export interface CreateUserData {
  username: string;
  email: string;
  password: string;
  role: 'fan' | 'artist';
}

// Purchase types
export interface PurchaseResult {
  success: boolean;
  transactionId: string;
  totalPaid: number;
  royalties: number;
  platformFee: number;
} 