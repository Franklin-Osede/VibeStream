// Infrastructure Service: BackendSyncService
// Sincronización perfecta con el backend event-driven

import { User, UserId } from '../../domain/entities/User';
import { Song, SongId } from '../../domain/entities/Song';

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

export interface SyncState {
  lastEventId: string;
  lastSyncTimestamp: string;
  isConnected: boolean;
  pendingEvents: BackendEvent[];
}

export class BackendSyncService {
  private ws: WebSocket | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000;
  private eventHandlers: Map<string, Function[]> = new Map();
  private syncState: SyncState = {
    lastEventId: '',
    lastSyncTimestamp: new Date().toISOString(),
    isConnected: false,
    pendingEvents: []
  };

  constructor(
    private baseUrl: string = 'ws://localhost:8080',
    private apiUrl: string = 'http://localhost:8080/api'
  ) {}

  // Connection Management
  async connect(token: string): Promise<void> {
    try {
      this.ws = new WebSocket(`${this.baseUrl}/ws?token=${token}`);
      
      this.ws.onopen = () => {
        console.log('WebSocket connected');
        this.syncState.isConnected = true;
        this.reconnectAttempts = 0;
        this.syncPendingEvents();
      };

      this.ws.onmessage = (event) => {
        this.handleWebSocketMessage(event);
      };

      this.ws.onclose = () => {
        console.log('WebSocket disconnected');
        this.syncState.isConnected = false;
        this.handleReconnection();
      };

      this.ws.onerror = (error) => {
        console.error('WebSocket error:', error);
      };

    } catch (error) {
      console.error('Failed to connect to WebSocket:', error);
      throw error;
    }
  }

  disconnect(): void {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
    this.syncState.isConnected = false;
  }

  private handleReconnection(): void {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      setTimeout(() => {
        console.log(`Attempting to reconnect (${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
        // Reconnect logic would go here
      }, this.reconnectDelay * this.reconnectAttempts);
    }
  }

  // Event Handling
  subscribe(eventType: string, handler: Function): void {
    if (!this.eventHandlers.has(eventType)) {
      this.eventHandlers.set(eventType, []);
    }
    this.eventHandlers.get(eventType)!.push(handler);
  }

  unsubscribe(eventType: string, handler: Function): void {
    const handlers = this.eventHandlers.get(eventType);
    if (handlers) {
      const index = handlers.indexOf(handler);
      if (index > -1) {
        handlers.splice(index, 1);
      }
    }
  }

  private handleWebSocketMessage(event: MessageEvent): void {
    try {
      const message: WebSocketMessage = JSON.parse(event.data);
      
      switch (message.type) {
        case 'event':
          this.handleBackendEvent(message.payload as BackendEvent);
          break;
        case 'command':
          this.handleCommand(message.payload);
          break;
        case 'query':
          this.handleQuery(message.payload);
          break;
        default:
          console.warn('Unknown message type:', message.type);
      }
    } catch (error) {
      console.error('Error parsing WebSocket message:', error);
    }
  }

  private handleBackendEvent(event: BackendEvent): void {
    console.log('Received backend event:', event);
    
    // Update sync state
    this.syncState.lastEventId = event.id;
    this.syncState.lastSyncTimestamp = event.timestamp;
    
    // Notify event handlers
    const handlers = this.eventHandlers.get(event.type);
    if (handlers) {
      handlers.forEach(handler => {
        try {
          handler(event);
        } catch (error) {
          console.error('Error in event handler:', error);
        }
      });
    }

    // Handle specific event types
    switch (event.type) {
      case 'SongPlayed':
        this.handleSongPlayed(event);
        break;
      case 'SongLiked':
        this.handleSongLiked(event);
        break;
      case 'UserFollowed':
        this.handleUserFollowed(event);
        break;
      case 'VREventCreated':
        this.handleVREventCreated(event);
        break;
      case 'NFTMinted':
        this.handleNFTMinted(event);
        break;
      case 'TradeExecuted':
        this.handleTradeExecuted(event);
        break;
      default:
        console.log('Unhandled event type:', event.type);
    }
  }

  // Event-specific handlers
  private handleSongPlayed(event: BackendEvent): void {
    // Update local song stats
    console.log('Song played:', event.data);
  }

  private handleSongLiked(event: BackendEvent): void {
    // Update local song likes
    console.log('Song liked:', event.data);
  }

  private handleUserFollowed(event: BackendEvent): void {
    // Update local user relationships
    console.log('User followed:', event.data);
  }

  private handleVREventCreated(event: BackendEvent): void {
    // Add new VR event to local cache
    console.log('VR event created:', event.data);
  }

  private handleNFTMinted(event: BackendEvent): void {
    // Add new NFT to local cache
    console.log('NFT minted:', event.data);
  }

  private handleTradeExecuted(event: BackendEvent): void {
    // Update trading data
    console.log('Trade executed:', event.data);
  }

  // Command handling
  private handleCommand(command: any): void {
    console.log('Received command:', command);
    // Handle commands from backend
  }

  private handleQuery(query: any): void {
    console.log('Received query:', query);
    // Handle queries from backend
  }

  // Publishing events to backend
  async publishEvent(eventType: string, data: any): Promise<void> {
    const event: WebSocketMessage = {
      type: 'event',
      payload: {
        type: eventType,
        data,
        timestamp: new Date().toISOString()
      },
      timestamp: new Date().toISOString()
    };

    if (this.syncState.isConnected && this.ws) {
      this.ws.send(JSON.stringify(event));
    } else {
      // Store for later sync
      this.syncState.pendingEvents.push({
        id: Date.now().toString(),
        type: eventType,
        aggregateId: data.aggregateId || '',
        aggregateType: data.aggregateType || '',
        data,
        timestamp: new Date().toISOString(),
        version: 1
      });
    }
  }

  private async syncPendingEvents(): Promise<void> {
    if (this.syncState.pendingEvents.length === 0) return;

    console.log(`Syncing ${this.syncState.pendingEvents.length} pending events`);

    for (const event of this.syncState.pendingEvents) {
      try {
        await this.publishEvent(event.type, event.data);
      } catch (error) {
        console.error('Failed to sync event:', error);
      }
    }

    this.syncState.pendingEvents = [];
  }

  // REST API calls for initial data loading
  async fetchInitialData(userId: string): Promise<any> {
    try {
      const response = await fetch(`${this.apiUrl}/users/${userId}/initial-data`);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      return await response.json();
    } catch (error) {
      console.error('Failed to fetch initial data:', error);
      throw error;
    }
  }

  async fetchUserProfile(userId: string): Promise<any> {
    try {
      const response = await fetch(`${this.apiUrl}/users/${userId}`);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      return await response.json();
    } catch (error) {
      console.error('Failed to fetch user profile:', error);
      throw error;
    }
  }

  async fetchSongs(filters?: any): Promise<any[]> {
    try {
      const queryParams = filters ? `?${new URLSearchParams(filters)}` : '';
      const response = await fetch(`${this.apiUrl}/songs${queryParams}`);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      return await response.json();
    } catch (error) {
      console.error('Failed to fetch songs:', error);
      throw error;
    }
  }

  // Sync state management
  getSyncState(): SyncState {
    return { ...this.syncState };
  }

  isConnected(): boolean {
    return this.syncState.isConnected;
  }
} 