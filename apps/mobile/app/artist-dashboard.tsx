import React, { useState } from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  ScrollView,
  Alert,
} from 'react-native';

export default function ArtistDashboardScreen({ navigation, route }: any) {
  const { user, token } = route.params || { user: { username: 'Artist Demo' }, token: 'demo' };
  const [totalEarnings, setTotalEarnings] = useState(1247.50);
  const [activeListeners, setActiveListeners] = useState(834);

  const uploadMusic = () => {
    Alert.alert(
      '🎵 Upload Music',
      'Choose your track to upload to VibeStream',
      [
        { text: 'Cancel', style: 'cancel' },
        { 
          text: 'Upload', 
          onPress: () => Alert.alert('🎉 Success!', 'Your track has been uploaded and is now live on VibeStream!') 
        },
      ]
    );
  };

  const createCampaign = (type: string) => {
    Alert.alert(
      '💎 Create NFT Campaign',
      `Launch ${type} campaign with multiplier rewards?`,
      [
        { text: 'Cancel', style: 'cancel' },
        { 
          text: 'Create', 
          onPress: () => Alert.alert('🚀 Campaign Live!', `${type} campaign is now active. Fans can purchase NFTs for boosted rewards!`) 
        },
      ]
    );
  };

  const sellFractionalShares = (song: string) => {
    Alert.alert(
      '🔗 Fractional Ownership',
      `Sell 10% ownership of "${song}" as ERC-1155 tokens?`,
      [
        { text: 'Cancel', style: 'cancel' },
        { 
          text: 'List', 
          onPress: () => Alert.alert('💰 Listed!', 'Fractional shares are now available for purchase by fans.') 
        },
      ]
    );
  };

  return (
    <ScrollView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.title}>Artist Dashboard</Text>
        <Text style={styles.subtitle}>Create • Earn • Connect</Text>
      </View>

      {/* Quick Stats */}
      <View style={styles.statsContainer}>
        <View style={styles.statCard}>
          <Text style={styles.statValue}>${totalEarnings.toFixed(2)}</Text>
          <Text style={styles.statLabel}>Total Earnings</Text>
        </View>
        <View style={styles.statCard}>
          <Text style={styles.statValue}>{activeListeners}</Text>
          <Text style={styles.statLabel}>Active Listeners</Text>
        </View>
        <View style={styles.statCard}>
          <Text style={styles.statValue}>23</Text>
          <Text style={styles.statLabel}>NFT Sales</Text>
        </View>
      </View>

      {/* Quick Actions */}
      <View style={styles.section}>
        <Text style={styles.sectionTitle}>🚀 Quick Actions</Text>
        
        <TouchableOpacity style={styles.actionButton} onPress={uploadMusic}>
          <Text style={styles.actionEmoji}>🎵</Text>
          <View style={styles.actionInfo}>
            <Text style={styles.actionTitle}>Upload New Track</Text>
            <Text style={styles.actionDesc}>Add music to your catalog</Text>
          </View>
          <Text style={styles.actionArrow}>→</Text>
        </TouchableOpacity>

        <TouchableOpacity 
          style={styles.actionButton} 
          onPress={() => createCampaign('Summer Vibes')}
        >
          <Text style={styles.actionEmoji}>💎</Text>
          <View style={styles.actionInfo}>
            <Text style={styles.actionTitle}>Create NFT Campaign</Text>
            <Text style={styles.actionDesc}>Launch promotional campaigns</Text>
          </View>
          <Text style={styles.actionArrow}>→</Text>
        </TouchableOpacity>
      </View>

      {/* Your Tracks */}
      <View style={styles.section}>
        <Text style={styles.sectionTitle}>🎧 Your Music</Text>
        
        <View style={styles.trackCard}>
          <Text style={styles.trackEmoji}>🌟</Text>
          <View style={styles.trackInfo}>
            <Text style={styles.trackName}>Neon Dreams</Text>
            <Text style={styles.trackStats}>2.3k plays • $124.50 earned</Text>
          </View>
          <TouchableOpacity 
            style={styles.fractionalButton}
            onPress={() => sellFractionalShares('Neon Dreams')}
          >
            <Text style={styles.fractionalText}>🔗 Sell Shares</Text>
          </TouchableOpacity>
        </View>

        <View style={styles.trackCard}>
          <Text style={styles.trackEmoji}>🚀</Text>
          <View style={styles.trackInfo}>
            <Text style={styles.trackName}>Cosmic Journey</Text>
            <Text style={styles.trackStats}>1.8k plays • $89.20 earned</Text>
          </View>
          <TouchableOpacity 
            style={styles.fractionalButton}
            onPress={() => sellFractionalShares('Cosmic Journey')}
          >
            <Text style={styles.fractionalText}>🔗 Sell Shares</Text>
          </TouchableOpacity>
        </View>

        <View style={styles.trackCard}>
          <Text style={styles.trackEmoji}>💖</Text>
          <View style={styles.trackInfo}>
            <Text style={styles.trackName}>Digital Love</Text>
            <Text style={styles.trackStats}>3.1k plays • $187.40 earned</Text>
          </View>
          <TouchableOpacity 
            style={styles.fractionalButton}
            onPress={() => sellFractionalShares('Digital Love')}
          >
            <Text style={styles.fractionalText}>🔗 Sell Shares</Text>
          </TouchableOpacity>
        </View>
      </View>

      {/* Active Campaigns */}
      <View style={styles.section}>
        <Text style={styles.sectionTitle}>💎 Active Campaigns</Text>
        
        <View style={styles.campaignCard}>
          <Text style={styles.campaignEmoji}>🏖️</Text>
          <View style={styles.campaignInfo}>
            <Text style={styles.campaignName}>Summer Vibes 2025</Text>
            <Text style={styles.campaignDesc}>47 NFTs sold • 2.5x multiplier</Text>
          </View>
          <View style={styles.campaignEarnings}>
            <Text style={styles.campaignAmount}>$234.50</Text>
          </View>
        </View>

        <View style={styles.campaignCard}>
          <Text style={styles.campaignEmoji}>⚡</Text>
          <View style={styles.campaignInfo}>
            <Text style={styles.campaignName}>Electronic Fusion</Text>
            <Text style={styles.campaignDesc}>23 NFTs sold • 1.8x multiplier</Text>
          </View>
          <View style={styles.campaignEarnings}>
            <Text style={styles.campaignAmount}>$145.30</Text>
          </View>
        </View>

        <TouchableOpacity 
          style={styles.newCampaignButton}
          onPress={() => createCampaign('New Campaign')}
        >
          <Text style={styles.newCampaignText}>+ Create New Campaign</Text>
        </TouchableOpacity>
      </View>

      {/* Analytics */}
      <View style={styles.section}>
        <Text style={styles.sectionTitle}>📊 Analytics</Text>
        
        <View style={styles.analyticsGrid}>
          <View style={styles.analyticsCard}>
            <Text style={styles.analyticsValue}>7.2k</Text>
            <Text style={styles.analyticsLabel}>Total Plays</Text>
          </View>
          <View style={styles.analyticsCard}>
            <Text style={styles.analyticsValue}>156</Text>
            <Text style={styles.analyticsLabel}>New Fans</Text>
          </View>
          <View style={styles.analyticsCard}>
            <Text style={styles.analyticsValue}>85%</Text>
            <Text style={styles.analyticsLabel}>ZK Verified</Text>
          </View>
          <View style={styles.analyticsCard}>
            <Text style={styles.analyticsValue}>4.8⭐</Text>
            <Text style={styles.analyticsLabel}>Avg Rating</Text>
          </View>
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
    color: '#FF6B35',
  },
  statsContainer: {
    flexDirection: 'row',
    paddingHorizontal: 24,
    marginBottom: 30,
    gap: 12,
  },
  statCard: {
    flex: 1,
    backgroundColor: '#2d2d54',
    padding: 16,
    borderRadius: 12,
    alignItems: 'center',
    borderWidth: 1,
    borderColor: '#FF6B35',
  },
  statValue: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#4CAF50',
    marginBottom: 4,
  },
  statLabel: {
    fontSize: 12,
    color: '#a8a8a8',
    textAlign: 'center',
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
  actionButton: {
    backgroundColor: '#2d2d54',
    flexDirection: 'row',
    alignItems: 'center',
    padding: 16,
    borderRadius: 12,
    marginBottom: 12,
    borderWidth: 1,
    borderColor: '#4CAF50',
  },
  actionEmoji: {
    fontSize: 24,
    marginRight: 16,
  },
  actionInfo: {
    flex: 1,
  },
  actionTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#ffffff',
    marginBottom: 4,
  },
  actionDesc: {
    fontSize: 14,
    color: '#a8a8a8',
  },
  actionArrow: {
    fontSize: 18,
    color: '#4CAF50',
    fontWeight: 'bold',
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
  trackStats: {
    fontSize: 14,
    color: '#a8a8a8',
  },
  fractionalButton: {
    backgroundColor: '#4CAF50',
    paddingHorizontal: 12,
    paddingVertical: 8,
    borderRadius: 8,
  },
  fractionalText: {
    fontSize: 12,
    fontWeight: '600',
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
  campaignEarnings: {
    alignItems: 'flex-end',
  },
  campaignAmount: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#4CAF50',
  },
  newCampaignButton: {
    backgroundColor: 'transparent',
    borderWidth: 2,
    borderColor: '#FF6B35',
    borderStyle: 'dashed',
    padding: 16,
    borderRadius: 12,
    alignItems: 'center',
  },
  newCampaignText: {
    fontSize: 16,
    fontWeight: '600',
    color: '#FF6B35',
  },
  analyticsGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    gap: 12,
  },
  analyticsCard: {
    backgroundColor: '#2d2d54',
    padding: 16,
    borderRadius: 12,
    alignItems: 'center',
    width: '47%',
  },
  analyticsValue: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#4CAF50',
    marginBottom: 4,
  },
  analyticsLabel: {
    fontSize: 12,
    color: '#a8a8a8',
    textAlign: 'center',
  },
}); 