export interface SongProps {
  id: string;
  title: string;
  artistId: string;
  artistName: string;
  duration: number; // in seconds
  genre: string;
  ipfsHash?: string;
  nftContractAddress?: string;
  nftTokenId?: string;
  royaltyPercentage: number;
  price: number; // in ETH
  isMinted: boolean;
  createdAt: Date;
}

export class Song {
  private constructor(private props: SongProps) {}

  static create(props: SongProps): Song {
    // Domain validations
    if (props.title.length < 1) {
      throw new Error('Song title cannot be empty');
    }

    if (props.duration <= 0) {
      throw new Error('Song duration must be positive');
    }

    if (props.royaltyPercentage < 0 || props.royaltyPercentage > 100) {
      throw new Error('Royalty percentage must be between 0 and 100');
    }

    if (props.price < 0) {
      throw new Error('Price cannot be negative');
    }

    return new Song(props);
  }

  // Getters
  get id(): string { return this.props.id; }
  get title(): string { return this.props.title; }
  get artistId(): string { return this.props.artistId; }
  get artistName(): string { return this.props.artistName; }
  get duration(): number { return this.props.duration; }
  get genre(): string { return this.props.genre; }
  get ipfsHash(): string | undefined { return this.props.ipfsHash; }
  get nftContractAddress(): string | undefined { return this.props.nftContractAddress; }
  get nftTokenId(): string | undefined { return this.props.nftTokenId; }
  get royaltyPercentage(): number { return this.props.royaltyPercentage; }
  get price(): number { return this.props.price; }
  get isMinted(): boolean { return this.props.isMinted; }
  get createdAt(): Date { return this.props.createdAt; }

  // Domain methods
  isAvailableForPurchase(): boolean {
    return this.props.price > 0 && !this.props.isMinted;
  }

  canBeMinted(): boolean {
    return !!this.props.ipfsHash && !this.props.isMinted;
  }

  calculateArtistRoyalty(purchaseAmount: number): number {
    return (purchaseAmount * this.props.royaltyPercentage) / 100;
  }

  calculatePlatformFee(purchaseAmount: number): number {
    const royalty = this.calculateArtistRoyalty(purchaseAmount);
    return purchaseAmount - royalty;
  }

  formatDuration(): string {
    const minutes = Math.floor(this.props.duration / 60);
    const seconds = this.props.duration % 60;
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
  }

  mint(contractAddress: string, tokenId: string): Song {
    if (this.props.isMinted) {
      throw new Error('Song is already minted');
    }

    if (!this.props.ipfsHash) {
      throw new Error('Cannot mint song without IPFS hash');
    }

    return new Song({
      ...this.props,
      nftContractAddress: contractAddress,
      nftTokenId: tokenId,
      isMinted: true
    });
  }

  toJSON(): SongProps {
    return { ...this.props };
  }
} 