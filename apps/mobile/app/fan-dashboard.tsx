import React, { useState, useEffect } from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  ScrollView,
  Alert,
} from 'react-native';
// Removed expo-router dependency for React Navigation

export default function FanDashboardScreen({ navigation, route }: any) {
  const { user, token } = route.params || { user: { username: 'Fan Demo' }, token: 'demo' };
  const [earnings, setEarnings] = useState(0);
  const [isListening, setIsListening] = useState(false);
  const [currentSong, setCurrentSong] = useState('');

  useEffect(() => {
    // Simulate earning $VIBERS while listening
    let interval: NodeJS.Timeout;
    if (isListening) {
      interval = setInterval(() => {
        setEarnings(prev => prev + 0.1);
      }, 1000);
    }
    return () => clearInterval(interval);
  }, [isListening]);

  const playTrack = (trackName: string) => {
    setCurrentSong(trackName);
    setIsListening(true);
    Alert.alert(
      '🎵 Now Playing',
      `${trackName}\n\nEarning $VIBERS tokens...`,
      [{ text: 'OK' }]
    );
  };

  const stopListening = () => {
    setIsListening(false);
    setCurrentSong('');
  };

  const buyCampaignNFT = (campaign: string) => {
    Alert.alert(
      '💎 Purchase NFT Campaign',
      `Buy ${campaign} for 2x listening rewards?`,
      [
        { text: 'Cancel', style: 'cancel' },
        { 
          text: 'Buy', 
          onPress: () => Alert.alert('🎉 Success!', 'Campaign NFT purchased! 2x rewards activated.') 
        },
      ]
    );
  };

  return (
    <ScrollView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.title}>Fan Dashboard</Text>
        <Text style={styles.subtitle}>Listen • Earn • Invest</Text>
      </View>

      {/* Earnings Widget */}
      <View style={styles.earningsCard}>
        <Text style={styles.cardTitle}>💰 Your Earnings</Text>
        <Text style={styles.earningsAmount}>{earnings.toFixed(2)} $VIBERS</Text>
        <Text style={styles.earningsSubtext}>
          {isListening ? '🎵 Earning while listening...' : 'Start listening to earn'}
        </Text>
      </View>

      {/* Currently Playing */}
      {isListening && (
        <View style={styles.nowPlayingCard}>
          <Text style={styles.cardTitle}>🎵 Now Playing</Text>
          <Text style={styles.songTitle}>{currentSong}</Text>
          <TouchableOpacity style={styles.stopButton} onPress={stopListening}>
            <Text style={styles.stopButtonText}>⏸️ Stop</Text>
          </TouchableOpacity>
        </View>
      )}

      {/* Featured Tracks */}
      <View style={styles.section}>
        <Text style={styles.sectionTitle}>🔥 Trending Tracks</Text>
        
        <TouchableOpacity 
          style={styles.trackCard}
          onPress={() => playTrack('Neon Dreams - CyberPunk')}
        >
          <Text style={styles.trackEmoji}>🌟</Text>
          <View style={styles.trackInfo}>
            <Text style={styles.trackName}>Neon Dreams</Text>
            <Text style={styles.artistName}>CyberPunk</Text>
          </View>
          <View style={styles.multiplierBadge}>
            <Text style={styles.multiplierText}>2.5x</Text>
          </View>
        </TouchableOpacity>

        <TouchableOpacity 
          style={styles.trackCard}
          onPress={() => playTrack('Solar Wind - Cosmos')}
        >
          <Text style={styles.trackEmoji}>🚀</Text>
          <View style={styles.trackInfo}>
            <Text style={styles.trackName}>Solar Wind</Text>
            <Text style={styles.artistName}>Cosmos</Text>
          </View>
          <View style={styles.multiplierBadge}>
            <Text style={styles.multiplierText}>1.8x</Text>
          </View>
        </TouchableOpacity>

        <TouchableOpacity 
          style={styles.trackCard}
          onPress={() => playTrack('Digital Love - VirtualBeat')}
        >
          <Text style={styles.trackEmoji}>💖</Text>
          <View style={styles.trackInfo}>
            <Text style={styles.trackName}>Digital Love</Text>
            <Text style={styles.artistName}>VirtualBeat</Text>
          </View>
          <View style={styles.multiplierBadge}>
            <Text style={styles.multiplierText}>1.5x</Text>
          </View>
        </TouchableOpacity>
      </View>

      {/* NFT Campaigns */}
      <View style={styles.section}>
        <Text style={styles.sectionTitle}>💎 Active NFT Campaigns</Text>
        
        <TouchableOpacity 
          style={styles.campaignCard}
          onPress={() => buyCampaignNFT('Summer Vibes 2025')}
        >
          <Text style={styles.campaignEmoji}>🏖️</Text>
          <View style={styles.campaignInfo}>
            <Text style={styles.campaignName}>Summer Vibes 2025</Text>
            <Text style={styles.campaignDesc}>2x rewards for beach music</Text>
          </View>
          <Text style={styles.campaignPrice}>5 $VIBERS</Text>
        </TouchableOpacity>

        <TouchableOpacity 
          style={styles.campaignCard}
          onPress={() => buyCampaignNFT('Electronic Fusion')}
        >
          <Text style={styles.campaignEmoji}>⚡</Text>
          <View style={styles.campaignInfo}>
            <Text style={styles.campaignName}>Electronic Fusion</Text>
            <Text style={styles.campaignDesc}>1.5x rewards for EDM tracks</Text>
          </View>
          <Text style={styles.campaignPrice}>3 $VIBERS</Text>
        </TouchableOpacity>
      </View>

      {/* Quick Stats */}
      <View style={styles.statsContainer}>
        <View style={styles.statCard}>
          <Text style={styles.statValue}>47</Text>
          <Text style={styles.statLabel}>Songs Heard</Text>
        </View>
        <View style={styles.statCard}>
          <Text style={styles.statValue}>12</Text>
          <Text style={styles.statLabel}>NFTs Owned</Text>
        </View>
        <View style={styles.statCard}>
          <Text style={styles.statValue}>3.2k</Text>
          <Text style={styles.statLabel}>ZK Proofs</Text>
        </View>
      </View>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#1a1a2e',
  },
  header: {
    alignItems: 'center',
    paddingHorizontal: 24,
    paddingTop: 60,
    paddingBottom: 30,
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#ffffff',
    marginBottom: 8,
  },
  subtitle: {
    fontSize: 16,
    color: '#4CAF50',
  },
  earningsCard: {
    backgroundColor: '#2d2d54',
    marginHorizontal: 24,
    marginBottom: 20,
    padding: 24,
    borderRadius: 16,
    alignItems: 'center',
    borderWidth: 2,
    borderColor: '#4CAF50',
  },
  cardTitle: {
    fontSize: 18,
    fontWeight: '600',
    color: '#ffffff',
    marginBottom: 12,
  },
  earningsAmount: {
    fontSize: 32,
    fontWeight: 'bold',
    color: '#4CAF50',
    marginBottom: 8,
  },
  earningsSubtext: {
    fontSize: 14,
    color: '#a8a8a8',
  },
  nowPlayingCard: {
    backgroundColor: '#FF6B35',
    marginHorizontal: 24,
    marginBottom: 20,
    padding: 20,
    borderRadius: 16,
    alignItems: 'center',
  },
  songTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#ffffff',
    marginBottom: 12,
  },
  stopButton: {
    backgroundColor: '#ffffff',
    paddingHorizontal: 20,
    paddingVertical: 8,
    borderRadius: 20,
  },
  stopButtonText: {
    fontSize: 16,
    fontWeight: '600',
    color: '#FF6B35',
  },
  section: {
    paddingHorizontal: 24,
    marginBottom: 30,
  },
  sectionTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#ffffff',
    marginBottom: 16,
  },
  trackCard: {
    backgroundColor: '#2d2d54',
    flexDirection: 'row',
    alignItems: 'center',
    padding: 16,
    borderRadius: 12,
    marginBottom: 12,
  },
  trackEmoji: {
    fontSize: 24,
    marginRight: 16,
  },
  trackInfo: {
    flex: 1,
  },
  trackName: {
    fontSize: 16,
    fontWeight: '600',
    color: '#ffffff',
    marginBottom: 4,
  },
  artistName: {
    fontSize: 14,
    color: '#a8a8a8',
  },
  multiplierBadge: {
    backgroundColor: '#4CAF50',
    paddingHorizontal: 12,
    paddingVertical: 6,
    borderRadius: 12,
  },
  multiplierText: {
    fontSize: 14,
    fontWeight: 'bold',
    color: '#ffffff',
  },
  campaignCard: {
    backgroundColor: '#2d2d54',
    flexDirection: 'row',
    alignItems: 'center',
    padding: 16,
    borderRadius: 12,
    marginBottom: 12,
    borderWidth: 1,
    borderColor: '#FF6B35',
  },
  campaignEmoji: {
    fontSize: 24,
    marginRight: 16,
  },
  campaignInfo: {
    flex: 1,
  },
  campaignName: {
    fontSize: 16,
    fontWeight: '600',
    color: '#ffffff',
    marginBottom: 4,
  },
  campaignDesc: {
    fontSize: 14,
    color: '#a8a8a8',
  },
  campaignPrice: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#4CAF50',
  },
  statsContainer: {
    flexDirection: 'row',
    paddingHorizontal: 24,
    marginBottom: 40,
    gap: 12,
  },
  statCard: {
    flex: 1,
    backgroundColor: '#2d2d54',
    padding: 16,
    borderRadius: 12,
    alignItems: 'center',
  },
  statValue: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#4CAF50',
    marginBottom: 4,
  },
  statLabel: {
    fontSize: 12,
    color: '#a8a8a8',
    textAlign: 'center',
  },
}); 