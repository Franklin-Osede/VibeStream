import { Campaign, CampaignProps } from '../entities/Campaign';
import { CampaignNFT } from '../entities/CampaignNFT';
import { DateRange } from '../value-objects/DateRange';
import { DomainEventDispatcher, CampaignCreated, CampaignActivated, NFTPurchased } from '../../shared/events/DomainEvent';

export interface CampaignAggregateProps {
  campaign: Campaign;
  nfts: CampaignNFT[];
  eventDispatcher?: DomainEventDispatcher;
}

/**
 * Campaign Aggregate Root
 * Encapsula Campaign y todos sus NFTs relacionados
 * Garantiza consistencia del dominio y maneja eventos
 */
export class CampaignAggregate {
  private constructor(
    private campaign: Campaign,
    private nfts: CampaignNFT[] = [],
    private eventDispatcher: DomainEventDispatcher = DomainEventDispatcher.getInstance()
  ) {}

  static create(props: Omit<CampaignProps, 'id' | 'nftsSold' | 'isActive' | 'createdAt'>): CampaignAggregate {
    const campaign = Campaign.create(props);
    const aggregate = new CampaignAggregate(campaign, []);
    
    // Dispatch domain event
    aggregate.eventDispatcher.dispatch(new CampaignCreated(
      campaign.id,
      {
        songId: campaign.songId,
        artistId: campaign.artistId,
        name: campaign.name,
        boostMultiplier: campaign.boostMultiplier,
        maxNFTs: campaign.maxNFTs,
        nftPrice: campaign.nftPrice,
        startDate: campaign.dateRange.start.toISOString(),
        endDate: campaign.dateRange.end.toISOString()
      }
    ));

    return aggregate;
  }

  static fromPersistence(campaign: Campaign, nfts: CampaignNFT[]): CampaignAggregate {
    return new CampaignAggregate(campaign, nfts);
  }

  // Getters for Campaign data
  get id(): string { return this.campaign.id; }
  get songId(): string { return this.campaign.songId; }
  get artistId(): string { return this.campaign.artistId; }
  get name(): string { return this.campaign.name; }
  get isActive(): boolean { return this.campaign.isActive; }
  get nftsSold(): number { return this.nfts.length; }
  
  // Override campaign's nftsSold to use aggregate's count
  getCampaignWithCorrectSoldCount(): Campaign {
    // Create a new campaign instance with correct sold count from aggregate
    const campaignData = this.campaign.toJSON();
    campaignData.nftsSold = this.nfts.length;
    
    // Use reflection to create campaign with correct data
    return Object.assign(Object.create(Object.getPrototypeOf(this.campaign)), {
      props: campaignData
    });
  }
  get availableNFTs(): number { return this.campaign.maxNFTs - this.nfts.length; }

  // Aggregate Business Logic
  canPurchaseNFT(): boolean {
    return this.campaign.canPurchaseNFT() && this.nfts.length < this.campaign.maxNFTs;
  }

  hasNFTsAvailable(): boolean {
    return this.nfts.length < this.campaign.maxNFTs;
  }

  getNFTByTokenId(tokenId: number): CampaignNFT | undefined {
    return this.nfts.find(nft => nft.tokenId === tokenId);
  }

  getUserNFTs(userId: string): CampaignNFT[] {
    return this.nfts.filter(nft => nft.ownerId === userId);
  }

  // Aggregate Actions (Maintain consistency across entities)
  async activate(): Promise<void> {
    if (!this.campaign.canBeActivated()) {
      throw new Error('Campaign cannot be activated');
    }

    this.campaign = this.campaign.activate();

    // Dispatch domain event
    await this.eventDispatcher.dispatch(new CampaignActivated(
      this.campaign.id,
      {
        songId: this.campaign.songId,
        artistId: this.campaign.artistId,
        activatedAt: new Date().toISOString()
      }
    ));
  }

  async purchaseNFT(buyerId: string): Promise<CampaignNFT> {
    if (!this.canPurchaseNFT()) {
      throw new Error('Cannot purchase NFT for this campaign');
    }

    // Create new NFT
    const nft = CampaignNFT.create({
      campaignId: this.campaign.id,
      tokenId: this.nfts.length + 1,
      price: this.campaign.nftPrice,
      ownerId: buyerId,
      metadata: {
        name: `${this.campaign.name} #${this.nfts.length + 1}`,
        description: this.campaign.description,
        boostMultiplier: this.campaign.boostMultiplier,
        songId: this.campaign.songId
      }
    });

    // Add to aggregate
    this.nfts.push(nft);

    // The campaign's nftsSold is automatically updated through the aggregate logic
    // No need to recreate the campaign here as we maintain consistency through the aggregate

    // Dispatch domain event
    await this.eventDispatcher.dispatch(new NFTPurchased(
      this.campaign.id,
      {
        nftId: nft.id,
        buyerId: buyerId,
        price: nft.price,
        tokenId: nft.tokenId
      }
    ));

    return nft;
  }

  calculateListenReward(baseReward: number): number {
    return this.campaign.calculateListenReward(baseReward);
  }

  getStats() {
    return this.campaign.getStats();
  }

  // Aggregate validation
  isConsistent(): boolean {
    // Verify that the number of NFTs matches the campaign's sold count
    const actualNFTsSold = this.nfts.length;
    const campaignNFTsSold = this.campaign.nftsSold;
    
    return actualNFTsSold === campaignNFTsSold;
  }

  // Aggregate invariants enforcement
  private enforceInvariants(): void {
    if (!this.isConsistent()) {
      throw new Error('Campaign aggregate is in an inconsistent state');
    }

    if (this.nfts.length > this.campaign.maxNFTs) {
      throw new Error('Cannot have more NFTs than maximum allowed');
    }

    // All NFTs must belong to this campaign
    const invalidNFTs = this.nfts.filter(nft => nft.campaignId !== this.campaign.id);
    if (invalidNFTs.length > 0) {
      throw new Error('All NFTs must belong to this campaign');
    }
  }

  // For persistence
  getCampaign(): Campaign {
    return this.campaign;
  }

  getNFTs(): CampaignNFT[] {
    return [...this.nfts]; // Return copy to prevent external modification
  }

  toJSON() {
    this.enforceInvariants();
    
    return {
      campaign: this.campaign.toJSON(),
      nfts: this.nfts.map(nft => nft.toJSON())
    };
  }
} 