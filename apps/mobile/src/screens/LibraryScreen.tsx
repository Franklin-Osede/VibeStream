import React from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  TouchableOpacity,
  Image,
  FlatList,
} from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { Ionicons } from '@expo/vector-icons';

interface LibraryItem {
  id: string;
  title: string;
  subtitle: string;
  imageUrl: string;
  type: 'playlist' | 'album' | 'track' | 'nft' | 'vr_event';
  isDownloaded?: boolean;
  isLiked?: boolean;
  duration?: string;
  trackCount?: number;
}

const mockLibraryData: LibraryItem[] = [
  {
    id: '1',
    title: 'Liked Songs',
    subtitle: 'Your favorite tracks',
    imageUrl: 'https://via.placeholder.com/80',
    type: 'playlist',
    isLiked: true,
    trackCount: 127,
  },
  {
    id: '2',
    title: 'Midnight Vibes',
    subtitle: 'Luna Echo',
    imageUrl: 'https://via.placeholder.com/80',
    type: 'album',
    isDownloaded: true,
    trackCount: 12,
  },
  {
    id: '3',
    title: 'Cosmic Journey',
    subtitle: 'Stellar Sound',
    imageUrl: 'https://via.placeholder.com/80',
    type: 'album',
    trackCount: 8,
  },
  {
    id: '4',
    title: 'Neon Dreams VR',
    subtitle: 'Virtual concert recording',
    imageUrl: 'https://via.placeholder.com/80',
    type: 'vr_event',
    duration: '2h 15m',
  },
  {
    id: '5',
    title: 'Genesis Collection #1',
    subtitle: 'Exclusive NFT',
    imageUrl: 'https://via.placeholder.com/80',
    type: 'nft',
  },
  {
    id: '6',
    title: 'Weekend Vibes',
    subtitle: 'Perfect for your weekend',
    imageUrl: 'https://via.placeholder.com/80',
    type: 'playlist',
    trackCount: 45,
  },
  {
    id: '7',
    title: 'Electronic Essentials',
    subtitle: 'Best electronic tracks',
    imageUrl: 'https://via.placeholder.com/80',
    type: 'playlist',
    trackCount: 89,
  },
  {
    id: '8',
    title: 'Chill Beats',
    subtitle: 'Relaxing music collection',
    imageUrl: 'https://via.placeholder.com/80',
    type: 'playlist',
    trackCount: 67,
  },
];

const libraryCategories = [
  { id: 'all', title: 'All', icon: 'library' },
  { id: 'playlists', title: 'Playlists', icon: 'list' },
  { id: 'albums', title: 'Albums', icon: 'albums' },
  { id: 'tracks', title: 'Tracks', icon: 'musical-note' },
  { id: 'nfts', title: 'NFTs', icon: 'diamond' },
  { id: 'vr_events', title: 'VR Events', icon: 'glasses' },
];

