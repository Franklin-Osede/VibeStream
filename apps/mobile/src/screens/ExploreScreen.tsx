import React from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  TouchableOpacity,
  Image,
  TextInput,
  FlatList,
} from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { Ionicons } from '@expo/vector-icons';

interface ExploreCategory {
  id: string;
  title: string;
  icon: string;
  color: string;
  count: number;
}

interface ExploreItem {
  id: string;
  title: string;
  subtitle: string;
  imageUrl: string;
  type: 'genre' | 'mood' | 'artist' | 'playlist' | 'vr_event' | 'nft';
}

const categories: ExploreCategory[] = [
  { id: '1', title: 'Genres', icon: 'musical-notes', color: '#8B5CF6', count: 25 },
  { id: '2', title: 'Moods', icon: 'heart', color: '#EF4444', count: 12 },
  { id: '3', title: 'Artists', icon: 'person', color: '#10B981', count: 150 },
  { id: '4', title: 'VR Events', icon: 'glasses', color: '#F59E0B', count: 8 },
  { id: '5', title: 'NFTs', icon: 'diamond', color: '#06B6D4', count: 45 },
  { id: '6', title: 'Playlists', icon: 'list', color: '#EC4899', count: 200 },
];

const exploreItems: ExploreItem[] = [
  {
    id: '1',
    title: 'Electronic',
    subtitle: 'Discover electronic music',
    imageUrl: 'https://via.placeholder.com/120',
    type: 'genre',
  },
  {
    id: '2',
    title: 'Chill Vibes',
    subtitle: 'Relaxing music for your day',
    imageUrl: 'https://via.placeholder.com/120',
    type: 'mood',
  },
  {
    id: '3',
    title: 'Neon Dreams VR',
    subtitle: 'Virtual concert experience',
    imageUrl: 'https://via.placeholder.com/120',
    type: 'vr_event',
  },
  {
    id: '4',
    title: 'Genesis Collection',
    subtitle: 'Exclusive NFT music',
    imageUrl: 'https://via.placeholder.com/120',
    type: 'nft',
  },
  {
    id: '5',
    title: 'Luna Echo',
    subtitle: 'Rising electronic artist',
    imageUrl: 'https://via.placeholder.com/120',
    type: 'artist',
  },
  {
    id: '6',
    title: 'Weekend Vibes',
    subtitle: 'Perfect for your weekend',
    imageUrl: 'https://via.placeholder.com/120',
    type: 'playlist',
  },
];

