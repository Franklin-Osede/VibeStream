export interface DomainEvent {
  readonly eventId: string;
  readonly eventType: string;
  readonly aggregateId: string;
  readonly aggregateType: string;
  readonly eventVersion: number;
  readonly occurredOn: Date;
  readonly eventData: Record<string, any>;
}

export abstract class BaseDomainEvent implements DomainEvent {
  public readonly eventId: string;
  public readonly eventVersion: number = 1;
  public readonly occurredOn: Date;

  constructor(
    public readonly eventType: string,
    public readonly aggregateId: string,
    public readonly aggregateType: string,
    public readonly eventData: Record<string, any>
  ) {
    this.eventId = crypto.randomUUID();
    this.occurredOn = new Date();
  }
}

// Campaign Domain Events
export class CampaignCreated extends BaseDomainEvent {
  constructor(
    campaignId: string,
    eventData: {
      songId: string;
      artistId: string;
      name: string;
      boostMultiplier: number;
      maxNFTs: number;
      nftPrice: number;
      startDate: string;
      endDate: string;
    }
  ) {
    super('CampaignCreated', campaignId, 'Campaign', eventData);
  }
}

export class CampaignActivated extends BaseDomainEvent {
  constructor(
    campaignId: string,
    eventData: {
      songId: string;
      artistId: string;
      activatedAt: string;
    }
  ) {
    super('CampaignActivated', campaignId, 'Campaign', eventData);
  }
}

export class NFTPurchased extends BaseDomainEvent {
  constructor(
    campaignId: string,
    eventData: {
      nftId: string;
      buyerId: string;
      price: number;
      tokenId: number;
      transactionHash?: string;
    }
  ) {
    super('NFTPurchased', campaignId, 'Campaign', eventData);
  }
}

// Listen Domain Events
export class ListenSessionStarted extends BaseDomainEvent {
  constructor(
    sessionId: string,
    eventData: {
      userId?: string;
      songId: string;
      deviceFingerprint: string;
      startedAt: string;
    }
  ) {
    super('ListenSessionStarted', sessionId, 'ListenSession', eventData);
  }
}

export class ListenSessionCompleted extends BaseDomainEvent {
  constructor(
    sessionId: string,
    eventData: {
      userId?: string;
      songId: string;
      duration: number;
      zkProofHash: string;
      completedAt: string;
    }
  ) {
    super('ListenSessionCompleted', sessionId, 'ListenSession', eventData);
  }
}

export class RewardCalculated extends BaseDomainEvent {
  constructor(
    sessionId: string,
    eventData: {
      userId?: string;
      songId: string;
      baseReward: number;
      boostedReward: number;
      campaignId?: string;
      multiplier?: number;
    }
  ) {
    super('RewardCalculated', sessionId, 'ListenSession', eventData);
  }
}

// Music Domain Events
export class SongCreated extends BaseDomainEvent {
  constructor(
    songId: string,
    eventData: {
      title: string;
      artistId: string;
      duration: number;
      genre: string;
      ipfsHash?: string;
    }
  ) {
    super('SongCreated', songId, 'Song', eventData);
  }
}

// Domain Event Dispatcher
export type DomainEventHandler<T extends DomainEvent> = (event: T) => Promise<void> | void;

export class DomainEventDispatcher {
  private static instance: DomainEventDispatcher;
  private handlers: Map<string, DomainEventHandler<any>[]> = new Map();

  static getInstance(): DomainEventDispatcher {
    if (!DomainEventDispatcher.instance) {
      DomainEventDispatcher.instance = new DomainEventDispatcher();
    }
    return DomainEventDispatcher.instance;
  }

  subscribe<T extends DomainEvent>(
    eventType: string, 
    handler: DomainEventHandler<T>
  ): void {
    if (!this.handlers.has(eventType)) {
      this.handlers.set(eventType, []);
    }
    this.handlers.get(eventType)!.push(handler);
  }

  async dispatch(event: DomainEvent): Promise<void> {
    const handlers = this.handlers.get(event.eventType) || [];
    
    await Promise.all(
      handlers.map(handler => {
        try {
          return handler(event);
        } catch (error) {
          console.error(`Error handling event ${event.eventType}:`, error);
          throw error;
        }
      })
    );
  }

  clear(): void {
    this.handlers.clear();
  }
} 