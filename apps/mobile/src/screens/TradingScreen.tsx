import React, { useState, useEffect } from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  StyleSheet,
  ScrollView,
  Alert,
  Dimensions,
  TextInput,
  Image,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { Ionicons } from '@expo/vector-icons';
import { SafeAreaView } from 'react-native-safe-area-context';

const { width, height } = Dimensions.get('window');

interface TradingScreenProps {
  navigation: any;
  route: {
    params: {
      song?: any;
      user: any;
    };
  };
}

export default function TradingScreen({ navigation, route }: TradingScreenProps) {
  const { song, user } = route.params;
  
  // Estado del trading
  const [selectedTab, setSelectedTab] = useState<'market' | 'portfolio' | 'history'>('market');
  const [buyAmount, setBuyAmount] = useState('');
  const [sellAmount, setSellAmount] = useState('');
  const [selectedSong, setSelectedSong] = useState(song);
  
  // Datos mock del mercado
  const [marketData, setMarketData] = useState([
    {
      id: '1',
      title: 'Blinding Lights',
      artist: 'The Weeknd',
      currentPrice: 0.085,
      priceChange: '+15.2%',
      volume: '2.4K ETH',
      totalShares: 1000,
      availableShares: 150,
      isPositive: true,
      imageUrl: 'https://via.placeholder.com/60',
    },
    {
      id: '2',
      title: 'Dance Monkey',
      artist: 'Tones and I',
      currentPrice: 0.042,
      priceChange: '-3.1%',
      volume: '1.8K ETH',
      totalShares: 1000,
      availableShares: 320,
      isPositive: false,
      imageUrl: 'https://via.placeholder.com/60',
    },
    {
      id: '3',
      title: 'Midnight Vibes',
      artist: 'Luna Echo',
      currentPrice: 0.067,
      priceChange: '+8.7%',
      volume: '950 ETH',
      totalShares: 1000,
      availableShares: 200,
      isPositive: true,
      imageUrl: 'https://via.placeholder.com/60',
    },
  ]);

  const [portfolioData, setPortfolioData] = useState([
    {
      id: '1',
      title: 'Blinding Lights',
      artist: 'The Weeknd',
      shares: 50,
      averagePrice: 0.075,
      currentPrice: 0.085,
      totalValue: 4.25,
      profitLoss: 0.5,
      profitLossPercentage: 13.3,
      imageUrl: 'https://via.placeholder.com/60',
    },
    {
      id: '2',
      title: 'Midnight Vibes',
      artist: 'Luna Echo',
      shares: 100,
      averagePrice: 0.060,
      currentPrice: 0.067,
      totalValue: 6.7,
      profitLoss: 0.7,
      profitLossPercentage: 11.7,
      imageUrl: 'https://via.placeholder.com/60',
    },
  ]);

  const [portfolioStats, setPortfolioStats] = useState({
    totalValue: 10.95,
    totalProfitLoss: 1.2,
    totalProfitLossPercentage: 12.3,
    totalShares: 150,
  });

  const handleBuyShares = () => {
    if (!buyAmount || parseFloat(buyAmount) <= 0) {
      Alert.alert('Error', 'Por favor ingresa una cantidad válida');
      return;
    }

    const amount = parseFloat(buyAmount);
    const totalCost = amount * selectedSong.currentPrice;

    Alert.alert(
      '🔗 Comprar Participaciones',
      `¿Comprar ${amount} participaciones de "${selectedSong.title}" por ${totalCost.toFixed(3)} ETH?`,
      [
        { text: 'Cancelar', style: 'cancel' },
        { 
          text: 'Comprar', 
          onPress: () => {
            // Aquí conectarías con el backend
            Alert.alert('✅ Éxito', 'Participaciones compradas exitosamente');
            setBuyAmount('');
          }
        },
      ]
    );
  };

  const handleSellShares = () => {
    if (!sellAmount || parseFloat(sellAmount) <= 0) {
      Alert.alert('Error', 'Por favor ingresa una cantidad válida');
      return;
    }

    const amount = parseFloat(sellAmount);
    const totalValue = amount * selectedSong.currentPrice;

    Alert.alert(
      '💰 Vender Participaciones',
      `¿Vender ${amount} participaciones de "${selectedSong.title}" por ${totalValue.toFixed(3)} ETH?`,
      [
        { text: 'Cancelar', style: 'cancel' },
        { 
          text: 'Vender', 
          onPress: () => {
            // Aquí conectarías con el backend
            Alert.alert('✅ Éxito', 'Participaciones vendidas exitosamente');
            setSellAmount('');
          }
        },
      ]
    );
  };

  const renderMarketTab = () => (
    <ScrollView style={styles.tabContent}>
      <Text style={styles.sectionTitle}>Mercado de Participaciones</Text>
      
      {marketData.map((item) => (
        <TouchableOpacity
          key={item.id}
          style={styles.marketItem}
          onPress={() => setSelectedSong(item)}
        >
          <Image source={{ uri: item.imageUrl }} style={styles.songImage} />
          <View style={styles.songInfo}>
            <Text style={styles.songTitle}>{item.title}</Text>
            <Text style={styles.artistName}>{item.artist}</Text>
            <View style={styles.marketStats}>
              <Text style={styles.priceText}>{item.currentPrice} ETH</Text>
              <Text style={[styles.changeText, { color: item.isPositive ? '#4ECDC4' : '#FF6B6B' }]}>
                {item.priceChange}
              </Text>
            </View>
          </View>
          <View style={styles.marketActions}>
            <Text style={styles.volumeText}>{item.volume}</Text>
            <Text style={styles.sharesText}>{item.availableShares} disponibles</Text>
          </View>
        </TouchableOpacity>
      ))}
    </ScrollView>
  );

  const renderPortfolioTab = () => (
    <ScrollView style={styles.tabContent}>
      {/* Portfolio Stats */}
      <View style={styles.portfolioStats}>
        <LinearGradient
          colors={['#6C5CE7', '#A29BFE']}
          style={styles.statsGradient}
        >
          <Text style={styles.statsTitle}>Portfolio Total</Text>
          <Text style={styles.statsValue}>{portfolioStats.totalValue} ETH</Text>
          <Text style={[styles.statsChange, { color: portfolioStats.totalProfitLoss >= 0 ? '#4ECDC4' : '#FF6B6B' }]}>
            {portfolioStats.totalProfitLoss >= 0 ? '+' : ''}{portfolioStats.totalProfitLossPercentage}%
          </Text>
        </LinearGradient>
      </View>

      <Text style={styles.sectionTitle}>Mis Participaciones</Text>
      
      {portfolioData.map((item) => (
        <TouchableOpacity
          key={item.id}
          style={styles.portfolioItem}
          onPress={() => setSelectedSong(item)}
        >
          <Image source={{ uri: item.imageUrl }} style={styles.songImage} />
          <View style={styles.songInfo}>
            <Text style={styles.songTitle}>{item.title}</Text>
            <Text style={styles.artistName}>{item.artist}</Text>
            <Text style={styles.sharesText}>{item.shares} participaciones</Text>
          </View>
          <View style={styles.portfolioActions}>
            <Text style={styles.valueText}>{item.totalValue} ETH</Text>
            <Text style={[styles.profitText, { color: item.profitLoss >= 0 ? '#4ECDC4' : '#FF6B6B' }]}>
              {item.profitLoss >= 0 ? '+' : ''}{item.profitLossPercentage}%
            </Text>
          </View>
        </TouchableOpacity>
      ))}
    </ScrollView>
  );

  const renderHistoryTab = () => (
    <ScrollView style={styles.tabContent}>
      <Text style={styles.sectionTitle}>Historial de Transacciones</Text>
      
      {/* Mock transaction history */}
      <View style={styles.historyItem}>
        <View style={styles.historyInfo}>
          <Text style={styles.historyTitle}>Compra - Blinding Lights</Text>
          <Text style={styles.historyDate}>Hace 2 horas</Text>
        </View>
        <View style={styles.historyAmount}>
          <Text style={styles.historyShares}>+25 participaciones</Text>
          <Text style={styles.historyPrice}>1.875 ETH</Text>
        </View>
      </View>

      <View style={styles.historyItem}>
        <View style={styles.historyInfo}>
          <Text style={styles.historyTitle}>Venta - Dance Monkey</Text>
          <Text style={styles.historyDate}>Hace 1 día</Text>
        </View>
        <View style={styles.historyAmount}>
          <Text style={styles.historyShares}>-10 participaciones</Text>
          <Text style={styles.historyPrice}>0.420 ETH</Text>
        </View>
      </View>
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
          <Text style={styles.headerTitle}>Trading Fraccional</Text>
          <TouchableOpacity>
            <Ionicons name="notifications-outline" size={24} color="#FFFFFF" />
          </TouchableOpacity>
        </View>

        {/* Tabs */}
        <View style={styles.tabs}>
          <TouchableOpacity
            style={[styles.tab, selectedTab === 'market' && styles.activeTab]}
            onPress={() => setSelectedTab('market')}
          >
            <Text style={[styles.tabText, selectedTab === 'market' && styles.activeTabText]}>
              Mercado
            </Text>
          </TouchableOpacity>
          <TouchableOpacity
            style={[styles.tab, selectedTab === 'portfolio' && styles.activeTab]}
            onPress={() => setSelectedTab('portfolio')}
          >
            <Text style={[styles.tabText, selectedTab === 'portfolio' && styles.activeTabText]}>
              Portfolio
            </Text>
          </TouchableOpacity>
          <TouchableOpacity
            style={[styles.tab, selectedTab === 'history' && styles.activeTab]}
            onPress={() => setSelectedTab('history')}
          >
            <Text style={[styles.tabText, selectedTab === 'history' && styles.activeTabText]}>
              Historial
            </Text>
          </TouchableOpacity>
        </View>

        {/* Tab Content */}
        {selectedTab === 'market' && renderMarketTab()}
        {selectedTab === 'portfolio' && renderPortfolioTab()}
        {selectedTab === 'history' && renderHistoryTab()}

        {/* Trading Panel */}
        {selectedSong && (
          <View style={styles.tradingPanel}>
            <LinearGradient
              colors={['#2D2D3A', '#3A3A4A']}
              style={styles.panelGradient}
            >
              <Text style={styles.panelTitle}>Trading: {selectedSong.title}</Text>
              
              <View style={styles.tradingInputs}>
                <View style={styles.inputContainer}>
                  <Text style={styles.inputLabel}>Comprar (ETH)</Text>
                  <TextInput
                    style={styles.input}
                    value={buyAmount}
                    onChangeText={setBuyAmount}
                    placeholder="0.0"
                    placeholderTextColor="#94A3B8"
                    keyboardType="numeric"
                  />
                  <TouchableOpacity style={styles.buyButton} onPress={handleBuyShares}>
                    <Text style={styles.buttonText}>Comprar</Text>
                  </TouchableOpacity>
                </View>

                <View style={styles.inputContainer}>
                  <Text style={styles.inputLabel}>Vender (ETH)</Text>
                  <TextInput
                    style={styles.input}
                    value={sellAmount}
                    onChangeText={setSellAmount}
                    placeholder="0.0"
                    placeholderTextColor="#94A3B8"
                    keyboardType="numeric"
                  />
                  <TouchableOpacity style={styles.sellButton} onPress={handleSellShares}>
                    <Text style={styles.buttonText}>Vender</Text>
                  </TouchableOpacity>
                </View>
              </View>

              <View style={styles.priceInfo}>
                <Text style={styles.priceLabel}>Precio actual:</Text>
                <Text style={styles.priceValue}>{selectedSong.currentPrice} ETH</Text>
              </View>
            </LinearGradient>
          </View>
        )}
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
  sectionTitle: {
    color: '#FFFFFF',
    fontSize: 20,
    fontWeight: 'bold',
    marginBottom: 20,
  },
  marketItem: {
    flexDirection: 'row',
    backgroundColor: '#2D2D3A',
    borderRadius: 12,
    padding: 15,
    marginBottom: 10,
    alignItems: 'center',
  },
  songImage: {
    width: 50,
    height: 50,
    borderRadius: 8,
    marginRight: 15,
  },
  songInfo: {
    flex: 1,
  },
  songTitle: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: '600',
    marginBottom: 4,
  },
  artistName: {
    color: '#94A3B8',
    fontSize: 14,
    marginBottom: 4,
  },
  marketStats: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  priceText: {
    color: '#FFFFFF',
    fontSize: 14,
    fontWeight: '600',
    marginRight: 10,
  },
  changeText: {
    fontSize: 12,
    fontWeight: '500',
  },
  marketActions: {
    alignItems: 'flex-end',
  },
  volumeText: {
    color: '#94A3B8',
    fontSize: 12,
    marginBottom: 4,
  },
  sharesText: {
    color: '#94A3B8',
    fontSize: 12,
  },
  portfolioStats: {
    marginBottom: 20,
  },
  statsGradient: {
    borderRadius: 15,
    padding: 20,
    alignItems: 'center',
  },
  statsTitle: {
    color: '#FFFFFF',
    fontSize: 16,
    marginBottom: 8,
  },
  statsValue: {
    color: '#FFFFFF',
    fontSize: 32,
    fontWeight: 'bold',
    marginBottom: 4,
  },
  statsChange: {
    fontSize: 18,
    fontWeight: '600',
  },
  portfolioItem: {
    flexDirection: 'row',
    backgroundColor: '#2D2D3A',
    borderRadius: 12,
    padding: 15,
    marginBottom: 10,
    alignItems: 'center',
  },
  portfolioActions: {
    alignItems: 'flex-end',
  },
  valueText: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: '600',
    marginBottom: 4,
  },
  profitText: {
    fontSize: 14,
    fontWeight: '500',
  },
  historyItem: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    backgroundColor: '#2D2D3A',
    borderRadius: 12,
    padding: 15,
    marginBottom: 10,
  },
  historyInfo: {
    flex: 1,
  },
  historyTitle: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: '600',
    marginBottom: 4,
  },
  historyDate: {
    color: '#94A3B8',
    fontSize: 12,
  },
  historyAmount: {
    alignItems: 'flex-end',
  },
  historyShares: {
    color: '#FFFFFF',
    fontSize: 14,
    fontWeight: '600',
    marginBottom: 4,
  },
  historyPrice: {
    color: '#94A3B8',
    fontSize: 12,
  },
  tradingPanel: {
    padding: 20,
  },
  panelGradient: {
    borderRadius: 15,
    padding: 20,
  },
  panelTitle: {
    color: '#FFFFFF',
    fontSize: 18,
    fontWeight: 'bold',
    marginBottom: 20,
    textAlign: 'center',
  },
  tradingInputs: {
    marginBottom: 20,
  },
  inputContainer: {
    marginBottom: 15,
  },
  inputLabel: {
    color: '#94A3B8',
    fontSize: 14,
    marginBottom: 8,
  },
  input: {
    backgroundColor: '#1A1A2E',
    borderRadius: 8,
    padding: 12,
    color: '#FFFFFF',
    fontSize: 16,
    marginBottom: 10,
  },
  buyButton: {
    backgroundColor: '#4ECDC4',
    borderRadius: 8,
    padding: 12,
    alignItems: 'center',
  },
  sellButton: {
    backgroundColor: '#FF6B6B',
    borderRadius: 8,
    padding: 12,
    alignItems: 'center',
  },
  buttonText: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: '600',
  },
  priceInfo: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  priceLabel: {
    color: '#94A3B8',
    fontSize: 14,
  },
  priceValue: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: 'bold',
  },
}); 