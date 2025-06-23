import { Campaign } from '../entities/Campaign';
import { CampaignNFT } from '../entities/CampaignNFT';
import { CampaignStats } from '../entities/CampaignStats';
import { MultiplierValue } from '../value-objects/MultiplierValue';

/**
 * Domain Service para lógica de negocio compleja que:
 * - No pertenece a una entidad específica
 * - Requiere múltiples agregados
 * - Implementa reglas de negocio del dominio
 */
export class CampaignDomainService {
  
  /**
   * Calcula el precio dinámico de NFT basado en demanda y scarcity
   */
  calculateDynamicNFTPrice(
    campaign: Campaign, 
    nftsSold: number, 
    timeRemaining: number
  ): number {
    const basePrice = campaign.nftPrice;
    const soldPercentage = nftsSold / campaign.maxNFTs;
    const timeProgressPercentage = 1 - (timeRemaining / campaign.dateRange.totalDays());

    // Scarcity multiplier - price increases as NFTs are sold
    let scarcityMultiplier = 1;
    if (soldPercentage > 0.8) {
      scarcityMultiplier = 1.5; // 50% increase when 80%+ sold
    } else if (soldPercentage > 0.5) {
      scarcityMultiplier = 1.25; // 25% increase when 50%+ sold
    }

    // Time urgency multiplier - price increases as time runs out
    let urgencyMultiplier = 1;
    if (timeProgressPercentage > 0.8) {
      urgencyMultiplier = 1.3; // 30% increase in final 20% of time
    } else if (timeProgressPercentage > 0.6) {
      urgencyMultiplier = 1.15; // 15% increase in final 40% of time
    }

    return Math.round(basePrice * scarcityMultiplier * urgencyMultiplier * 100) / 100;
  }

  /**
   * Valida si un multiplicador es apropiado para el tipo de campaña
   */
  validateMultiplierForCampaignType(
    multiplier: MultiplierValue,
    campaignType: 'standard' | 'premium' | 'exclusive'
  ): boolean {
    switch (campaignType) {
      case 'standard':
        return multiplier.raw <= 2.5;
      case 'premium':
        return multiplier.raw >= 2.0 && multiplier.raw <= 5.0;
      case 'exclusive':
        return multiplier.raw >= 3.0;
      default:
        return false;
    }
  }

  /**
   * Calcula las recompensas totales distribuidas por una campaña
   */
  calculateTotalRewardsDistributed(
    campaign: Campaign,
    listensData: Array<{ userId: string; listens: number; hasNFT: boolean }>
  ): number {
    return listensData.reduce((total, userListens) => {
      const baseReward = userListens.listens * 0.1; // Base reward per listen
      const reward = userListens.hasNFT 
        ? campaign.calculateListenReward(baseReward)
        : baseReward;
      return total + reward;
    }, 0);
  }

  /**
   * Determina si una campaña debe ser promovida basado en performance
   */
  shouldPromoteCampaign(
    stats: CampaignStats,
    benchmarkStats: { averageCompletion: number; averageRevenue: number }
  ): boolean {
    const isAboveAverageCompletion = stats.completionRate > benchmarkStats.averageCompletion;
    const isAboveAverageRevenue = stats.totalRevenue > benchmarkStats.averageRevenue;
    const hasStrongVelocity = stats.salesVelocity() > 1; // More than 1 NFT per day

    return isAboveAverageCompletion && (isAboveAverageRevenue || hasStrongVelocity);
  }

  /**
   * Calcula la distribución óptima de NFTs para maximizar engagement
   */
  calculateOptimalNFTDistribution(
    totalNFTs: number,
    expectedParticipants: number,
    campaignDuration: number
  ): {
    earlyBird: number;
    regular: number;
    lastChance: number;
  } {
    // Early bird phase (first 20% of time): 30% of NFTs
    const earlyBird = Math.floor(totalNFTs * 0.3);
    
    // Last chance phase (final 20% of time): 20% of NFTs
    const lastChance = Math.floor(totalNFTs * 0.2);
    
    // Regular phase (middle 60% of time): remaining NFTs
    const regular = totalNFTs - earlyBird - lastChance;

    return { earlyBird, regular, lastChance };
  }

  /**
   * Evalúa el riesgo de una campaña basado en múltiples factores
   */
  evaluateCampaignRisk(campaign: Campaign): {
    riskLevel: 'low' | 'medium' | 'high';
    factors: string[];
  } {
    const factors: string[] = [];
    let riskScore = 0;

    // Check campaign duration
    const duration = campaign.dateRange.totalDays();
    if (duration < 7) {
      factors.push('Very short campaign duration');
      riskScore += 2;
    } else if (duration > 60) {
      factors.push('Very long campaign duration');
      riskScore += 1;
    }

    // Check NFT price relative to multiplier
    const priceToMultiplierRatio = campaign.nftPrice / campaign.boostMultiplier;
    if (priceToMultiplierRatio > 50) {
      factors.push('High price relative to boost multiplier');
      riskScore += 2;
    }

    // Check total NFT supply
    if (campaign.maxNFTs > 1000) {
      factors.push('Very high NFT supply');
      riskScore += 1;
    } else if (campaign.maxNFTs < 10) {
      factors.push('Very low NFT supply');
      riskScore += 1;
    }

    let riskLevel: 'low' | 'medium' | 'high';
    if (riskScore >= 4) {
      riskLevel = 'high';
    } else if (riskScore >= 2) {
      riskLevel = 'medium';
    } else {
      riskLevel = 'low';
    }

    return { riskLevel, factors };
  }

  /**
   * Sugiere optimizaciones para mejorar el rendimiento de una campaña
   */
  suggestCampaignOptimizations(
    campaign: Campaign,
    stats: CampaignStats
  ): Array<{
    type: 'pricing' | 'duration' | 'multiplier' | 'marketing';
    suggestion: string;
    impact: 'low' | 'medium' | 'high';
  }> {
    const suggestions = [];

    // Pricing optimization
    if (stats.completionRate < 30 && stats.daysRemaining > 7) {
      suggestions.push({
        type: 'pricing' as const,
        suggestion: 'Consider reducing NFT price by 10-20% to increase demand',
        impact: 'high' as const
      });
    }

    // Multiplier optimization
    if (stats.completionRate > 80 && stats.daysRemaining > 14) {
      suggestions.push({
        type: 'multiplier' as const,
        suggestion: 'Multiplier could be increased for future campaigns',
        impact: 'medium' as const
      });
    }

    // Marketing optimization
    if (stats.salesVelocity() < 0.5 && stats.daysRemaining > 5) {
      suggestions.push({
        type: 'marketing' as const,
        suggestion: 'Increase marketing efforts to boost awareness',
        impact: 'high' as const
      });
    }

    // Duration optimization
    if (stats.completionRate > 90 && stats.daysRemaining > 10) {
      suggestions.push({
        type: 'duration' as const,
        suggestion: 'Campaign could end early due to high success rate',
        impact: 'low' as const
      });
    }

    return suggestions;
  }
} 