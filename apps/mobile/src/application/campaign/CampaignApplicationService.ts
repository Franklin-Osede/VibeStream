import { CampaignAggregate } from '../../domain/campaign/aggregates/CampaignAggregate';
import { Campaign } from '../../domain/campaign/entities/Campaign';
import { CampaignDomainService } from '../../domain/campaign/services/CampaignDomainService';
import { DateRange } from '../../domain/campaign/value-objects/DateRange';
import { MultiplierValue } from '../../domain/campaign/value-objects/MultiplierValue';

// Interfaces para repositorios (puertos)
export interface CampaignRepository {
  save(aggregate: CampaignAggregate): Promise<void>;
  findById(id: string): Promise<CampaignAggregate | null>;
  findBySongId(songId: string): Promise<CampaignAggregate[]>;
  findActiveByArtistId(artistId: string): Promise<CampaignAggregate[]>;
  findAll(): Promise<CampaignAggregate[]>;
}

export interface PaymentService {
  processNFTPurchase(userId: string, amount: number, nftId: string): Promise<{ success: boolean; transactionId?: string }>;
}

export interface NotificationService {
  sendCampaignCreated(campaignId: string, artistId: string): Promise<void>;
  sendNFTPurchased(buyerId: string, campaignId: string, nftId: string): Promise<void>;
}

// DTOs para entrada y salida
export interface CreateCampaignCommand {
  songId: string;
  artistId: string;
  name: string;
  description: string;
  boostMultiplier: number;
  maxNFTs: number;
  nftPrice: number;
  startDate: Date;
  endDate: Date;
}

export interface PurchaseNFTCommand {
  campaignId: string;
  buyerId: string;
  paymentMethod: string;
}

export interface CampaignDTO {
  id: string;
  songId: string;
  artistId: string;
  name: string;
  description: string;
  boostMultiplier: number;
  maxNFTs: number;
  nftPrice: number;
  nftsSold: number;
  isActive: boolean;
  startDate: string;
  endDate: string;
  createdAt: string;
}

/**
 * Application Service para Campaign
 * Orquesta casos de uso sin lógica de dominio
 * Independiente de infraestructura (usa interfaces/puertos)
 */
export class CampaignApplicationService {
  constructor(
    private campaignRepository: CampaignRepository,
    private paymentService: PaymentService,
    private notificationService: NotificationService,
    private campaignDomainService: CampaignDomainService = new CampaignDomainService()
  ) {}

