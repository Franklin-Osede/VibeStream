import React, { useState, useEffect } from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  StyleSheet,
  ScrollView,
  FlatList,
  Image,
  Dimensions,
} from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { LinearGradient } from 'expo-linear-gradient';
import { useTheme } from '../../theme';

const { width } = Dimensions.get('window');

interface MusicExploreScreenProps {
  navigation: any;
  route: {
    params: {
      user: any;
      token: string;
    };
  };
}

interface Track {
  id: string;
  title: string;
  artist: string;
  duration: string;
  image: string;
  campaign?: {
    multiplier: number;
    active: boolean;
  };
}

interface Artist {
  id: string;
  name: string;
  image: string;
  followers: string;
}

const MusicExploreScreen: React.FC<MusicExploreScreenProps> = ({ 
  navigation, 
  route 
}) => {
  const [recentlyPlayed, setRecentlyPlayed] = useState<Track[]>([]);
  const [trendingTracks, setTrendingTracks] = useState<Track[]>([]);
  const [featuredArtists, setFeaturedArtists] = useState<Artist[]>([]);
  const [earnedToday, setEarnedToday] = useState(0);
  const theme = useTheme();

  useEffect(() => {
    loadMusicData();
  }, []);

  const loadMusicData = async () => {
    // Simular datos de música
    setRecentlyPlayed([
      {
        id: '1',
        title: 'Midnight Groove',
        artist: 'ElectroVibes',
        duration: '3:45',
        image: 'https://picsum.photos/200/200?random=1'
      },
      {
        id: '2',
        title: 'Neon Dreams',
        artist: 'SynthWave',
        duration: '4:12',
        image: 'https://picsum.photos/200/200?random=2',
        campaign: {
          multiplier: 2.5,
          active: true
        }
      }
    ]);

    setTrendingTracks([
      {
        id: '3',
        title: 'Summer Beats',
        artist: 'TropicalHouse',
        duration: '3:28',
        image: 'https://picsum.photos/200/200?random=3',
        campaign: {
          multiplier: 1.8,
          active: true
        }
      },
      {
        id: '4',
        title: 'Electric Dreams',
        artist: 'FutureBase',
        duration: '5:01',
        image: 'https://picsum.photos/200/200?random=4'
      }
    ]);

    setFeaturedArtists([
      {
        id: '1',
        name: 'ElectroVibes',
        image: 'https://picsum.photos/300/300?random=10',
        followers: '125K'
      },
      {
        id: '2',
        name: 'SynthWave',
        image: 'https://picsum.photos/300/300?random=11',
        followers: '89K'
      }
    ]);

    setEarnedToday(12.5);
  };

  const handlePlayTrack = (track: Track) => {
    navigation.navigate('MusicPlayer', {
      track,
      user: route.params.user,
      token: route.params.token
    });
  };

  const handleArtistPress = (artist: Artist) => {
    navigation.navigate('ArtistProfile', {
      artist,
      user: route.params.user,
      token: route.params.token
    });
  };

  const renderTrackItem = ({ item: track }: { item: Track }) => (
    <TouchableOpacity 
      style={styles.trackItem} 
      onPress={() => handlePlayTrack(track)}
    >
      <Image source={{ uri: track.image }} style={styles.trackImage} />
      <View style={styles.trackInfo}>
        <Text style={styles.trackTitle} numberOfLines={1}>
          {track.title}
        </Text>
        <Text style={styles.trackArtist} numberOfLines={1}>
          {track.artist}
        </Text>
        <Text style={styles.trackDuration}>{track.duration}</Text>
      </View>
      
      {track.campaign?.active && (
        <View style={styles.campaignBadge}>
          <Text style={styles.campaignText}>×{track.campaign.multiplier}</Text>
        </View>
      )}
      
      <TouchableOpacity style={styles.playButton}>
        <Text style={styles.playIcon}>▶️</Text>
      </TouchableOpacity>
    </TouchableOpacity>
  );

  const renderArtistItem = ({ item: artist }: { item: Artist }) => (
    <TouchableOpacity 
      style={styles.artistItem} 
      onPress={() => handleArtistPress(artist)}
    >
      <Image source={{ uri: artist.image }} style={styles.artistImage} />
      <Text style={styles.artistName} numberOfLines={1}>
        {artist.name}
      </Text>
      <Text style={styles.artistFollowers}>
        {artist.followers} seguidores
      </Text>
    </TouchableOpacity>
  );

  const styles = createStyles(theme);

  return (
    <View style={styles.container}>
      <StatusBar style="light" />
      
      <LinearGradient
        colors={theme.gradients.background}
        style={styles.background}
        start={{ x: 0, y: 0 }}
        end={{ x: 1, y: 1 }}
      />

      <ScrollView contentContainerStyle={styles.scrollContent}>
        {/* Header */}
        <View style={styles.header}>
          <View style={styles.greetingContainer}>
            <Text style={styles.greeting}>
              ¡Hola, {route.params.user.username}!
            </Text>
            <Text style={styles.subtitle}>Descubre música y gana $VIBERS</Text>
          </View>
          
          <TouchableOpacity style={styles.searchButton}>
            <Text style={styles.searchIcon}>🔍</Text>
          </TouchableOpacity>
        </View>

        {/* Today's Earnings */}
        <View style={styles.earningsCard}>
          <LinearGradient
            colors={[theme.primary, theme.colors.primaryLight]}
            style={styles.earningsGradient}
            start={{ x: 0, y: 0 }}
            end={{ x: 1, y: 1 }}
          >
            <Text style={styles.earningsLabel}>Ganado Hoy</Text>
            <Text style={styles.earningsAmount}>+{earnedToday} $VIBERS</Text>
            <Text style={styles.earningsDescription}>
              🎧 Sigue escuchando para ganar más
            </Text>
          </LinearGradient>
        </View>

        {/* Quick Actions */}
        <View style={styles.quickActions}>
          <TouchableOpacity style={styles.actionButton}>
            <LinearGradient
              colors={[theme.colors.glassLight, theme.colors.glassMedium]}
              style={styles.actionGradient}
            >
              <Text style={styles.actionIcon}>🎵</Text>
              <Text style={styles.actionText}>Mis Playlists</Text>
            </LinearGradient>
          </TouchableOpacity>
          
          <TouchableOpacity style={styles.actionButton}>
            <LinearGradient
              colors={[theme.colors.glassLight, theme.colors.glassMedium]}
              style={styles.actionGradient}
            >
              <Text style={styles.actionIcon}>🔥</Text>
              <Text style={styles.actionText}>Campañas</Text>
            </LinearGradient>
          </TouchableOpacity>
          
          <TouchableOpacity style={styles.actionButton}>
            <LinearGradient
              colors={[theme.colors.glassLight, theme.colors.glassMedium]}
              style={styles.actionGradient}
            >
              <Text style={styles.actionIcon}>💎</Text>
              <Text style={styles.actionText}>NFTs</Text>
            </LinearGradient>
          </TouchableOpacity>
        </View>

        {/* Recently Played */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>Escuchado Recientemente</Text>
            <TouchableOpacity>
              <Text style={styles.seeAllText}>Ver todo</Text>
            </TouchableOpacity>
          </View>
          
          <FlatList
            data={recentlyPlayed}
            renderItem={renderTrackItem}
            keyExtractor={(item) => item.id}
            horizontal
            showsHorizontalScrollIndicator={false}
            contentContainerStyle={styles.horizontalList}
          />
        </View>

        {/* Featured Artists */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>Artistas Destacados</Text>
            <TouchableOpacity>
              <Text style={styles.seeAllText}>Ver todo</Text>
            </TouchableOpacity>
          </View>
          
          <FlatList
            data={featuredArtists}
            renderItem={renderArtistItem}
            keyExtractor={(item) => item.id}
            horizontal
            showsHorizontalScrollIndicator={false}
            contentContainerStyle={styles.horizontalList}
          />
        </View>

        {/* Trending Tracks with Campaigns */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>🚀 Tendencias con Campañas</Text>
            <TouchableOpacity>
              <Text style={styles.seeAllText}>Ver todo</Text>
            </TouchableOpacity>
          </View>
          
          {trendingTracks.map((track) => (
            <TouchableOpacity 
              key={track.id}
              style={styles.trendingTrack}
              onPress={() => handlePlayTrack(track)}
            >
              <LinearGradient
                colors={
                  track.campaign?.active 
                    ? [theme.colors.accentPink, theme.colors.primaryLight]
                    : [theme.colors.glassLight, theme.colors.glassMedium]
                }
                style={styles.trendingGradient}
              >
                <Image source={{ uri: track.image }} style={styles.trendingImage} />
                <View style={styles.trendingInfo}>
                  <Text style={styles.trendingTitle}>{track.title}</Text>
                  <Text style={styles.trendingArtist}>{track.artist}</Text>
                  {track.campaign?.active && (
                    <Text style={styles.trendingCampaign}>
                      🔥 ×{track.campaign.multiplier} multiplicador activo
                    </Text>
                  )}
                </View>
                <TouchableOpacity style={styles.trendingPlayButton}>
                  <Text style={styles.playIcon}>▶️</Text>
                </TouchableOpacity>
              </LinearGradient>
            </TouchableOpacity>
          ))}
        </View>
      </ScrollView>
    </View>
  );
};

const createStyles = (theme: ReturnType<typeof useTheme>) => StyleSheet.create({
  container: {
    flex: 1,
  },
  background: {
    position: 'absolute',
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
  },
  scrollContent: {
    padding: theme.spacing.lg,
    paddingTop: theme.spacing.xxl + theme.spacing.lg,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: theme.spacing.xl,
  },
  greetingContainer: {
    flex: 1,
  },
  greeting: {
    ...theme.styles.titleLarge,
    color: theme.text,
    marginBottom: theme.spacing.sm,
  },
  subtitle: {
    fontSize: 14,
    color: theme.textSecondary,
  },
  searchButton: {
    backgroundColor: theme.colors.glassLight,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
  },
  searchIcon: {
    fontSize: 20,
  },
  earningsCard: {
    borderRadius: theme.borderRadius.xl,
    marginBottom: theme.spacing.xl,
    ...theme.shadows.lg,
  },
  earningsGradient: {
    padding: theme.spacing.xl,
    borderRadius: theme.borderRadius.xl,
    alignItems: 'center',
  },
  earningsLabel: {
    fontSize: 14,
    color: theme.text,
    opacity: 0.8,
    marginBottom: theme.spacing.sm,
  },
  earningsAmount: {
    fontSize: 28,
    color: theme.text,
    fontWeight: 'bold',
    marginBottom: theme.spacing.sm,
  },
  earningsDescription: {
    fontSize: 12,
    color: theme.text,
    opacity: 0.7,
  },
  quickActions: {
    flexDirection: 'row',
    gap: theme.spacing.md,
    marginBottom: theme.spacing.xl,
  },
  actionButton: {
    flex: 1,
    borderRadius: theme.borderRadius.md,
    ...theme.shadows.sm,
  },
  actionGradient: {
    padding: theme.spacing.lg,
    borderRadius: theme.borderRadius.md,
    alignItems: 'center',
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
  },
  actionIcon: {
    fontSize: 24,
    marginBottom: theme.spacing.sm,
  },
  actionText: {
    fontSize: 12,
    color: theme.text,
    fontWeight: '600',
  },
  section: {
    marginBottom: theme.spacing.xl,
  },
  sectionHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: theme.spacing.lg,
  },
  sectionTitle: {
    ...theme.styles.titleMedium,
    color: theme.text,
    fontWeight: 'bold',
  },
  seeAllText: {
    fontSize: 14,
    color: theme.primary,
    fontWeight: '600',
  },
  horizontalList: {
    paddingRight: theme.spacing.lg,
  },
  trackItem: {
    width: 160,
    marginRight: theme.spacing.md,
    backgroundColor: theme.colors.glassLight,
    borderRadius: theme.borderRadius.md,
    padding: theme.spacing.md,
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
  },
  trackImage: {
    width: '100%',
    height: 120,
    borderRadius: theme.borderRadius.sm,
    marginBottom: theme.spacing.md,
  },
  trackInfo: {
    flex: 1,
    marginBottom: theme.spacing.md,
  },
  trackTitle: {
    fontSize: 14,
    color: theme.text,
    fontWeight: 'bold',
    marginBottom: theme.spacing.xs,
  },
  trackArtist: {
    fontSize: 12,
    color: theme.textSecondary,
    marginBottom: theme.spacing.xs,
  },
  trackDuration: {
    fontSize: 10,
    color: theme.textMuted,
  },
  campaignBadge: {
    position: 'absolute',
    top: theme.spacing.md,
    right: theme.spacing.md,
    backgroundColor: theme.colors.accentPink,
    paddingHorizontal: theme.spacing.sm,
    paddingVertical: theme.spacing.xs,
    borderRadius: theme.borderRadius.sm,
  },
  campaignText: {
    fontSize: 10,
    color: theme.text,
    fontWeight: 'bold',
  },
  playButton: {
    alignSelf: 'center',
    backgroundColor: theme.primary,
    width: 32,
    height: 32,
    borderRadius: 16,
    justifyContent: 'center',
    alignItems: 'center',
  },
  playIcon: {
    fontSize: 12,
  },
  artistItem: {
    width: 120,
    marginRight: theme.spacing.md,
    alignItems: 'center',
  },
  artistImage: {
    width: 80,
    height: 80,
    borderRadius: 40,
    marginBottom: theme.spacing.md,
  },
  artistName: {
    fontSize: 14,
    color: theme.text,
    fontWeight: 'bold',
    marginBottom: theme.spacing.xs,
    textAlign: 'center',
  },
  artistFollowers: {
    fontSize: 10,
    color: theme.textMuted,
    textAlign: 'center',
  },
  trendingTrack: {
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.md,
    ...theme.shadows.sm,
  },
  trendingGradient: {
    flexDirection: 'row',
    padding: theme.spacing.lg,
    borderRadius: theme.borderRadius.md,
    alignItems: 'center',
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
  },
  trendingImage: {
    width: 60,
    height: 60,
    borderRadius: theme.borderRadius.sm,
    marginRight: theme.spacing.lg,
  },
  trendingInfo: {
    flex: 1,
  },
  trendingTitle: {
    fontSize: 16,
    color: theme.text,
    fontWeight: 'bold',
    marginBottom: theme.spacing.xs,
  },
  trendingArtist: {
    fontSize: 14,
    color: theme.text,
    opacity: 0.8,
    marginBottom: theme.spacing.xs,
  },
  trendingCampaign: {
    fontSize: 12,
    color: theme.text,
    fontWeight: 'bold',
  },
  trendingPlayButton: {
    backgroundColor: theme.text,
    width: 40,
    height: 40,
    borderRadius: 20,
    justifyContent: 'center',
    alignItems: 'center',
  },
});

export default MusicExploreScreen; 