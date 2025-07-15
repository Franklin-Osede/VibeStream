import React from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  RefreshControl,
  TouchableOpacity,
  Image,
} from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { Ionicons } from '@expo/vector-icons';

interface TrendingItem {
  id: string;
  type: 'track' | 'album' | 'artist' | 'vr_event' | 'nft';
  title: string;
  artist?: string;
  imageUrl: string;
  plays: number;
  trend: 'up' | 'down' | 'stable';
  change: number;
}

const mockTrendingData: TrendingItem[] = [
  {
    id: '1',
    type: 'track',
    title: 'Midnight Vibes',
    artist: 'Luna Echo',
    imageUrl: 'https://via.placeholder.com/60',
    plays: 1250000,
    trend: 'up',
    change: 15,
  },
  {
    id: '2',
    type: 'vr_event',
    title: 'Virtual Concert: Neon Dreams',
    artist: 'Cyber Collective',
    imageUrl: 'https://via.placeholder.com/60',
    plays: 89000,
    trend: 'up',
    change: 45,
  },
  {
    id: '3',
    type: 'nft',
    title: 'Genesis Collection #1',
    artist: 'Digital Art Studio',
    imageUrl: 'https://via.placeholder.com/60',
    plays: 0,
    trend: 'up',
    change: 200,
  },
  {
    id: '4',
    type: 'album',
    title: 'Cosmic Journey',
    artist: 'Stellar Sound',
    imageUrl: 'https://via.placeholder.com/60',
    plays: 450000,
    trend: 'up',
    change: 8,
  },
  {
    id: '5',
    type: 'artist',
    title: 'Neon Pulse',
    artist: 'Electronic Pioneer',
    imageUrl: 'https://via.placeholder.com/60',
    plays: 2100000,
    trend: 'up',
    change: 25,
  },
];

export default function TrendingScreen() {
  const [refreshing, setRefreshing] = React.useState(false);
  const [selectedFilter, setSelectedFilter] = React.useState('all');

  const onRefresh = React.useCallback(() => {
    setRefreshing(true);
    // Simular carga de datos
    setTimeout(() => {
      setRefreshing(false);
    }, 2000);
  }, []);

  const getTrendIcon = (trend: string) => {
    switch (trend) {
      case 'up':
        return <Ionicons name="trending-up" size={16} color="#10B981" />;
      case 'down':
        return <Ionicons name="trending-down" size={16} color="#EF4444" />;
      default:
        return <Ionicons name="remove" size={16} color="#6B7280" />;
    }
  };

  const formatPlays = (plays: number) => {
    if (plays >= 1000000) {
      return `${(plays / 1000000).toFixed(1)}M`;
    } else if (plays >= 1000) {
      return `${(plays / 1000).toFixed(1)}K`;
    }
    return plays.toString();
  };

  const renderTrendingItem = (item: TrendingItem) => (
    <TouchableOpacity key={item.id} style={styles.trendingItem}>
      <Image source={{ uri: item.imageUrl }} style={styles.itemImage} />
      <View style={styles.itemContent}>
        <View style={styles.itemHeader}>
          <Text style={styles.itemTitle} numberOfLines={1}>
            {item.title}
          </Text>
          <View style={styles.trendContainer}>
            {getTrendIcon(item.trend)}
            <Text style={[styles.trendText, { color: item.trend === 'up' ? '#10B981' : '#EF4444' }]}>
              {item.change}%
            </Text>
          </View>
        </View>
        <Text style={styles.itemArtist} numberOfLines={1}>
          {item.artist}
        </Text>
        <View style={styles.itemStats}>
          <Ionicons name="play" size={12} color="#6B7280" />
          <Text style={styles.playsText}>{formatPlays(item.plays)} plays</Text>
          <View style={styles.typeBadge}>
            <Text style={styles.typeText}>{item.type.toUpperCase()}</Text>
          </View>
        </View>
      </View>
    </TouchableOpacity>
  );

  return (
    <SafeAreaView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.title}>Trending</Text>
        <Text style={styles.subtitle}>What's hot right now</Text>
      </View>

      {/* Filtros */}
      <ScrollView 
        horizontal 
        showsHorizontalScrollIndicator={false}
        style={styles.filtersContainer}
        contentContainerStyle={styles.filtersContent}
      >
        {['all', 'tracks', 'albums', 'artists', 'vr_events', 'nfts'].map((filter) => (
          <TouchableOpacity
            key={filter}
            style={[
              styles.filterButton,
              selectedFilter === filter && styles.filterButtonActive
            ]}
            onPress={() => setSelectedFilter(filter)}
          >
            <Text style={[
              styles.filterText,
              selectedFilter === filter && styles.filterTextActive
            ]}>
              {filter.replace('_', ' ').toUpperCase()}
            </Text>
          </TouchableOpacity>
        ))}
      </ScrollView>

      <ScrollView
        style={styles.content}
        refreshControl={
          <RefreshControl refreshing={refreshing} onRefresh={onRefresh} />
        }
      >
        <View style={styles.trendingList}>
          {mockTrendingData.map(renderTrendingItem)}
        </View>
      </ScrollView>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#FFFFFF',
  },
  header: {
    paddingHorizontal: 20,
    paddingVertical: 16,
    borderBottomWidth: 1,
    borderBottomColor: '#E5E7EB',
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#111827',
    marginBottom: 4,
  },
  subtitle: {
    fontSize: 16,
    color: '#6B7280',
  },
  filtersContainer: {
    borderBottomWidth: 1,
    borderBottomColor: '#E5E7EB',
  },
  filtersContent: {
    paddingHorizontal: 20,
    paddingVertical: 12,
  },
  filterButton: {
    paddingHorizontal: 16,
    paddingVertical: 8,
    marginRight: 12,
    borderRadius: 20,
    backgroundColor: '#F3F4F6',
  },
  filterButtonActive: {
    backgroundColor: '#8B5CF6',
  },
  filterText: {
    fontSize: 12,
    fontWeight: '600',
    color: '#6B7280',
  },
  filterTextActive: {
    color: '#FFFFFF',
  },
  content: {
    flex: 1,
  },
  trendingList: {
    padding: 20,
  },
  trendingItem: {
    flexDirection: 'row',
    marginBottom: 16,
    padding: 12,
    backgroundColor: '#F9FAFB',
    borderRadius: 12,
  },
  itemImage: {
    width: 60,
    height: 60,
    borderRadius: 8,
    marginRight: 12,
  },
  itemContent: {
    flex: 1,
    justifyContent: 'space-between',
  },
  itemHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
    marginBottom: 4,
  },
  itemTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#111827',
    flex: 1,
    marginRight: 8,
  },
  trendContainer: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  trendText: {
    fontSize: 12,
    fontWeight: '600',
    marginLeft: 4,
  },
  itemArtist: {
    fontSize: 14,
    color: '#6B7280',
    marginBottom: 8,
  },
  itemStats: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  playsText: {
    fontSize: 12,
    color: '#6B7280',
    marginLeft: 4,
    marginRight: 12,
  },
  typeBadge: {
    backgroundColor: '#E0E7FF',
    paddingHorizontal: 8,
    paddingVertical: 2,
    borderRadius: 4,
  },
  typeText: {
    fontSize: 10,
    fontWeight: '600',
    color: '#3730A3',
  },
}); 
 
 