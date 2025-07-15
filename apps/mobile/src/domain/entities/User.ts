// Domain Entity: User
// Siguiendo principios DDD - Rich Domain Model

export interface UserId {
  value: string;
}

export interface WalletAddress {
  value: string;
  isValid(): boolean;
}

export interface Balance {
  vibers: number;
  eth: number;
  
  addVibers(amount: number): Balance;
  subtractVibers(amount: number): Balance;
  addEth(amount: number): Balance;
  subtractEth(amount: number): Balance;
  hasEnoughVibers(amount: number): boolean;
  hasEnoughEth(amount: number): boolean;
}

export interface UserProfile {
  username: string;
  email: string;
  avatar: string;
  role: 'artist' | 'fan';
  followers: number;
  following: number;
  
  followUser(): void;
  unfollowUser(): void;
  isArtist(): boolean;
  isFan(): boolean;
}

export class User {
  private readonly _id: UserId;
  private _profile: UserProfile;
  private _walletAddress?: WalletAddress;
  private _balance: Balance;

  constructor(
    id: UserId,
    profile: UserProfile,
    balance: Balance,
    walletAddress?: WalletAddress
  ) {
    this._id = id;
    this._profile = profile;
    this._balance = balance;
    this._walletAddress = walletAddress;
  }

  // Identity
  get id(): UserId {
    return this._id;
  }

  // Profile
  get profile(): UserProfile {
    return this._profile;
  }

  // Wallet
  get walletAddress(): WalletAddress | undefined {
    return this._walletAddress;
  }

  setWalletAddress(address: WalletAddress): void {
    this._walletAddress = address;
  }

  // Balance
  get balance(): Balance {
    return this._balance;
  }

  // Business Logic
  canAffordVibers(amount: number): boolean {
    return this._balance.hasEnoughVibers(amount);
  }

  canAffordEth(amount: number): boolean {
    return this._balance.hasEnoughEth(amount);
  }

  purchaseWithVibers(amount: number): boolean {
    if (!this.canAffordVibers(amount)) {
      return false;
    }
    this._balance = this._balance.subtractVibers(amount);
    return true;
  }

  purchaseWithEth(amount: number): boolean {
    if (!this.canAffordEth(amount)) {
      return false;
    }
    this._balance = this._balance.subtractEth(amount);
    return true;
  }

  earnVibers(amount: number): void {
    this._balance = this._balance.addVibers(amount);
  }

  earnEth(amount: number): void {
    this._balance = this._balance.addEth(amount);
  }

  // Social Logic
  followUser(): void {
    this._profile.followUser();
  }

  unfollowUser(): void {
    this._profile.unfollowUser();
  }

  // Role-based Logic
  canCreateContent(): boolean {
    return this._profile.isArtist();
  }

  canInvest(): boolean {
    return this._profile.isFan();
  }

  // Validation
  isValid(): boolean {
    return (
      this._id.value.length > 0 &&
      this._profile.username.length > 0 &&
      this._profile.email.includes('@') &&
      this._balance.vibers >= 0 &&
      this._balance.eth >= 0
    );
  }

  // Factory Method
  static create(
    id: string,
    username: string,
    email: string,
    role: 'artist' | 'fan',
    avatar: string = 'https://via.placeholder.com/100'
  ): User {
    const userId: UserId = { value: id };
    
    const profile: UserProfile = {
      username,
      email,
      avatar,
      role,
      followers: 0,
      following: 0,
      followUser: function() { this.following++; },
      unfollowUser: function() { this.following = Math.max(0, this.following - 1); },
      isArtist: function() { return this.role === 'artist'; },
      isFan: function() { return this.role === 'fan'; }
    };

    const balance: Balance = {
      vibers: 0,
      eth: 0,
      addVibers: function(amount: number) { return { ...this, vibers: this.vibers + amount }; },
      subtractVibers: function(amount: number) { return { ...this, vibers: Math.max(0, this.vibers - amount) }; },
      addEth: function(amount: number) { return { ...this, eth: this.eth + amount }; },
      subtractEth: function(amount: number) { return { ...this, eth: Math.max(0, this.eth - amount) }; },
      hasEnoughVibers: function(amount: number) { return this.vibers >= amount; },
      hasEnoughEth: function(amount: number) { return this.eth >= amount; }
    };

    return new User(userId, profile, balance);
  }
} 