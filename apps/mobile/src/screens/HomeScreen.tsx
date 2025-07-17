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

// Mock data mejorado basado en Audius
const mockFeedData = [
  {
    id: '1',
    title: 'U Should Know (danse)',
    artist: 'molly mcphaul',
    artistVerified: true,
    duration: '3:15',
    plays: '46',
    reposts: 25,
    likes: 32,
    comments: 9,
    imageUrl: 'https://via.placeholder.com/80',
    isLiked: false,
    isReposted: false,
  },
  {
    id: '2',
    title: 'mellodaze - in retrospect',
    artist: 'Arden Records',
    artistVerified: false,
    duration: '1:20',
    plays: '6.87K',
    reposts: 48,
    likes: 78,
    comments: 12,
    imageUrl: 'https://via.placeholder.com/80',
    isLiked: false,
    isReposted: false,
  },
  {
    id: '3',
    title: 'hi july - instrumental',
    artist: 'molly mcphaul',
    artistVerified: true,
    duration: '1:52',
    plays: '6.88K',
    reposts: 88,
    likes: 154,
    comments: 28,
    imageUrl: 'https://via.placeholder.com/80',
    isLiked: false,
    isReposted: false,
  },
  {
    id: '4',
    title: 'Karina Rosee & Stevie Rain',
    artist: 'Karina Rosee',
    artistVerified: true,
    duration: '3:21',
    plays: '3.7K',
    reposts: 76,
    likes: 108,
    comments: 30,
    imageUrl: 'https://via.placeholder.com/80',
    isLiked: false,
    isReposted: false,
  },
];

