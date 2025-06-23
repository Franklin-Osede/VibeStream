export class MultiplierValue {
  private constructor(private readonly value: number) {
    if (value <= 1) {
      throw new Error('Multiplier must be greater than 1');
    }
    
    if (value > 10) {
      throw new Error('Multiplier cannot exceed 10x');
    }
    
    // Only allow one decimal place
    if (Math.round(value * 10) / 10 !== value) {
      throw new Error('Multiplier can only have one decimal place');
    }
  }

  static create(value: number): MultiplierValue {
    return new MultiplierValue(value);
  }

  static fromString(valueStr: string): MultiplierValue {
    const numValue = parseFloat(valueStr);
    if (isNaN(numValue)) {
      throw new Error('Invalid multiplier format');
    }
    return new MultiplierValue(numValue);
  }

  get raw(): number {
    return this.value;
  }

  // Predefined common multipliers
  static readonly BASIC = new MultiplierValue(1.5);
  static readonly STANDARD = new MultiplierValue(2.0);
  static readonly PREMIUM = new MultiplierValue(3.0);
  static readonly SUPER = new MultiplierValue(5.0);

  // Domain Logic
  isBasic(): boolean {
    return this.value >= 1.1 && this.value < 2.0;
  }

  isStandard(): boolean {
    return this.value >= 2.0 && this.value < 3.0;
  }

  isPremium(): boolean {
    return this.value >= 3.0 && this.value < 5.0;
  }

  isSuper(): boolean {
    return this.value >= 5.0;
  }

  getTier(): 'basic' | 'standard' | 'premium' | 'super' {
    if (this.isSuper()) return 'super';
    if (this.isPremium()) return 'premium';
    if (this.isStandard()) return 'standard';
    return 'basic';
  }

  calculateReward(baseAmount: number): number {
    if (baseAmount < 0) {
      throw new Error('Base amount cannot be negative');
    }
    return baseAmount * this.value;
  }

  equals(other: MultiplierValue): boolean {
    return this.value === other.value;
  }

  isGreaterThan(other: MultiplierValue): boolean {
    return this.value > other.value;
  }

  isLessThan(other: MultiplierValue): boolean {
    return this.value < other.value;
  }

  toString(): string {
    return `${this.value}x`;
  }

  toJSON(): number {
    return this.value;
  }
} 