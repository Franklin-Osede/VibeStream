export interface UserProps {
  id: string;
  email: string;
  username: string;
  role: 'user' | 'artist' | 'admin';
  walletAddress?: string;
  isVerified: boolean;
  createdAt: Date;
}

export class User {
  private constructor(private props: UserProps) {}

  static create(props: UserProps): User {
    // Domain validations
    if (!props.email.includes('@')) {
      throw new Error('Invalid email format');
    }
    
    if (props.username.length < 3) {
      throw new Error('Username must be at least 3 characters');
    }

    return new User(props);
  }

  // Getters
  get id(): string { return this.props.id; }
  get email(): string { return this.props.email; }
  get username(): string { return this.props.username; }
  get role(): string { return this.props.role; }
  get walletAddress(): string | undefined { return this.props.walletAddress; }
  get isVerified(): boolean { return this.props.isVerified; }
  get createdAt(): Date { return this.props.createdAt; }

  // Domain methods
  canPurchaseMusic(): boolean {
    return this.props.role === 'user' || this.props.role === 'artist';
  }

  canCreateMusic(): boolean {
    return this.props.role === 'artist';
  }

  hasWalletConnected(): boolean {
    return !!this.props.walletAddress;
  }

  connectWallet(walletAddress: string): User {
    if (!walletAddress.startsWith('0x') && !walletAddress.startsWith('sol')) {
      throw new Error('Invalid wallet address format');
    }

    return new User({
      ...this.props,
      walletAddress
    });
  }

  toJSON(): UserProps {
    return { ...this.props };
  }
} 