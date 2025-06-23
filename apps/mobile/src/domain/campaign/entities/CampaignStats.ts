export interface CampaignStatsProps {
  campaignId: string;
  totalNFTsSold: number;
  totalRevenue: number;
  completionRate: number;
  isActive: boolean;
  daysRemaining: number;
  averagePurchaseTime?: number; // hours since campaign start
  topBuyerRegions?: string[];
  revenueByDay?: Array<{ date: string; revenue: number }>;
}

export class CampaignStats {
  private constructor(private props: CampaignStatsProps) {}

  static create(props: CampaignStatsProps): CampaignStats {
    // Domain validations
    if (props.totalNFTsSold < 0) {
      throw new Error('Total NFTs sold cannot be negative');
    }

    if (props.totalRevenue < 0) {
      throw new Error('Total revenue cannot be negative');
    }

    if (props.completionRate < 0 || props.completionRate > 100) {
      throw new Error('Completion rate must be between 0 and 100');
    }

    return new CampaignStats(props);
  }

  // Getters
  get campaignId(): string { return this.props.campaignId; }
  get totalNFTsSold(): number { return this.props.totalNFTsSold; }
  get totalRevenue(): number { return this.props.totalRevenue; }
  get completionRate(): number { return this.props.completionRate; }
  get isActive(): boolean { return this.props.isActive; }
  get daysRemaining(): number { return this.props.daysRemaining; }
  get averagePurchaseTime(): number | undefined { return this.props.averagePurchaseTime; }
  get topBuyerRegions(): string[] | undefined { return this.props.topBuyerRegions; }
  get revenueByDay(): Array<{ date: string; revenue: number }> | undefined { 
    return this.props.revenueByDay; 
  }

  // Domain Logic
  isSuccessful(): boolean {
    return this.props.completionRate >= 50; // 50% or more considered successful
  }

  isNearCompletion(): boolean {
    return this.props.completionRate >= 80;
  }

  hasStrongSales(): boolean {
    return this.props.totalNFTsSold > 10 && this.props.completionRate > 25;
  }

  averageRevenuePerNFT(): number {
    if (this.props.totalNFTsSold === 0) return 0;
    return this.props.totalRevenue / this.props.totalNFTsSold;
  }

  salesVelocity(): number {
    // NFTs sold per day (assuming campaign just started for simplicity)
    if (this.props.daysRemaining === 0) return this.props.totalNFTsSold;
    
    const daysElapsed = Math.max(1, 30 - this.props.daysRemaining); // Assuming 30-day campaigns
    return this.props.totalNFTsSold / daysElapsed;
  }

  projectedFinalSales(): number {
    const velocity = this.salesVelocity();
    const totalDays = 30; // Assuming 30-day campaigns
    return velocity * totalDays;
  }

  getPerformanceLevel(): 'excellent' | 'good' | 'average' | 'poor' {
    if (this.props.completionRate >= 80) return 'excellent';
    if (this.props.completionRate >= 60) return 'good';
    if (this.props.completionRate >= 30) return 'average';
    return 'poor';
  }

  getDailyAverageRevenue(): number {
    if (!this.props.revenueByDay || this.props.revenueByDay.length === 0) {
      return 0;
    }

    const totalDailyRevenue = this.props.revenueByDay.reduce(
      (sum, day) => sum + day.revenue, 
      0
    );
    return totalDailyRevenue / this.props.revenueByDay.length;
  }

  toJSON(): CampaignStatsProps {
    return { ...this.props };
  }
} 