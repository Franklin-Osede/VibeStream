import { DateRange } from '../value-objects/DateRange';
import { MultiplierValue } from '../value-objects/MultiplierValue';
import { CampaignNFT } from './CampaignNFT';
import { CampaignStats } from './CampaignStats';

export interface CampaignProps {
  id: string;
  songId: string;
  artistId: string;
  name: string;
  description: string;
  boostMultiplier: number;
  dateRange: DateRange;
  nftPrice: number; // in $VIBERS
  maxNFTs: number;
  nftsSold: number;
  isActive: boolean;
  createdAt: Date;
}

export class Campaign {
  private constructor(private props: CampaignProps) {}

  static create(props: Omit<CampaignProps, 'id' | 'nftsSold' | 'isActive' | 'createdAt'>): Campaign {
    // Domain validations
    if (props.boostMultiplier <= 1) {
      throw new Error('Boost multiplier must be greater than 1');
    }

    if (props.maxNFTs <= 0) {
      throw new Error('Maximum NFTs must be positive');
    }

    if (props.nftPrice <= 0) {
      throw new Error('NFT price must be positive');
    }

    if (!props.dateRange.isValid()) {
      throw new Error('Invalid date range');
    }

    return new Campaign({
      ...props,
      id: crypto.randomUUID(),
      nftsSold: 0,
      isActive: false,
      createdAt: new Date()
    });
  }

  // Getters
  get id(): string { return this.props.id; }
  get songId(): string { return this.props.songId; }
  get artistId(): string { return this.props.artistId; }
  get name(): string { return this.props.name; }
  get description(): string { return this.props.description; }
  get boostMultiplier(): number { return this.props.boostMultiplier; }
  get dateRange(): DateRange { return this.props.dateRange; }
  get nftPrice(): number { return this.props.nftPrice; }
  get maxNFTs(): number { return this.props.maxNFTs; }
  get nftsSold(): number { return this.props.nftsSold; }
  get isActive(): boolean { return this.props.isActive; }
  get createdAt(): Date { return this.props.createdAt; }

  // Domain Business Logic
  canPurchaseNFT(): boolean {
    return this.isActive && 
           this.nftsSold < this.maxNFTs && 
           this.dateRange.isCurrentlyActive();
  }

  canBeActivated(): boolean {
    return !this.isActive && 
           this.dateRange.startsInFuture();
  }

  isExpired(): boolean {
    return this.dateRange.isExpired();
  }

  calculateListenReward(baseReward: number): number {
    if (!this.isActive || !this.dateRange.isCurrentlyActive()) {
      return baseReward;
    }
    return baseReward * this.boostMultiplier;
  }

  availableNFTs(): number {
    return this.maxNFTs - this.nftsSold;
  }

  completionPercentage(): number {
    return (this.nftsSold / this.maxNFTs) * 100;
  }

  // Domain Actions
  activate(): Campaign {
    if (!this.canBeActivated()) {
      throw new Error('Campaign cannot be activated');
    }

    return new Campaign({
      ...this.props,
      isActive: true
    });
  }

  purchaseNFT(): { campaign: Campaign; nft: CampaignNFT } {
    if (!this.canPurchaseNFT()) {
      throw new Error('Cannot purchase NFT for this campaign');
    }

    const nft = CampaignNFT.create({
      campaignId: this.id,
      tokenId: this.nftsSold + 1,
      price: this.nftPrice,
      metadata: {
        name: `${this.name} #${this.nftsSold + 1}`,
        description: this.description,
        boostMultiplier: this.boostMultiplier,
        songId: this.songId
      }
    });

    const updatedCampaign = new Campaign({
      ...this.props,
      nftsSold: this.nftsSold + 1
    });

    return { campaign: updatedCampaign, nft };
  }

  deactivate(): Campaign {
    return new Campaign({
      ...this.props,
      isActive: false
    });
  }

  updateMultiplier(newMultiplier: number): Campaign {
    if (newMultiplier <= 1) {
      throw new Error('Boost multiplier must be greater than 1');
    }

    if (this.isActive) {
      throw new Error('Cannot update multiplier of active campaign');
    }

    return new Campaign({
      ...this.props,
      boostMultiplier: newMultiplier
    });
  }

  getStats(): CampaignStats {
    return CampaignStats.create({
      campaignId: this.id,
      totalNFTsSold: this.nftsSold,
      totalRevenue: this.nftsSold * this.nftPrice,
      completionRate: this.completionPercentage(),
      isActive: this.isActive,
      daysRemaining: this.dateRange.daysUntilEnd()
    });
  }

  toJSON(): CampaignProps {
    return { ...this.props };
  }
} 