export interface NFTMetadata {
  name: string;
  description: string;
  boostMultiplier: number;
  songId: string;
  image?: string;
  attributes?: Array<{
    trait_type: string;
    value: string | number;
  }>;
}

export interface CampaignNFTProps {
  id: string;
  campaignId: string;
  tokenId: number;
  ownerId?: string;
  price: number;
  metadata: NFTMetadata;
  contractAddress?: string;
  transactionHash?: string;
  isMinted: boolean;
  createdAt: Date;
}

export class CampaignNFT {
  private constructor(private props: CampaignNFTProps) {}

  static create(props: Omit<CampaignNFTProps, 'id' | 'isMinted' | 'createdAt'>): CampaignNFT {
    // Domain validations
    if (props.tokenId <= 0) {
      throw new Error('Token ID must be positive');
    }

    if (props.price <= 0) {
      throw new Error('NFT price must be positive');
    }

    if (!props.metadata.name || props.metadata.name.trim() === '') {
      throw new Error('NFT name is required');
    }

    return new CampaignNFT({
      ...props,
      id: crypto.randomUUID(),
      isMinted: false,
      createdAt: new Date()
    });
  }

  // Getters
  get id(): string { return this.props.id; }
  get campaignId(): string { return this.props.campaignId; }
  get tokenId(): number { return this.props.tokenId; }
  get ownerId(): string | undefined { return this.props.ownerId; }
  get price(): number { return this.props.price; }
  get metadata(): NFTMetadata { return { ...this.props.metadata }; }
  get contractAddress(): string | undefined { return this.props.contractAddress; }
  get transactionHash(): string | undefined { return this.props.transactionHash; }
  get isMinted(): boolean { return this.props.isMinted; }
  get createdAt(): Date { return this.props.createdAt; }

  // Domain Logic
  isOwned(): boolean {
    return !!this.props.ownerId;
  }

  canBeTransferred(): boolean {
    return this.props.isMinted && this.isOwned();
  }

  // Domain Actions
  mint(contractAddress: string, transactionHash: string): CampaignNFT {
    if (this.props.isMinted) {
      throw new Error('NFT is already minted');
    }

    return new CampaignNFT({
      ...this.props,
      contractAddress,
      transactionHash,
      isMinted: true
    });
  }

  assignOwner(ownerId: string): CampaignNFT {
    if (!this.props.isMinted) {
      throw new Error('Cannot assign owner to unminted NFT');
    }

    return new CampaignNFT({
      ...this.props,
      ownerId
    });
  }

  transfer(newOwnerId: string): CampaignNFT {
    if (!this.canBeTransferred()) {
      throw new Error('NFT cannot be transferred');
    }

    return new CampaignNFT({
      ...this.props,
      ownerId: newOwnerId
    });
  }

  toJSON(): CampaignNFTProps {
    return { ...this.props };
  }
} 