  /**
   * Caso de uso: Crear campaña
   */
  async createCampaign(command: CreateCampaignCommand): Promise<{ success: boolean; campaignId?: string; error?: string }> {
    try {
      // Validaciones de aplicación
      const dateRange = DateRange.create(command.startDate, command.endDate);
      const multiplier = MultiplierValue.create(command.boostMultiplier);

      // Validaciones de negocio usando domain service
      const riskAssessment = this.campaignDomainService.evaluateCampaignRisk(
        Campaign.create({
          songId: command.songId,
          artistId: command.artistId,
          name: command.name,
          description: command.description,
          boostMultiplier: multiplier.raw,
          maxNFTs: command.maxNFTs,
          nftPrice: command.nftPrice,
          dateRange
        })
      );

      if (riskAssessment.riskLevel === 'high') {
        return {
          success: false,
          error: `Campaign risk too high: ${riskAssessment.factors.join(', ')}`
        };
      }

      // Crear el agregado
      const aggregate = CampaignAggregate.create({
        songId: command.songId,
        artistId: command.artistId,
        name: command.name,
        description: command.description,
        boostMultiplier: multiplier.raw,
        maxNFTs: command.maxNFTs,
        nftPrice: command.nftPrice,
        dateRange
      });

      // Persistir
      await this.campaignRepository.save(aggregate);

      // Notificación
      await this.notificationService.sendCampaignCreated(aggregate.id, command.artistId);

      return {
        success: true,
        campaignId: aggregate.id
      };

    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error'
      };
    }
  }

  /**
   * Caso de uso: Activar campaña
   */
  async activateCampaign(campaignId: string): Promise<{ success: boolean; error?: string }> {
    try {
      const aggregate = await this.campaignRepository.findById(campaignId);
      if (!aggregate) {
        return { success: false, error: 'Campaign not found' };
      }

      await aggregate.activate();
      await this.campaignRepository.save(aggregate);

      return { success: true };

    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error'
      };
    }
  }

  /**
   * Caso de uso: Comprar NFT
   */
  async purchaseNFT(command: PurchaseNFTCommand): Promise<{ 
    success: boolean; 
    nftId?: string; 
    transactionId?: string; 
    error?: string 
  }> {
    try {
      const aggregate = await this.campaignRepository.findById(command.campaignId);
      if (!aggregate) {
        return { success: false, error: 'Campaign not found' };
      }

      if (!aggregate.canPurchaseNFT()) {
        return { success: false, error: 'Cannot purchase NFT for this campaign' };
      }

      // Procesar pago
      const paymentResult = await this.paymentService.processNFTPurchase(
        command.buyerId,
        aggregate.getCampaign().nftPrice,
        `campaign-${command.campaignId}`
      );

      if (!paymentResult.success) {
        return { success: false, error: 'Payment failed' };
      }

      // Comprar NFT en el agregado
      const nft = await aggregate.purchaseNFT(command.buyerId);

      // Persistir cambios
      await this.campaignRepository.save(aggregate);

      // Notificación
      await this.notificationService.sendNFTPurchased(
        command.buyerId,
        command.campaignId,
        nft.id
      );

      return {
        success: true,
        nftId: nft.id,
        transactionId: paymentResult.transactionId
      };

    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error'
      };
    }
  }

  /**
   * Query: Obtener campaña por ID
   */
  async getCampaignById(campaignId: string): Promise<CampaignDTO | null> {
    const aggregate = await this.campaignRepository.findById(campaignId);
    if (!aggregate) return null;

    return this.mapToDTO(aggregate.getCampaign());
  }

  /**
   * Query: Obtener campañas activas de un artista
   */
  async getActiveCampaignsByArtist(artistId: string): Promise<CampaignDTO[]> {
    const aggregates = await this.campaignRepository.findActiveByArtistId(artistId);
    return aggregates.map(agg => this.mapToDTO(agg.getCampaign()));
  }

  /**
   * Query: Obtener campañas por canción
   */
  async getCampaignsBySong(songId: string): Promise<CampaignDTO[]> {
    const aggregates = await this.campaignRepository.findBySongId(songId);
    return aggregates.map(agg => this.mapToDTO(agg.getCampaign()));
  }

  /**
   * Query: Obtener analytics de campaña
   */
  async getCampaignAnalytics(campaignId: string): Promise<{
    campaign: CampaignDTO;
    stats: any;
    suggestions: Array<{
      type: string;
      suggestion: string;
      impact: string;
    }>;
  } | null> {
    const aggregate = await this.campaignRepository.findById(campaignId);
    if (!aggregate) return null;

    const campaign = aggregate.getCampaign();
    const stats = campaign.getStats();
    
    const suggestions = this.campaignDomainService.suggestCampaignOptimizations(
      campaign,
      stats
    );

    return {
      campaign: this.mapToDTO(campaign),
      stats: stats.toJSON(),
      suggestions
    };
  }

  /**
   * Comando: Optimizar precio dinámico
   */
  async updateDynamicPricing(campaignId: string): Promise<{ success: boolean; newPrice?: number; error?: string }> {
    try {
      const aggregate = await this.campaignRepository.findById(campaignId);
      if (!aggregate) {
        return { success: false, error: 'Campaign not found' };
      }

      const campaign = aggregate.getCampaign();
      const timeRemaining = campaign.dateRange.daysUntilEnd();
      
      const newPrice = this.campaignDomainService.calculateDynamicNFTPrice(
        campaign,
        aggregate.nftsSold,
        timeRemaining
      );

      // Aquí iríamos al agregado para actualizar el precio
      // Por simplicidad, asumimos que esto es válido
      
      return {
        success: true,
        newPrice
      };

    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error'
      };
    }
  }

  private mapToDTO(campaign: Campaign): CampaignDTO {
    return {
      id: campaign.id,
      songId: campaign.songId,
      artistId: campaign.artistId,
      name: campaign.name,
      description: campaign.description,
      boostMultiplier: campaign.boostMultiplier,
      maxNFTs: campaign.maxNFTs,
      nftPrice: campaign.nftPrice,
      nftsSold: campaign.nftsSold,
      isActive: campaign.isActive,
      startDate: campaign.dateRange.start.toISOString(),
      endDate: campaign.dateRange.end.toISOString(),
      createdAt: campaign.createdAt.toISOString()
    };
  }
} 