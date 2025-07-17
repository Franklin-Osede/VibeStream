import React, { useState, useEffect } from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  StyleSheet,
  ScrollView,
  Alert,
  Dimensions,
  Image,
  TextInput,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { Ionicons } from '@expo/vector-icons';
import { SafeAreaView } from 'react-native-safe-area-context';

const { width, height } = Dimensions.get('window');

interface NFTMarketplaceScreenProps {
  navigation: any;
  route: {
    params: {
      song?: any;
      user: any;
    };
  };
}

export default function NFTMarketplaceScreen({ navigation, route }: NFTMarketplaceScreenProps) {
  const { song, user } = route.params;
  
  // Estado del marketplace
  const [selectedTab, setSelectedTab] = useState<'collections' | 'marketplace' | 'my-nfts'>('collections');
  const [selectedCollection, setSelectedCollection] = useState(null);
  const [searchQuery, setSearchQuery] = useState('');
  
  // Datos mock de colecciones
  const [collections, setCollections] = useState([
    {
      id: '1',
      name: 'Genesis Collection',
      artist: 'Luna Echo',
      description: 'La primera colección NFT de Luna Echo con regalías automáticas',
      totalSupply: 100,
      minted: 75,
      floorPrice: 0.5,
      totalVolume: '125 ETH',
      imageUrl: 'https://via.placeholder.com/120',
      bannerUrl: 'https://via.placeholder.com/300x150',
      isVerified: true,
    },
    {
      id: '2',
      name: 'Neon Dreams',
      artist: 'Cyber Collective',
      description: 'Colección futurista con efectos visuales únicos',
      totalSupply: 50,
      minted: 30,
      floorPrice: 1.2,
      totalVolume: '85 ETH',
      imageUrl: 'https://via.placeholder.com/120',
      bannerUrl: 'https://via.placeholder.com/300x150',
      isVerified: true,
    },
    {
      id: '3',
      name: 'Summer Vibes 2025',
      artist: 'Beach Waves',
      description: 'NFTs temáticos de verano con beneficios exclusivos',
      totalSupply: 200,
      minted: 150,
      floorPrice: 0.3,
      totalVolume: '45 ETH',
      imageUrl: 'https://via.placeholder.com/120',
      bannerUrl: 'https://via.placeholder.com/300x150',
      isVerified: false,
    },
  ]);

  // Datos mock de NFTs en venta
  const [marketplaceNFTs, setMarketplaceNFTs] = useState([
    {
      id: '1',
      name: 'Genesis #001',
      collection: 'Genesis Collection',
      artist: 'Luna Echo',
      price: 0.8,
      rarity: 'legendary',
      attributes: [
        { trait: 'Background', value: 'Neon Purple', rarity: 'rare' },
        { trait: 'Instrument', value: 'Synthesizer', rarity: 'common' },
        { trait: 'Effect', value: 'Holographic', rarity: 'legendary' },
      ],
      imageUrl: 'https://via.placeholder.com/200',
      isForSale: true,
      tokenId: '1',
      contractAddress: '0x1234567890abcdef',
    },
    {
      id: '2',
      name: 'Neon Dreams #015',
      collection: 'Neon Dreams',
      artist: 'Cyber Collective',
      price: 1.5,
      rarity: 'epic',
      attributes: [
        { trait: 'Background', value: 'Cyber Grid', rarity: 'epic' },
        { trait: 'Instrument', value: 'Digital Piano', rarity: 'rare' },
        { trait: 'Effect', value: 'Glitch', rarity: 'epic' },
      ],
      imageUrl: 'https://via.placeholder.com/200',
      isForSale: true,
      tokenId: '15',
      contractAddress: '0xabcdef1234567890',
    },
  ]);

  // Datos mock de NFTs del usuario
  const [myNFTs, setMyNFTs] = useState([
    {
      id: '1',
      name: 'Genesis #023',
      collection: 'Genesis Collection',
      artist: 'Luna Echo',
      rarity: 'rare',
      attributes: [
        { trait: 'Background', value: 'Ocean Blue', rarity: 'rare' },
        { trait: 'Instrument', value: 'Acoustic Guitar', rarity: 'common' },
        { trait: 'Effect', value: 'Wave', rarity: 'rare' },
      ],
      imageUrl: 'https://via.placeholder.com/200',
      isForSale: false,
      tokenId: '23',
      contractAddress: '0x1234567890abcdef',
      royaltiesEarned: 0.15,
    },
  ]);

  const handleBuyNFT = (nft: any) => {
    Alert.alert(
      '🎨 Comprar NFT',
      `¿Comprar "${nft.name}" por ${nft.price} ETH?`,
      [
        { text: 'Cancelar', style: 'cancel' },
        { 
          text: 'Comprar', 
          onPress: () => {
            // Aquí conectarías con el backend
            Alert.alert('✅ Éxito', 'NFT comprado exitosamente');
          }
        },
      ]
    );
  };

  const handleSellNFT = (nft: any) => {
    Alert.alert(
      '💰 Vender NFT',
      `¿Poner en venta "${nft.name}"?`,
      [
        { text: 'Cancelar', style: 'cancel' },
        { 
          text: 'Vender', 
          onPress: () => {
            // Navegar a pantalla de configuración de venta
            navigation.navigate('NFTSellScreen', { nft });
          }
        },
      ]
    );
  };

  const getRarityColor = (rarity: string) => {
    switch (rarity) {
      case 'legendary': return '#FFD93D';
      case 'epic': return '#A29BFE';
      case 'rare': return '#4ECDC4';
      case 'common': return '#94A3B8';
      default: return '#94A3B8';
    }
  };

  const renderCollectionsTab = () => (
    <ScrollView style={styles.tabContent}>
      <Text style={styles.sectionTitle}>Colecciones Destacadas</Text>
      
      {collections.map((collection) => (
        <TouchableOpacity
          key={collection.id}
          style={styles.collectionCard}
          onPress={() => setSelectedCollection(collection)}
        >
          <Image source={{ uri: collection.bannerUrl }} style={styles.collectionBanner} />
          <View style={styles.collectionInfo}>
            <View style={styles.collectionHeader}>
              <Image source={{ uri: collection.imageUrl }} style={styles.collectionImage} />
              <View style={styles.collectionDetails}>
                <View style={styles.collectionTitleRow}>
                  <Text style={styles.collectionName}>{collection.name}</Text>
                  {collection.isVerified && (
                    <Ionicons name="checkmark-circle" size={16} color="#4ECDC4" />
                  )}
                </View>
                <Text style={styles.collectionArtist}>por {collection.artist}</Text>
              </View>
            </View>
            <Text style={styles.collectionDescription}>{collection.description}</Text>
            <View style={styles.collectionStats}>
              <View style={styles.statItem}>
                <Text style={styles.statLabel}>Floor Price</Text>
                <Text style={styles.statValue}>{collection.floorPrice} ETH</Text>
              </View>
              <View style={styles.statItem}>
                <Text style={styles.statLabel}>Supply</Text>
                <Text style={styles.statValue}>{collection.minted}/{collection.totalSupply}</Text>
              </View>
              <View style={styles.statItem}>
                <Text style={styles.statLabel}>Volume</Text>
                <Text style={styles.statValue}>{collection.totalVolume}</Text>
              </View>
            </View>
          </View>
        </TouchableOpacity>
      ))}
    </ScrollView>
  );

  const renderMarketplaceTab = () => (
    <ScrollView style={styles.tabContent}>
      {/* Search Bar */}
      <View style={styles.searchContainer}>
        <Ionicons name="search" size={20} color="#94A3B8" style={styles.searchIcon} />
        <TextInput
          style={styles.searchInput}
          placeholder="Buscar NFTs..."
          placeholderTextColor="#94A3B8"
          value={searchQuery}
          onChangeText={setSearchQuery}
        />
      </View>

      <Text style={styles.sectionTitle}>NFTs en Venta</Text>
      
      <View style={styles.nftGrid}>
        {marketplaceNFTs.map((nft) => (
          <TouchableOpacity
            key={nft.id}
            style={styles.nftCard}
            onPress={() => navigation.navigate('NFTDetailScreen', { nft })}
          >
            <Image source={{ uri: nft.imageUrl }} style={styles.nftImage} />
            <View style={styles.nftInfo}>
              <Text style={styles.nftName}>{nft.name}</Text>
              <Text style={styles.nftCollection}>{nft.collection}</Text>
              <View style={styles.nftRarity}>
                <View style={[styles.rarityBadge, { backgroundColor: getRarityColor(nft.rarity) }]}>
                  <Text style={styles.rarityText}>{nft.rarity.toUpperCase()}</Text>
                </View>
              </View>
              <View style={styles.nftPrice}>
                <Text style={styles.priceLabel}>Precio</Text>
                <Text style={styles.priceValue}>{nft.price} ETH</Text>
              </View>
              <TouchableOpacity
                style={styles.buyButton}
                onPress={() => handleBuyNFT(nft)}
              >
                <Text style={styles.buyButtonText}>Comprar</Text>
              </TouchableOpacity>
            </View>
          </TouchableOpacity>
        ))}
      </View>
    </ScrollView>
  );

  const renderMyNFTsTab = () => (
    <ScrollView style={styles.tabContent}>
      <Text style={styles.sectionTitle}>Mis NFTs</Text>
      
      {myNFTs.map((nft) => (
        <TouchableOpacity
          key={nft.id}
          style={styles.myNftCard}
          onPress={() => navigation.navigate('NFTDetailScreen', { nft })}
        >
          <Image source={{ uri: nft.imageUrl }} style={styles.myNftImage} />
          <View style={styles.myNftInfo}>
            <Text style={styles.myNftName}>{nft.name}</Text>
            <Text style={styles.myNftCollection}>{nft.collection}</Text>
            <View style={styles.myNftRarity}>
              <View style={[styles.rarityBadge, { backgroundColor: getRarityColor(nft.rarity) }]}>
                <Text style={styles.rarityText}>{nft.rarity.toUpperCase()}</Text>
              </View>
            </View>
            <View style={styles.royaltiesInfo}>
              <Text style={styles.royaltiesLabel}>Regalías Ganadas</Text>
              <Text style={styles.royaltiesValue}>{nft.royaltiesEarned} ETH</Text>
            </View>
            <TouchableOpacity
              style={styles.sellButton}
              onPress={() => handleSellNFT(nft)}
            >
              <Text style={styles.sellButtonText}>Vender</Text>
            </TouchableOpacity>
          </View>
        </TouchableOpacity>
      ))}
    </ScrollView>
  );

  return (
    <SafeAreaView style={styles.container}>
      <LinearGradient
        colors={['#0F0F1E', '#1A1A2E', '#16213E']}
        style={styles.gradient}
      >
        {/* Header */}
        <View style={styles.header}>
          <TouchableOpacity onPress={() => navigation.goBack()}>
            <Ionicons name="arrow-back" size={24} color="#FFFFFF" />
          </TouchableOpacity>
          <Text style={styles.headerTitle}>NFT Marketplace</Text>
          <TouchableOpacity>
            <Ionicons name="wallet-outline" size={24} color="#FFFFFF" />
          </TouchableOpacity>
        </View>

        {/* Tabs */}
        <View style={styles.tabs}>
          <TouchableOpacity
            style={[styles.tab, selectedTab === 'collections' && styles.activeTab]}
            onPress={() => setSelectedTab('collections')}
          >
            <Text style={[styles.tabText, selectedTab === 'collections' && styles.activeTabText]}>
              Colecciones
            </Text>
          </TouchableOpacity>
          <TouchableOpacity
            style={[styles.tab, selectedTab === 'marketplace' && styles.activeTab]}
            onPress={() => setSelectedTab('marketplace')}
          >
            <Text style={[styles.tabText, selectedTab === 'marketplace' && styles.activeTabText]}>
              Marketplace
            </Text>
          </TouchableOpacity>
          <TouchableOpacity
            style={[styles.tab, selectedTab === 'my-nfts' && styles.activeTab]}
            onPress={() => setSelectedTab('my-nfts')}
          >
            <Text style={[styles.tabText, selectedTab === 'my-nfts' && styles.activeTabText]}>
              Mis NFTs
            </Text>
          </TouchableOpacity>
        </View>

        {/* Tab Content */}
        {selectedTab === 'collections' && renderCollectionsTab()}
        {selectedTab === 'marketplace' && renderMarketplaceTab()}
        {selectedTab === 'my-nfts' && renderMyNFTsTab()}
      </LinearGradient>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  gradient: {
    flex: 1,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: 20,
    paddingVertical: 15,
  },
  headerTitle: {
    color: '#FFFFFF',
    fontSize: 18,
    fontWeight: 'bold',
  },
  tabs: {
    flexDirection: 'row',
    paddingHorizontal: 20,
    marginBottom: 20,
  },
  tab: {
    flex: 1,
    paddingVertical: 12,
    alignItems: 'center',
    borderBottomWidth: 2,
    borderBottomColor: 'transparent',
  },
  activeTab: {
    borderBottomColor: '#6C5CE7',
  },
  tabText: {
    color: '#94A3B8',
    fontSize: 14,
    fontWeight: '500',
  },
  activeTabText: {
    color: '#6C5CE7',
  },
  tabContent: {
    flex: 1,
    paddingHorizontal: 20,
  },
  searchContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#2D2D3A',
    borderRadius: 12,
    paddingHorizontal: 15,
    marginBottom: 20,
  },
  searchIcon: {
    marginRight: 10,
  },
  searchInput: {
    flex: 1,
    color: '#FFFFFF',
    fontSize: 16,
    paddingVertical: 12,
  },
  sectionTitle: {
    color: '#FFFFFF',
    fontSize: 20,
    fontWeight: 'bold',
    marginBottom: 20,
  },
  collectionCard: {
    backgroundColor: '#2D2D3A',
    borderRadius: 15,
    marginBottom: 20,
    overflow: 'hidden',
  },
  collectionBanner: {
    width: '100%',
    height: 120,
  },
  collectionInfo: {
    padding: 15,
  },
  collectionHeader: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: 10,
  },
  collectionImage: {
    width: 50,
    height: 50,
    borderRadius: 25,
    marginRight: 15,
  },
  collectionDetails: {
    flex: 1,
  },
  collectionTitleRow: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  collectionName: {
    color: '#FFFFFF',
    fontSize: 18,
    fontWeight: 'bold',
    marginRight: 8,
  },
  collectionArtist: {
    color: '#94A3B8',
    fontSize: 14,
  },
  collectionDescription: {
    color: '#E2E8F0',
    fontSize: 14,
    marginBottom: 15,
    lineHeight: 20,
  },
  collectionStats: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
  statItem: {
    alignItems: 'center',
  },
  statLabel: {
    color: '#94A3B8',
    fontSize: 12,
    marginBottom: 4,
  },
  statValue: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: '600',
  },
  nftGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    justifyContent: 'space-between',
  },
  nftCard: {
    width: (width - 60) / 2,
    backgroundColor: '#2D2D3A',
    borderRadius: 12,
    marginBottom: 20,
    overflow: 'hidden',
  },
  nftImage: {
    width: '100%',
    height: 150,
  },
  nftInfo: {
    padding: 12,
  },
  nftName: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: '600',
    marginBottom: 4,
  },
  nftCollection: {
    color: '#94A3B8',
    fontSize: 12,
    marginBottom: 8,
  },
  nftRarity: {
    marginBottom: 8,
  },
  rarityBadge: {
    alignSelf: 'flex-start',
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 6,
  },
  rarityText: {
    color: '#FFFFFF',
    fontSize: 10,
    fontWeight: 'bold',
  },
  nftPrice: {
    marginBottom: 10,
  },
  priceLabel: {
    color: '#94A3B8',
    fontSize: 12,
    marginBottom: 2,
  },
  priceValue: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: 'bold',
  },
  buyButton: {
    backgroundColor: '#6C5CE7',
    borderRadius: 8,
    paddingVertical: 8,
    alignItems: 'center',
  },
  buyButtonText: {
    color: '#FFFFFF',
    fontSize: 14,
    fontWeight: '600',
  },
  myNftCard: {
    flexDirection: 'row',
    backgroundColor: '#2D2D3A',
    borderRadius: 12,
    padding: 15,
    marginBottom: 15,
    alignItems: 'center',
  },
  myNftImage: {
    width: 80,
    height: 80,
    borderRadius: 8,
    marginRight: 15,
  },
  myNftInfo: {
    flex: 1,
  },
  myNftName: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: '600',
    marginBottom: 4,
  },
  myNftCollection: {
    color: '#94A3B8',
    fontSize: 12,
    marginBottom: 8,
  },
  myNftRarity: {
    marginBottom: 8,
  },
  royaltiesInfo: {
    marginBottom: 10,
  },
  royaltiesLabel: {
    color: '#94A3B8',
    fontSize: 12,
    marginBottom: 2,
  },
  royaltiesValue: {
    color: '#4ECDC4',
    fontSize: 14,
    fontWeight: '600',
  },
  sellButton: {
    backgroundColor: '#FF6B6B',
    borderRadius: 8,
    paddingVertical: 6,
    paddingHorizontal: 12,
    alignSelf: 'flex-start',
  },
  sellButtonText: {
    color: '#FFFFFF',
    fontSize: 12,
    fontWeight: '600',
  },
}); 