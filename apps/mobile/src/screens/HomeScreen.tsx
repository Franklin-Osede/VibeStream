import React, { useState, useEffect } from 'react';
import {
  View,
  Text,
  ScrollView,
  TouchableOpacity,
  StyleSheet,
  Dimensions,
  Image,
  StatusBar,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { Ionicons } from '@expo/vector-icons';
import { SafeAreaView } from 'react-native-safe-area-context';

const { width, height } = Dimensions.get('window');

// Mock data - después conectaremos con el backend
const mockFeedData = [
  {
    id: '1',
    title: 'U Should Know (danse)',
    artist: 'molly mcphaul',
    duration: '3:15',
    plays: '46',
    reposts: 25,
    likes: 32,
    comments: 9,
    imageUrl: 'https://via.placeholder.com/80',
    isVerified: true,
  },
  {
    id: '2',
    title: 'mellodaze - in retrospect',
    artist: 'Arden Records',
    duration: '1:20',
    plays: '6.87K',
    reposts: 48,
    likes: 78,
    comments: 12,
    imageUrl: 'https://via.placeholder.com/80',
    isVerified: true,
  },
  {
    id: '3',
    title: 'hi july - instrumental',
    artist: 'molly mcphaul',
    duration: '1:52',
    plays: '6.88K',
    reposts: 88,
    likes: 154,
    comments: 28,
    imageUrl: 'https://via.placeholder.com/80',
    isVerified: true,
  },
];

const mockVREvents = [
  {
    id: '1',
    title: 'Live VR Concert',
    artist: 'Skrillex',
    date: 'Tonight 8PM',
    attendees: '1.2K',
    maxAttendees: '5K',
    price: '0.1 ETH',
    imageUrl: 'https://via.placeholder.com/120',
  },
  {
    id: '2',
    title: 'Virtual Festival',
    artist: 'Deadmau5',
    date: 'Tomorrow 9PM',
    attendees: '856',
    maxAttendees: '3K',
    price: '0.05 ETH',
    imageUrl: 'https://via.placeholder.com/120',
  },
];

const mockFeaturedNFTs = [
  {
    id: '1',
    title: 'Exclusive Track NFT',
    artist: 'Daft Punk',
    price: '2.5 ETH',
    rarity: 'Legendary',
    imageUrl: 'https://via.placeholder.com/100',
  },
  {
    id: '2',
    title: 'Album Cover NFT',
    artist: 'The Weeknd',
    price: '1.8 ETH',
    rarity: 'Epic',
    imageUrl: 'https://via.placeholder.com/100',
  },
];

const mockTradingHighlights = [
  {
    id: '1',
    songTitle: 'Blinding Lights',
    artist: 'The Weeknd',
    priceChange: '+15.2%',
    volume: '2.4K ETH',
    isPositive: true,
  },
  {
    id: '2',
    songTitle: 'Dance Monkey',
    artist: 'Tones and I',
    priceChange: '-3.1%',
    volume: '1.8K ETH',
    isPositive: false,
  },
];

export const HomeScreen = () => {
  const [currentSong, setCurrentSong] = useState(mockFeedData[0]);
  const [isPlaying, setIsPlaying] = useState(false);

  const handlePlayPause = () => {
    setIsPlaying(!isPlaying);
    // Aquí conectaremos con el backend
  };

  const handleLike = (songId: string) => {
    // Conectar con backend
    console.log('Liked song:', songId);
  };

  const handleRepost = (songId: string) => {
    // Conectar con backend
    console.log('Reposted song:', songId);
  };

  const handleShare = (songId: string) => {
    // Conectar con backend
    console.log('Shared song:', songId);
  };

  const handleJoinVREvent = (eventId: string) => {
    // Conectar con VR backend
    console.log('Joining VR event:', eventId);
  };

  const handleBuyNFT = (nftId: string) => {
    // Conectar con NFT marketplace
    console.log('Buying NFT:', nftId);
  };

  const handleTrade = (songId: string) => {
    // Conectar con trading backend
    console.log('Trading song:', songId);
  };

  return (
    <SafeAreaView style={styles.container}>
      <StatusBar barStyle="dark-content" backgroundColor="#ffffff" />
      
      {/* Header */}
      <View style={styles.header}>
        <View style={styles.headerLeft}>
          <Image
            source={{ uri: 'https://via.placeholder.com/40' }}
            style={styles.profileImage}
          />
        </View>
        
        <View style={styles.headerCenter}>
          <Text style={styles.logo}>VIBESTREAM</Text>
        </View>
        
        <TouchableOpacity style={styles.headerRight}>
          <Ionicons name="search" size={24} color="#333" />
        </TouchableOpacity>
      </View>

      <ScrollView style={styles.scrollView} showsVerticalScrollIndicator={false}>
        
        {/* Your Feed Section */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <View style={styles.sectionTitleContainer}>
              <Ionicons name="people" size={20} color="#667eea" />
              <Text style={styles.sectionTitle}>Your Feed</Text>
            </View>
            <TouchableOpacity style={styles.allPostsButton}>
              <Text style={styles.allPostsText}>All Posts</Text>
            </TouchableOpacity>
          </View>

          {mockFeedData.map((song) => (
            <View key={song.id} style={styles.songCard}>
              <View style={styles.songCardHeader}>
                <Image source={{ uri: song.imageUrl }} style={styles.songImage} />
                <View style={styles.songInfo}>
                  <Text style={styles.songTitle} numberOfLines={1}>
                    {song.title}
                  </Text>
                  <View style={styles.artistContainer}>
                    <Text style={styles.artistName}>{song.artist}</Text>
                    {song.isVerified && (
                      <Ionicons name="checkmark-circle" size={16} color="#667eea" />
                    )}
                  </View>
                </View>
                <Text style={styles.duration}>{song.duration}</Text>
              </View>

              <View style={styles.songMetrics}>
                <View style={styles.metricItem}>
                  <Ionicons name="repeat" size={16} color="#666" />
                  <Text style={styles.metricText}>{song.reposts}</Text>
                </View>
                <View style={styles.metricItem}>
                  <Ionicons name="heart" size={16} color="#666" />
                  <Text style={styles.metricText}>{song.likes}</Text>
                </View>
                <View style={styles.metricItem}>
                  <Ionicons name="chatbubble" size={16} color="#666" />
                  <Text style={styles.metricText}>{song.comments}</Text>
                </View>
                <Text style={styles.playsText}>{song.plays} Plays</Text>
              </View>

              <View style={styles.songActions}>
                <TouchableOpacity onPress={() => handleRepost(song.id)}>
                  <Ionicons name="repeat" size={20} color="#666" />
                </TouchableOpacity>
                <TouchableOpacity onPress={() => handleLike(song.id)}>
                  <Ionicons name="heart-outline" size={20} color="#666" />
                </TouchableOpacity>
                <TouchableOpacity onPress={() => handleShare(song.id)}>
                  <Ionicons name="share-outline" size={20} color="#666" />
                </TouchableOpacity>
                <TouchableOpacity>
                  <Ionicons name="ellipsis-horizontal" size={20} color="#666" />
                </TouchableOpacity>
              </View>
            </View>
          ))}
        </View>

        {/* Live VR Events Section */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>🎪 Live VR Events</Text>
            <TouchableOpacity>
              <Text style={styles.seeAllText}>See All</Text>
            </TouchableOpacity>
          </View>
          
          <ScrollView horizontal showsHorizontalScrollIndicator={false}>
            {mockVREvents.map((event) => (
              <TouchableOpacity
                key={event.id}
                style={styles.vrEventCard}
                onPress={() => handleJoinVREvent(event.id)}
              >
                <LinearGradient
                  colors={['#667eea', '#764ba2']}
                  style={styles.vrEventGradient}
                >
                  <Image source={{ uri: event.imageUrl }} style={styles.vrEventImage} />
                  <View style={styles.vrEventInfo}>
                    <Text style={styles.vrEventTitle}>{event.title}</Text>
                    <Text style={styles.vrEventArtist}>{event.artist}</Text>
                    <Text style={styles.vrEventDate}>{event.date}</Text>
                    <View style={styles.vrEventStats}>
                      <Text style={styles.vrEventAttendees}>
                        {event.attendees}/{event.maxAttendees}
                      </Text>
                      <Text style={styles.vrEventPrice}>{event.price}</Text>
                    </View>
                  </View>
                </LinearGradient>
              </TouchableOpacity>
            ))}
          </ScrollView>
        </View>

        {/* Featured NFTs Section */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>🖼️ Featured NFTs</Text>
            <TouchableOpacity>
              <Text style={styles.seeAllText}>See All</Text>
            </TouchableOpacity>
          </View>
          
          <ScrollView horizontal showsHorizontalScrollIndicator={false}>
            {mockFeaturedNFTs.map((nft) => (
              <TouchableOpacity
                key={nft.id}
                style={styles.nftCard}
                onPress={() => handleBuyNFT(nft.id)}
              >
                <Image source={{ uri: nft.imageUrl }} style={styles.nftImage} />
                <View style={styles.nftInfo}>
                  <Text style={styles.nftTitle}>{nft.title}</Text>
                  <Text style={styles.nftArtist}>{nft.artist}</Text>
                  <View style={styles.nftPriceContainer}>
                    <Text style={styles.nftPrice}>{nft.price}</Text>
                    <View style={styles.nftRarityBadge}>
                      <Text style={styles.nftRarityText}>{nft.rarity}</Text>
                    </View>
                  </View>
                </View>
              </TouchableOpacity>
            ))}
          </ScrollView>
        </View>

        {/* Trading Highlights Section */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>💰 Trading Highlights</Text>
            <TouchableOpacity>
              <Text style={styles.seeAllText}>See All</Text>
            </TouchableOpacity>
          </View>
          
          {mockTradingHighlights.map((trade) => (
            <TouchableOpacity
              key={trade.id}
              style={styles.tradingCard}
              onPress={() => handleTrade(trade.id)}
            >
              <View style={styles.tradingInfo}>
                <Text style={styles.tradingSongTitle}>{trade.songTitle}</Text>
                <Text style={styles.tradingArtist}>{trade.artist}</Text>
              </View>
              <View style={styles.tradingStats}>
                <Text style={[
                  styles.tradingPriceChange,
                  { color: trade.isPositive ? '#4CAF50' : '#F44336' }
                ]}>
                  {trade.priceChange}
                </Text>
                <Text style={styles.tradingVolume}>{trade.volume}</Text>
              </View>
            </TouchableOpacity>
          ))}
        </View>

        {/* Your Analytics Section */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>📊 Your Analytics</Text>
          </View>
          
          <View style={styles.analyticsGrid}>
            <View style={styles.analyticsCard}>
              <Ionicons name="play" size={24} color="#667eea" />
              <Text style={styles.analyticsValue}>1,247</Text>
              <Text style={styles.analyticsLabel}>Total Listens</Text>
            </View>
            <View style={styles.analyticsCard}>
              <Ionicons name="trending-up" size={24} color="#4CAF50" />
              <Text style={styles.analyticsValue}>2.4 ETH</Text>
              <Text style={styles.analyticsLabel}>Portfolio Value</Text>
            </View>
            <View style={styles.analyticsCard}>
              <Ionicons name="people" size={24} color="#FF9800" />
              <Text style={styles.analyticsValue}>156</Text>
              <Text style={styles.analyticsLabel}>Followers</Text>
            </View>
            <View style={styles.analyticsCard}>
              <Ionicons name="trophy" size={24} color="#9C27B0" />
              <Text style={styles.analyticsValue}>8</Text>
              <Text style={styles.analyticsLabel}>VR Events</Text>
            </View>
          </View>
        </View>

        {/* Bottom spacing for mini player */}
        <View style={{ height: 80 }} />
      </ScrollView>

      {/* Mini Player */}
      {currentSong && (
        <View style={styles.miniPlayer}>
          <Image source={{ uri: currentSong.imageUrl }} style={styles.miniPlayerImage} />
          <View style={styles.miniPlayerInfo}>
            <Text style={styles.miniPlayerTitle} numberOfLines={1}>
              {currentSong.title}
            </Text>
            <Text style={styles.miniPlayerArtist} numberOfLines={1}>
              {currentSong.artist}
            </Text>
          </View>
          <TouchableOpacity onPress={handlePlayPause} style={styles.miniPlayerButton}>
            <Ionicons
              name={isPlaying ? "pause" : "play"}
              size={24}
              color="#667eea"
            />
          </TouchableOpacity>
        </View>
      )}
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#ffffff',
  },
  header: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingHorizontal: 20,
    paddingVertical: 15,
    borderBottomWidth: 1,
    borderBottomColor: '#f0f0f0',
  },
  headerLeft: {
    flex: 1,
  },
  profileImage: {
    width: 40,
    height: 40,
    borderRadius: 20,
  },
  headerCenter: {
    flex: 2,
    alignItems: 'center',
  },
  logo: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#667eea',
  },
  headerRight: {
    flex: 1,
    alignItems: 'flex-end',
  },
  scrollView: {
    flex: 1,
  },
  section: {
    marginBottom: 30,
  },
  sectionHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: 20,
    marginBottom: 15,
  },
  sectionTitleContainer: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  sectionTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#333',
    marginLeft: 8,
  },
  allPostsButton: {
    backgroundColor: '#667eea',
    paddingHorizontal: 15,
    paddingVertical: 8,
    borderRadius: 20,
  },
  allPostsText: {
    color: 'white',
    fontSize: 14,
    fontWeight: '600',
  },
  seeAllText: {
    color: '#667eea',
    fontSize: 14,
    fontWeight: '600',
  },
  songCard: {
    backgroundColor: '#f8f9fa',
    marginHorizontal: 20,
    marginBottom: 15,
    borderRadius: 12,
    padding: 15,
  },
  songCardHeader: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: 10,
  },
  songImage: {
    width: 60,
    height: 60,
    borderRadius: 8,
    marginRight: 12,
  },
  songInfo: {
    flex: 1,
  },
  songTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#333',
    marginBottom: 4,
  },
  artistContainer: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  artistName: {
    fontSize: 14,
    color: '#666',
    marginRight: 5,
  },
  duration: {
    fontSize: 14,
    color: '#999',
  },
  songMetrics: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: 10,
  },
  metricItem: {
    flexDirection: 'row',
    alignItems: 'center',
    marginRight: 15,
  },
  metricText: {
    fontSize: 12,
    color: '#666',
    marginLeft: 4,
  },
  playsText: {
    fontSize: 12,
    color: '#999',
    marginLeft: 'auto',
  },
  songActions: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    borderTopWidth: 1,
    borderTopColor: '#eee',
    paddingTop: 10,
  },
  vrEventCard: {
    width: 200,
    marginLeft: 20,
    borderRadius: 12,
    overflow: 'hidden',
  },
  vrEventGradient: {
    padding: 15,
  },
  vrEventImage: {
    width: '100%',
    height: 100,
    borderRadius: 8,
    marginBottom: 10,
  },
  vrEventInfo: {
    flex: 1,
  },
  vrEventTitle: {
    fontSize: 16,
    fontWeight: 'bold',
    color: 'white',
    marginBottom: 4,
  },
  vrEventArtist: {
    fontSize: 14,
    color: 'rgba(255,255,255,0.8)',
    marginBottom: 4,
  },
  vrEventDate: {
    fontSize: 12,
    color: 'rgba(255,255,255,0.7)',
    marginBottom: 8,
  },
  vrEventStats: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
  vrEventAttendees: {
    fontSize: 12,
    color: 'rgba(255,255,255,0.8)',
  },
  vrEventPrice: {
    fontSize: 12,
    color: 'rgba(255,255,255,0.8)',
    fontWeight: '600',
  },
  nftCard: {
    width: 150,
    marginLeft: 20,
    backgroundColor: '#f8f9fa',
    borderRadius: 12,
    padding: 12,
  },
  nftImage: {
    width: '100%',
    height: 100,
    borderRadius: 8,
    marginBottom: 10,
  },
  nftInfo: {
    flex: 1,
  },
  nftTitle: {
    fontSize: 14,
    fontWeight: '600',
    color: '#333',
    marginBottom: 4,
  },
  nftArtist: {
    fontSize: 12,
    color: '#666',
    marginBottom: 8,
  },
  nftPriceContainer: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  nftPrice: {
    fontSize: 14,
    fontWeight: 'bold',
    color: '#667eea',
  },
  nftRarityBadge: {
    backgroundColor: '#FFD700',
    paddingHorizontal: 6,
    paddingVertical: 2,
    borderRadius: 4,
  },
  nftRarityText: {
    fontSize: 10,
    color: '#333',
    fontWeight: '600',
  },
  tradingCard: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    backgroundColor: '#f8f9fa',
    marginHorizontal: 20,
    marginBottom: 10,
    padding: 15,
    borderRadius: 12,
  },
  tradingInfo: {
    flex: 1,
  },
  tradingSongTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#333',
    marginBottom: 4,
  },
  tradingArtist: {
    fontSize: 14,
    color: '#666',
  },
  tradingStats: {
    alignItems: 'flex-end',
  },
  tradingPriceChange: {
    fontSize: 16,
    fontWeight: 'bold',
    marginBottom: 4,
  },
  tradingVolume: {
    fontSize: 12,
    color: '#666',
  },
  analyticsGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    paddingHorizontal: 20,
  },
  analyticsCard: {
    width: (width - 60) / 2,
    backgroundColor: '#f8f9fa',
    padding: 15,
    marginBottom: 10,
    marginHorizontal: 5,
    borderRadius: 12,
    alignItems: 'center',
  },
  analyticsValue: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#333',
    marginTop: 8,
    marginBottom: 4,
  },
  analyticsLabel: {
    fontSize: 12,
    color: '#666',
    textAlign: 'center',
  },
  miniPlayer: {
    position: 'absolute',
    bottom: 0,
    left: 0,
    right: 0,
    backgroundColor: 'white',
    flexDirection: 'row',
    alignItems: 'center',
    paddingHorizontal: 20,
    paddingVertical: 15,
    borderTopWidth: 1,
    borderTopColor: '#eee',
  },
  miniPlayerImage: {
    width: 50,
    height: 50,
    borderRadius: 8,
    marginRight: 15,
  },
  miniPlayerInfo: {
    flex: 1,
  },
  miniPlayerTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#333',
    marginBottom: 4,
  },
  miniPlayerArtist: {
    fontSize: 14,
    color: '#666',
  },
  miniPlayerButton: {
    padding: 10,
  },
}); 