export default function ExploreScreen() {
  const [searchQuery, setSearchQuery] = React.useState('');
  const [selectedCategory, setSelectedCategory] = React.useState<string | null>(null);

  const renderCategory = ({ item }: { item: ExploreCategory }) => (
    <TouchableOpacity
      style={[
        styles.categoryCard,
        { backgroundColor: item.color + '20' },
        selectedCategory === item.id && { backgroundColor: item.color + '40' }
      ]}
      onPress={() => setSelectedCategory(selectedCategory === item.id ? null : item.id)}
    >
      <View style={[styles.categoryIcon, { backgroundColor: item.color }]}>
        <Ionicons name={item.icon as any} size={24} color="#FFFFFF" />
      </View>
      <Text style={styles.categoryTitle}>{item.title}</Text>
      <Text style={styles.categoryCount}>{item.count} items</Text>
    </TouchableOpacity>
  );

  const renderExploreItem = ({ item }: { item: ExploreItem }) => (
    <TouchableOpacity style={styles.exploreItem}>
      <Image source={{ uri: item.imageUrl }} style={styles.exploreImage} />
      <View style={styles.exploreOverlay}>
        <Text style={styles.exploreTitle}>{item.title}</Text>
        <Text style={styles.exploreSubtitle}>{item.subtitle}</Text>
      </View>
    </TouchableOpacity>
  );

  return (
    <SafeAreaView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.title}>Explore</Text>
        <Text style={styles.subtitle}>Discover new music and experiences</Text>
      </View>

      {/* Barra de búsqueda */}
      <View style={styles.searchContainer}>
        <View style={styles.searchInputContainer}>
          <Ionicons name="search" size={20} color="#6B7280" style={styles.searchIcon} />
          <TextInput
            style={styles.searchInput}
            placeholder="Search for music, artists, events..."
            placeholderTextColor="#9CA3AF"
            value={searchQuery}
            onChangeText={setSearchQuery}
          />
          {searchQuery.length > 0 && (
            <TouchableOpacity onPress={() => setSearchQuery('')}>
              <Ionicons name="close-circle" size={20} color="#6B7280" />
            </TouchableOpacity>
          )}
        </View>
      </View>

      <ScrollView style={styles.content} showsVerticalScrollIndicator={false}>
        {/* Categorías */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Categories</Text>
          <FlatList
            data={categories}
            renderItem={renderCategory}
            keyExtractor={(item) => item.id}
            horizontal
            showsHorizontalScrollIndicator={false}
            contentContainerStyle={styles.categoriesList}
          />
        </View>

        {/* Contenido de exploración */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>
            {selectedCategory 
              ? categories.find(c => c.id === selectedCategory)?.title 
              : 'Recommended for you'
            }
          </Text>
          <View style={styles.exploreGrid}>
            {exploreItems.map((item) => (
              <View key={item.id} style={styles.exploreItemContainer}>
                {renderExploreItem({ item })}
              </View>
            ))}
          </View>
        </View>

        {/* Sección de eventos VR en vivo */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>Live VR Events</Text>
            <TouchableOpacity>
              <Text style={styles.seeAllText}>See all</Text>
            </TouchableOpacity>
          </View>
          <ScrollView horizontal showsHorizontalScrollIndicator={false}>
            {exploreItems.filter(item => item.type === 'vr_event').map((item) => (
              <View key={item.id} style={styles.liveEventCard}>
                <Image source={{ uri: item.imageUrl }} style={styles.liveEventImage} />
                <View style={styles.liveIndicator}>
                  <View style={styles.liveDot} />
                  <Text style={styles.liveText}>LIVE</Text>
                </View>
                <View style={styles.liveEventInfo}>
                  <Text style={styles.liveEventTitle}>{item.title}</Text>
                  <Text style={styles.liveEventSubtitle}>{item.subtitle}</Text>
                </View>
              </View>
            ))}
          </ScrollView>
        </View>

        {/* Sección de NFTs destacados */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>Featured NFTs</Text>
            <TouchableOpacity>
              <Text style={styles.seeAllText}>See all</Text>
            </TouchableOpacity>
          </View>
          <ScrollView horizontal showsHorizontalScrollIndicator={false}>
            {exploreItems.filter(item => item.type === 'nft').map((item) => (
              <View key={item.id} style={styles.nftCard}>
                <Image source={{ uri: item.imageUrl }} style={styles.nftImage} />
                <View style={styles.nftInfo}>
                  <Text style={styles.nftTitle}>{item.title}</Text>
                  <Text style={styles.nftSubtitle}>{item.subtitle}</Text>
                  <View style={styles.nftPrice}>
                    <Ionicons name="diamond" size={12} color="#06B6D4" />
                    <Text style={styles.nftPriceText}>0.5 ETH</Text>
                  </View>
                </View>
              </View>
            ))}
          </ScrollView>
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
  searchContainer: {
    paddingHorizontal: 20,
    paddingVertical: 16,
  },
  searchInputContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#F3F4F6',
    borderRadius: 12,
    paddingHorizontal: 16,
    paddingVertical: 12,
  },
  searchIcon: {
    marginRight: 12,
  },
  searchInput: {
    flex: 1,
    fontSize: 16,
    color: '#111827',
  },
  content: {
    flex: 1,
  },
  section: {
    marginBottom: 24,
  },
  sectionHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: 20,
    marginBottom: 16,
  },
  sectionTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#111827',
    paddingHorizontal: 20,
    marginBottom: 16,
  },
  seeAllText: {
    fontSize: 14,
    color: '#8B5CF6',
    fontWeight: '600',
  },
  categoriesList: {
    paddingHorizontal: 20,
  },
  categoryCard: {
    width: 100,
    padding: 16,
    borderRadius: 12,
    marginRight: 12,
    alignItems: 'center',
  },
  categoryIcon: {
    width: 48,
    height: 48,
    borderRadius: 24,
    justifyContent: 'center',
    alignItems: 'center',
    marginBottom: 8,
  },
  categoryTitle: {
    fontSize: 14,
    fontWeight: '600',
    color: '#111827',
    textAlign: 'center',
    marginBottom: 4,
  },
  categoryCount: {
    fontSize: 12,
    color: '#6B7280',
  },
  exploreGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    paddingHorizontal: 20,
  },
  exploreItemContainer: {
    width: '48%',
    marginBottom: 16,
    marginRight: '2%',
  },
  exploreItem: {
    height: 120,
    borderRadius: 12,
    overflow: 'hidden',
  },
  exploreImage: {
    width: '100%',
    height: '100%',
  },
  exploreOverlay: {
    position: 'absolute',
    bottom: 0,
    left: 0,
    right: 0,
    backgroundColor: 'rgba(0,0,0,0.7)',
    padding: 12,
  },
  exploreTitle: {
    fontSize: 14,
    fontWeight: '600',
    color: '#FFFFFF',
    marginBottom: 2,
  },
  exploreSubtitle: {
    fontSize: 12,
    color: '#D1D5DB',
  },
  liveEventCard: {
    width: 200,
    marginRight: 16,
    marginLeft: 20,
  },
  liveEventImage: {
    width: '100%',
    height: 120,
    borderRadius: 12,
  },
  liveIndicator: {
    position: 'absolute',
    top: 8,
    left: 8,
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#EF4444',
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 12,
  },
  liveDot: {
    width: 6,
    height: 6,
    borderRadius: 3,
    backgroundColor: '#FFFFFF',
    marginRight: 4,
  },
  liveText: {
    fontSize: 10,
    fontWeight: 'bold',
    color: '#FFFFFF',
  },
  liveEventInfo: {
    padding: 12,
  },
  liveEventTitle: {
    fontSize: 14,
    fontWeight: '600',
    color: '#111827',
    marginBottom: 2,
  },
  liveEventSubtitle: {
    fontSize: 12,
    color: '#6B7280',
  },
  nftCard: {
    width: 150,
    marginRight: 16,
    marginLeft: 20,
  },
  nftImage: {
    width: '100%',
    height: 100,
    borderRadius: 12,
  },
  nftInfo: {
    padding: 8,
  },
  nftTitle: {
    fontSize: 12,
    fontWeight: '600',
    color: '#111827',
    marginBottom: 2,
  },
  nftSubtitle: {
    fontSize: 10,
    color: '#6B7280',
    marginBottom: 4,
  },
  nftPrice: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  nftPriceText: {
    fontSize: 10,
    fontWeight: '600',
    color: '#06B6D4',
    marginLeft: 4,
  },
}); 