export default function LibraryScreen() {
  const [selectedCategory, setSelectedCategory] = React.useState('all');
  const [sortBy, setSortBy] = React.useState('recent');

  const filteredData = React.useMemo(() => {
    if (selectedCategory === 'all') {
      return mockLibraryData;
    }
    return mockLibraryData.filter(item => item.type === selectedCategory.slice(0, -1));
  }, [selectedCategory]);

  const getTypeIcon = (type: string) => {
    switch (type) {
      case 'playlist':
        return 'list';
      case 'album':
        return 'albums';
      case 'track':
        return 'musical-note';
      case 'nft':
        return 'diamond';
      case 'vr_event':
        return 'glasses';
      default:
        return 'document';
    }
  };

  const getTypeColor = (type: string) => {
    switch (type) {
      case 'playlist':
        return '#EC4899';
      case 'album':
        return '#8B5CF6';
      case 'track':
        return '#10B981';
      case 'nft':
        return '#06B6D4';
      case 'vr_event':
        return '#F59E0B';
      default:
        return '#6B7280';
    }
  };

  const renderLibraryItem = ({ item }: { item: LibraryItem }) => (
    <TouchableOpacity style={styles.libraryItem}>
      <Image source={{ uri: item.imageUrl }} style={styles.itemImage} />
      <View style={styles.itemContent}>
        <View style={styles.itemHeader}>
          <Text style={styles.itemTitle} numberOfLines={1}>
            {item.title}
          </Text>
          <View style={styles.itemActions}>
            {item.isDownloaded && (
              <Ionicons name="download" size={16} color="#10B981" />
            )}
            {item.isLiked && (
              <Ionicons name="heart" size={16} color="#EF4444" />
            )}
          </View>
        </View>
        <Text style={styles.itemSubtitle} numberOfLines={1}>
          {item.subtitle}
        </Text>
        <View style={styles.itemMeta}>
          <View style={[styles.typeBadge, { backgroundColor: getTypeColor(item.type) + '20' }]}>
            <Ionicons 
              name={getTypeIcon(item.type) as any} 
              size={12} 
              color={getTypeColor(item.type)} 
            />
            <Text style={[styles.typeText, { color: getTypeColor(item.type) }]}>
              {item.type.toUpperCase()}
            </Text>
          </View>
          {item.trackCount && (
            <Text style={styles.trackCount}>{item.trackCount} tracks</Text>
          )}
          {item.duration && (
            <Text style={styles.duration}>{item.duration}</Text>
          )}
        </View>
      </View>
      <TouchableOpacity style={styles.moreButton}>
        <Ionicons name="ellipsis-vertical" size={20} color="#6B7280" />
      </TouchableOpacity>
    </TouchableOpacity>
  );

  return (
    <SafeAreaView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.title}>Your Library</Text>
        <View style={styles.headerActions}>
          <TouchableOpacity style={styles.searchButton}>
            <Ionicons name="search" size={24} color="#6B7280" />
          </TouchableOpacity>
          <TouchableOpacity style={styles.addButton}>
            <Ionicons name="add" size={24} color="#6B7280" />
          </TouchableOpacity>
        </View>
      </View>

      {/* Categorías */}
      <ScrollView 
        horizontal 
        showsHorizontalScrollIndicator={false}
        style={styles.categoriesContainer}
        contentContainerStyle={styles.categoriesContent}
      >
        {libraryCategories.map((category) => (
          <TouchableOpacity
            key={category.id}
            style={[
              styles.categoryButton,
              selectedCategory === category.id && styles.categoryButtonActive
            ]}
            onPress={() => setSelectedCategory(category.id)}
          >
            <Ionicons 
              name={category.icon as any} 
              size={16} 
              color={selectedCategory === category.id ? '#FFFFFF' : '#6B7280'} 
            />
            <Text style={[
              styles.categoryText,
              selectedCategory === category.id && styles.categoryTextActive
            ]}>
              {category.title}
            </Text>
          </TouchableOpacity>
        ))}
      </ScrollView>

      {/* Ordenar por */}
      <View style={styles.sortContainer}>
        <Text style={styles.sortLabel}>Sort by:</Text>
        <TouchableOpacity 
          style={styles.sortButton}
          onPress={() => setSortBy(sortBy === 'recent' ? 'name' : 'recent')}
        >
          <Text style={styles.sortText}>
            {sortBy === 'recent' ? 'Recently Added' : 'Name'}
          </Text>
          <Ionicons name="chevron-down" size={16} color="#6B7280" />
        </TouchableOpacity>
      </View>

      {/* Lista de elementos */}
      <FlatList
        data={filteredData}
        renderItem={renderLibraryItem}
        keyExtractor={(item) => item.id}
        style={styles.libraryList}
        contentContainerStyle={styles.libraryListContent}
        showsVerticalScrollIndicator={false}
      />

      {/* Botón flotante para crear playlist */}
      <TouchableOpacity style={styles.fab}>
        <Ionicons name="add" size={24} color="#FFFFFF" />
      </TouchableOpacity>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#FFFFFF',
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: 20,
    paddingVertical: 16,
    borderBottomWidth: 1,
    borderBottomColor: '#E5E7EB',
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#111827',
  },
  headerActions: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  searchButton: {
    marginRight: 16,
  },
  addButton: {
    marginRight: 8,
  },
  categoriesContainer: {
    borderBottomWidth: 1,
    borderBottomColor: '#E5E7EB',
  },
  categoriesContent: {
    paddingHorizontal: 20,
    paddingVertical: 12,
  },
  categoryButton: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingHorizontal: 16,
    paddingVertical: 8,
    marginRight: 12,
    borderRadius: 20,
    backgroundColor: '#F3F4F6',
  },
  categoryButtonActive: {
    backgroundColor: '#8B5CF6',
  },
  categoryText: {
    fontSize: 14,
    fontWeight: '600',
    color: '#6B7280',
    marginLeft: 6,
  },
  categoryTextActive: {
    color: '#FFFFFF',
  },
  sortContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingHorizontal: 20,
    paddingVertical: 12,
    borderBottomWidth: 1,
    borderBottomColor: '#E5E7EB',
  },
  sortLabel: {
    fontSize: 14,
    color: '#6B7280',
    marginRight: 8,
  },
  sortButton: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingHorizontal: 12,
    paddingVertical: 6,
    backgroundColor: '#F3F4F6',
    borderRadius: 16,
  },
  sortText: {
    fontSize: 14,
    fontWeight: '500',
    color: '#111827',
    marginRight: 4,
  },
  libraryList: {
    flex: 1,
  },
  libraryListContent: {
    padding: 20,
  },
  libraryItem: {
    flexDirection: 'row',
    alignItems: 'center',
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
  itemActions: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  itemSubtitle: {
    fontSize: 14,
    color: '#6B7280',
    marginBottom: 8,
  },
  itemMeta: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  typeBadge: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingHorizontal: 8,
    paddingVertical: 2,
    borderRadius: 4,
    marginRight: 8,
  },
  typeText: {
    fontSize: 10,
    fontWeight: '600',
    marginLeft: 4,
  },
  trackCount: {
    fontSize: 12,
    color: '#6B7280',
    marginRight: 8,
  },
  duration: {
    fontSize: 12,
    color: '#6B7280',
  },
  moreButton: {
    padding: 8,
  },
  fab: {
    position: 'absolute',
    bottom: 20,
    right: 20,
    width: 56,
    height: 56,
    borderRadius: 28,
    backgroundColor: '#8B5CF6',
    justifyContent: 'center',
    alignItems: 'center',
    elevation: 8,
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 4,
    },
    shadowOpacity: 0.3,
    shadowRadius: 4.65,
  },
}); 