// Backend Configuration - Infrastructure Layer
// Perfect sync with backend using DDD and Event-Driven Architecture

export interface BackendConfig {
  baseUrl: string;
  apiVersion: string;
  timeout: number;
  retryAttempts: number;
  websocketUrl: string;
}

export interface ApiEndpoints {
  // User Management
  auth: {
    login: string;
    register: string;
    refresh: string;
    logout: string;
  };
  
  // Music Domain
  music: {
    songs: string;
    playlists: string;
    albums: string;
    artists: string;
    genres: string;
    moods: string;
    play: string;
    like: string;
    repost: string;
    download: string;
  };
  
  // VR Events Domain
  vrEvents: {
    events: string;
    join: string;
    leave: string;
    live: string;
    recordings: string;
  };
  
  // NFT Domain
  nfts: {
    collections: string;
    tokens: string;
    buy: string;
    sell: string;
    transfer: string;
    metadata: string;
  };
  
  // Trading Domain
  trading: {
    positions: string;
    buy: string;
    sell: string;
    portfolio: string;
    market: string;
    history: string;
  };
  
  // Social Domain
  social: {
    posts: string;
    feed: string;
    follow: string;
    unfollow: string;
    followers: string;
    following: string;
  };
  
  // Analytics Domain
  analytics: {
    user: string;
    music: string;
    trading: string;
    earnings: string;
  };
  
  // WebSocket Events
  websocket: {
    music: string;
    social: string;
    trading: string;
    vr: string;
    notifications: string;
  };
}

// Development Configuration
export const devConfig: BackendConfig = {
  baseUrl: 'http://localhost:8080',
  apiVersion: 'v1',
  timeout: 10000,
  retryAttempts: 3,
  websocketUrl: 'ws://localhost:8080/ws'
};

// Production Configuration
export const prodConfig: BackendConfig = {
  baseUrl: 'https://api.vibestream.com',
  apiVersion: 'v1',
  timeout: 15000,
  retryAttempts: 5,
  websocketUrl: 'wss://api.vibestream.com/ws'
};

// Get configuration based on environment
export function getBackendConfig(): BackendConfig {
  return __DEV__ ? devConfig : prodConfig;
}

// Generate API endpoints
export function getApiEndpoints(): ApiEndpoints {
  const config = getBackendConfig();
  const base = `${config.baseUrl}/api/${config.apiVersion}`;
  
  return {
    auth: {
      login: `${base}/auth/login`,
      register: `${base}/auth/register`,
      refresh: `${base}/auth/refresh`,
      logout: `${base}/auth/logout`,
    },
    music: {
      songs: `${base}/music/songs`,
      playlists: `${base}/music/playlists`,
      albums: `${base}/music/albums`,
      artists: `${base}/music/artists`,
      genres: `${base}/music/genres`,
      moods: `${base}/music/moods`,
      play: `${base}/music/play`,
      like: `${base}/music/like`,
      repost: `${base}/music/repost`,
      download: `${base}/music/download`,
    },
    vrEvents: {
      events: `${base}/vr/events`,
      join: `${base}/vr/join`,
      leave: `${base}/vr/leave`,
      live: `${base}/vr/live`,
      recordings: `${base}/vr/recordings`,
    },
    nfts: {
      collections: `${base}/nfts/collections`,
      tokens: `${base}/nfts/tokens`,
      buy: `${base}/nfts/buy`,
      sell: `${base}/nfts/sell`,
      transfer: `${base}/nfts/transfer`,
      metadata: `${base}/nfts/metadata`,
    },
    trading: {
      positions: `${base}/trading/positions`,
      buy: `${base}/trading/buy`,
      sell: `${base}/trading/sell`,
      portfolio: `${base}/trading/portfolio`,
      market: `${base}/trading/market`,
      history: `${base}/trading/history`,
    },
    social: {
      posts: `${base}/social/posts`,
      feed: `${base}/social/feed`,
      follow: `${base}/social/follow`,
      unfollow: `${base}/social/unfollow`,
      followers: `${base}/social/followers`,
      following: `${base}/social/following`,
    },
    analytics: {
      user: `${base}/analytics/user`,
      music: `${base}/analytics/music`,
      trading: `${base}/analytics/trading`,
      earnings: `${base}/analytics/earnings`,
    },
    websocket: {
      music: `${config.websocketUrl}/music`,
      social: `${config.websocketUrl}/social`,
      trading: `${config.websocketUrl}/trading`,
      vr: `${config.websocketUrl}/vr`,
      notifications: `${config.websocketUrl}/notifications`,
    },
  };
}

// Event Types for WebSocket Communication
export enum EventType {
  // Music Events
  SONG_STARTED = 'SongStarted',
  SONG_PAUSED = 'SongPaused',
  SONG_RESUMED = 'SongResumed',
  SONG_STOPPED = 'SongStopped',
  PLAYBACK_PROGRESS = 'PlaybackProgress',
  