export const HomeScreen = ({ navigation }: any) => {
  const [currentSong, setCurrentSong] = useState(mockFeedData[0]);
  const [isPlaying, setIsPlaying] = useState(false);
  const [selectedFilter, setSelectedFilter] = useState('All Posts');

  const handlePlayPause = (song: any) => {
    setCurrentSong(song);
    setIsPlaying(!isPlaying);
    // Navegar al reproductor
    navigation.navigate('MusicPlayer', { song, user: { id: '1', name: 'User' } });
  };

  const handleLike = (songId: string) => {
    // Actualizar estado local
    const updatedData = mockFeedData.map(song => 
      song.id === songId ? { ...song, isLiked: !song.isLiked } : song
    );
    // Aquí conectarías con el backend
    console.log('Liked song:', songId);
  };

  const handleRepost = (songId: string) => {
    // Actualizar estado local
    const updatedData = mockFeedData.map(song => 
      song.id === songId ? { ...song, isReposted: !song.isReposted } : song
    );
    // Aquí conectarías con el backend
    console.log('Reposted song:', songId);
  };

  const handleShare = (songId: string) => {
    // Aquí conectarías con el backend
    console.log('Shared song:', songId);
  };

  const renderFeedItem = (song: any) => (
    <View key={song.id} style={styles.feedItem}>
      {/* Song Thumbnail */}
      <TouchableOpacity 
        style={styles.thumbnailContainer}
        onPress={() => handlePlayPause(song)}
      >
        <Image source={{ uri: song.imageUrl }} style={styles.thumbnail} />
        <View style={styles.playOverlay}>
          <Ionicons name="play" size={20} color="#FFFFFF" />
        </View>
      </TouchableOpacity>

      {/* Song Info */}
      <View style={styles.songInfo}>
        <View style={styles.songHeader}>
          <View style={styles.songTitleContainer}>
            <Text style={styles.songTitle} numberOfLines={1}>
              {song.title}
            </Text>
            <View style={styles.artistContainer}>
              <Text style={styles.artistName} numberOfLines={1}>
                {song.artist}
              </Text>
              {song.artistVerified && (
                <Ionicons name="checkmark-circle" size={16} color="#FFD93D" />
              )}
            </View>
          </View>
          <Text style={styles.duration}>{song.duration}</Text>
        </View>

        {/* Interaction Stats */}
        <View style={styles.interactionStats}>
          <View style={styles.statItem}>
            <Ionicons name="repeat" size={14} color="#94A3B8" />
            <Text style={styles.statText}>{song.reposts}</Text>
          </View>
          <View style={styles.statItem}>
            <Ionicons name="heart" size={14} color="#94A3B8" />
            <Text style={styles.statText}>{song.likes}</Text>
          </View>
          <View style={styles.statItem}>
            <Ionicons name="chatbubble" size={14} color="#94A3B8" />
            <Text style={styles.statText}>{song.comments}</Text>
          </View>
          <Text style={styles.playsText}>{song.plays} Plays</Text>
        </View>

        {/* Action Buttons */}
        <View style={styles.actionButtons}>
          <TouchableOpacity 
            style={styles.actionButton}
            onPress={() => handleRepost(song.id)}
          >
            <Ionicons 
              name="repeat" 
              size={20} 
              color={song.isReposted ? "#6C5CE7" : "#94A3B8"} 
            />
          </TouchableOpacity>
          
          <TouchableOpacity 
            style={styles.actionButton}
            onPress={() => handleLike(song.id)}
          >
            <Ionicons 
              name={song.isLiked ? "heart" : "heart-outline"} 
              size={20} 
              color={song.isLiked ? "#FF6B6B" : "#94A3B8"} 
            />
          </TouchableOpacity>
          
          <TouchableOpacity 
            style={styles.actionButton}
            onPress={() => handleShare(song.id)}
          >
            <Ionicons name="share-outline" size={20} color="#94A3B8" />
          </TouchableOpacity>
          
          <TouchableOpacity style={styles.actionButton}>
            <Ionicons name="ellipsis-horizontal" size={20} color="#94A3B8" />
          </TouchableOpacity>
        </View>
      </View>
    </View>
  );

  return (
    <SafeAreaView style={styles.container}>
      <StatusBar barStyle="light-content" backgroundColor="#0F0F1E" />
      
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
        
        <View style={styles.headerRight}>
          <TouchableOpacity style={styles.searchButton}>
            <Ionicons name="search" size={24} color="#FFFFFF" />
          </TouchableOpacity>
        </View>
      </View>

      {/* Feed Header */}
      <View style={styles.feedHeader}>
        <View style={styles.feedTitleContainer}>
          <Ionicons name="people" size={20} color="#6C5CE7" />
          <Text style={styles.feedTitle}>Your Feed</Text>
        </View>
        
        <TouchableOpacity style={styles.filterButton}>
          <Text style={styles.filterText}>{selectedFilter}</Text>
        </TouchableOpacity>
      </View>

      {/* Feed Content */}
      <ScrollView style={styles.feedContainer} showsVerticalScrollIndicator={false}>
        {mockFeedData.map(renderFeedItem)}
      </ScrollView>

      {/* Mini Player */}
      {currentSong && (
        <TouchableOpacity 
          style={styles.miniPlayer}
          onPress={() => navigation.navigate('MusicPlayer', { song: currentSong, user: { id: '1', name: 'User' } })}
        >
          <Image source={{ uri: currentSong.imageUrl }} style={styles.miniPlayerImage} />
          <View style={styles.miniPlayerInfo}>
            <Text style={styles.miniPlayerTitle} numberOfLines={1}>
              {currentSong.title}
            </Text>
            <Text style={styles.miniPlayerArtist} numberOfLines={1}>
              {currentSong.artist}
            </Text>
          </View>
          <TouchableOpacity 
            style={styles.miniPlayerButton}
            onPress={() => setIsPlaying(!isPlaying)}
          >
            <Ionicons 
              name={isPlaying ? "pause" : "play"} 
              size={24} 
              color="#FFFFFF" 
            />
          </TouchableOpacity>
        </TouchableOpacity>
      )}
    </SafeAreaView>
  );
};

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
    paddingVertical: 15,
    backgroundColor: '#FFFFFF',
    borderBottomWidth: 1,
    borderBottomColor: '#E5E7EB',
  },
  headerLeft: {
    flex: 1,
  },
  profileImage: {
    width: 32,
    height: 32,
    borderRadius: 16,
  },
  headerCenter: {
    flex: 2,
    alignItems: 'center',
  },
  logo: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#6C5CE7',
  },
  headerRight: {
    flex: 1,
    alignItems: 'flex-end',
  },
  searchButton: {
    padding: 5,
  },
  feedHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: 20,
    paddingVertical: 15,
    backgroundColor: '#FFFFFF',
  },
  feedTitleContainer: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  feedTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#6C5CE7',
    marginLeft: 8,
  },
  filterButton: {
    backgroundColor: '#6C5CE7',
    paddingHorizontal: 16,
    paddingVertical: 8,
    borderRadius: 20,
  },
  filterText: {
    color: '#FFFFFF',
    fontSize: 14,
    fontWeight: '600',
  },
  feedContainer: {
    flex: 1,
    backgroundColor: '#FFFFFF',
  },
  feedItem: {
    flexDirection: 'row',
    padding: 15,
    borderBottomWidth: 1,
    borderBottomColor: '#F3F4F6',
    backgroundColor: '#FFFFFF',
  },
  thumbnailContainer: {
    position: 'relative',
    marginRight: 15,
  },
  thumbnail: {
    width: 60,
    height: 60,
    borderRadius: 8,
  },
  playOverlay: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    backgroundColor: 'rgba(0,0,0,0.3)',
    borderRadius: 8,
    justifyContent: 'center',
    alignItems: 'center',
  },
  songInfo: {
    flex: 1,
  },
  songHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
    marginBottom: 8,
  },
  songTitleContainer: {
    flex: 1,
    marginRight: 10,
  },
  songTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#1F2937',
    marginBottom: 4,
  },
  artistContainer: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  artistName: {
    fontSize: 14,
    color: '#6B7280',
    marginRight: 4,
  },
  duration: {
    fontSize: 12,
    color: '#9CA3AF',
    fontWeight: '500',
  },
  interactionStats: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: 12,
  },
  statItem: {
    flexDirection: 'row',
    alignItems: 'center',
    marginRight: 15,
  },
  statText: {
    fontSize: 12,
    color: '#6B7280',
    marginLeft: 4,
  },
  playsText: {
    fontSize: 12,
    color: '#9CA3AF',
    marginLeft: 'auto',
  },
  actionButtons: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingRight: 20,
  },
  actionButton: {
    padding: 8,
  },
  miniPlayer: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#6C5CE7',
    paddingHorizontal: 20,
    paddingVertical: 12,
    borderTopWidth: 1,
    borderTopColor: '#E5E7EB',
  },
  miniPlayerImage: {
    width: 40,
    height: 40,
    borderRadius: 6,
    marginRight: 12,
  },
  miniPlayerInfo: {
    flex: 1,
  },
  miniPlayerTitle: {
    fontSize: 14,
    fontWeight: '600',
    color: '#FFFFFF',
    marginBottom: 2,
  },
  miniPlayerArtist: {
    fontSize: 12,
    color: '#E2E8F0',
  },
  miniPlayerButton: {
    padding: 8,
  },
}); 