  // Social Events
  POST_CREATED = 'PostCreated',
  POST_LIKED = 'PostLiked',
  POST_REPOSTED = 'PostReposted',
  USER_FOLLOWED = 'UserFollowed',
  USER_UNFOLLOWED = 'UserUnfollowed',
  
  // VR Events
  VR_EVENT_STARTED = 'VREventStarted',
  VR_EVENT_ENDED = 'VREventEnded',
  VR_USER_JOINED = 'VRUserJoined',
  VR_USER_LEFT = 'VRUserLeft',
  
  // NFT Events
  NFT_MINTED = 'NFTMinted',
  NFT_SOLD = 'NFTSold',
  NFT_TRANSFERRED = 'NFTTransferred',
  
  // Trading Events
  TRADE_EXECUTED = 'TradeExecuted',
  PRICE_UPDATED = 'PriceUpdated',
  PORTFOLIO_UPDATED = 'PortfolioUpdated',
  
  // System Events
  NOTIFICATION_RECEIVED = 'NotificationReceived',
  BALANCE_UPDATED = 'BalanceUpdated',
  ERROR_OCCURRED = 'ErrorOccurred',
}

// Event Interfaces
export interface BaseEvent {
  eventType: EventType;
  timestamp: string;
  userId?: string;
  sessionId?: string;
}

export interface MusicEvent extends BaseEvent {
  eventType: EventType.SONG_STARTED | EventType.SONG_PAUSED | EventType.SONG_RESUMED | EventType.SONG_STOPPED;
  songId: string;
  eventData: {
    duration?: number;
    currentTime?: number;
    position?: number;
  };
}

export interface SocialEvent extends BaseEvent {
  eventType: EventType.POST_CREATED | EventType.POST_LIKED | EventType.POST_REPOSTED;
  postId: string;
  eventData: {
    content?: string;
    userId?: string;
    action?: string;
  };
}

export interface VREvent extends BaseEvent {
  eventType: EventType.VR_EVENT_STARTED | EventType.VR_EVENT_ENDED | EventType.VR_USER_JOINED | EventType.VR_USER_LEFT;
  eventId: string;
  eventData: {
    eventName?: string;
    attendees?: number;
    maxAttendees?: number;
  };
}

export interface TradingEvent extends BaseEvent {
  eventType: EventType.TRADE_EXECUTED | EventType.PRICE_UPDATED | EventType.PORTFOLIO_UPDATED;
  songId: string;
  eventData: {
    action?: 'buy' | 'sell';
    amount?: number;
    price?: number;
    totalValue?: number;
    profitLoss?: number;
  };
}

export type AppEvent = MusicEvent | SocialEvent | VREvent | TradingEvent;

// WebSocket Connection Manager
export class WebSocketManager {
  private connections: Map<string, WebSocket> = new Map();
  private eventHandlers: Map<EventType, Function[]> = new Map();
  private config = getBackendConfig();

  connect(channel: keyof ApiEndpoints['websocket']): WebSocket {
    const url = getApiEndpoints().websocket[channel];
    
    if (this.connections.has(channel)) {
      return this.connections.get(channel)!;
    }

    const ws = new WebSocket(url);
    
    ws.onopen = () => {
      console.log(`WebSocket connected to ${channel}`);
    };
    
    ws.onmessage = (event) => {
      try {
        const data: AppEvent = JSON.parse(event.data);
        this.handleEvent(data);
      } catch (error) {
        console.error('Failed to parse WebSocket message:', error);
      }
    };
    
    ws.onerror = (error) => {
      console.error(`WebSocket error on ${channel}:`, error);
    };
    
    ws.onclose = () => {
      console.log(`WebSocket disconnected from ${channel}`);
      this.connections.delete(channel);
    };

    this.connections.set(channel, ws);
    return ws;
  }

  disconnect(channel: string): void {
    const ws = this.connections.get(channel);
    if (ws) {
      ws.close();
      this.connections.delete(channel);
    }
  }

  subscribe(eventType: EventType, handler: Function): void {
    if (!this.eventHandlers.has(eventType)) {
      this.eventHandlers.set(eventType, []);
    }
    this.eventHandlers.get(eventType)!.push(handler);
  }

  unsubscribe(eventType: EventType, handler: Function): void {
    const handlers = this.eventHandlers.get(eventType);
    if (handlers) {
      const index = handlers.indexOf(handler);
      if (index > -1) {
        handlers.splice(index, 1);
      }
    }
  }

  private handleEvent(event: AppEvent): void {
    const handlers = this.eventHandlers.get(event.eventType);
    if (handlers) {
      handlers.forEach(handler => {
        try {
          handler(event);
        } catch (error) {
          console.error('Error in event handler:', error);
        }
      });
    }
  }

  publish(channel: string, event: AppEvent): void {
    const ws = this.connections.get(channel);
    if (ws && ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(event));
    }
  }
}

// Singleton instance
export const wsManager = new WebSocketManager(); 