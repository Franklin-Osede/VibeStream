import React, { useState, useEffect } from 'react';
import {
  View,
  Text,
  ScrollView,
  TouchableOpacity,
  StyleSheet,
  SafeAreaView,
  FlatList,
  Image,
} from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { LinearGradient } from 'expo-linear-gradient';

// Theme and localization
import { useTheme } from '../../theme';
import { useTranslation } from '../../localization/hooks/useTranslation';
import { LanguageSelector } from '../../localization/components/LanguageSelector';

// Mock data para demos
const MOCK_SONGS = [
  {
    id: '1',
    title: 'Electric Dreams',
    artist: 'Nova Waves',
    duration: '3:42',
    price: '0.05',
    cover: 'https://via.placeholder.com/80x80/6c5ce7/ffffff?text=ED',
    isPlaying: false,
  },
  {
    id: '2',
    title: 'Midnight Vibes',
    artist: 'Crypto Beats',
    duration: '4:15',
    price: '0.08',
    cover: 'https://via.placeholder.com/80x80/ff6b6b/ffffff?text=MV',
    isPlaying: false,
  },
  {
    id: '3',
    title: 'Digital Soul',
    artist: 'Blockchain Symphony',
    duration: '3:28',
    price: '0.12',
    cover: 'https://via.placeholder.com/80x80/00d4ff/ffffff?text=DS',
    isPlaying: true,
  },
  {
    id: '4',
    title: 'Future Bass',
    artist: 'Decentralized',
    duration: '5:03',
    price: '0.15',
    cover: 'https://via.placeholder.com/80x80/ffd93d/000000?text=FB',
    isPlaying: false,
  },
];

interface Song {
  id: string;
  title: string;
  artist: string;
  duration: string;
  price: string;
  cover: string;
  isPlaying: boolean;
}

export default function HomeScreen({ navigation, route }: any) {
  const { user } = route.params;
  const { t } = useTranslation();
  const theme = useTheme();
  const [showLanguageSelector, setShowLanguageSelector] = useState(false);
  const [songs, setSongs] = useState(MOCK_SONGS);
  const [userBalance] = useState('2.45'); // Mock balance

  const styles = createStyles(theme);

  // Función para reproducir/pausar música
  const togglePlayPause = (songId: string) => {
    setSongs(songs.map(song => ({
      ...song,
      isPlaying: song.id === songId ? !song.isPlaying : false
    })));
  };

  // Componente para cada canción
  const SongItem = ({ item }: { item: Song }) => (
    <TouchableOpacity style={styles.songItem}>
      <Image source={{ uri: item.cover }} style={styles.songCover} />
      
      <View style={styles.songInfo}>
        <Text style={styles.songTitle}>{item.title}</Text>
        <Text style={styles.songArtist}>{item.artist}</Text>
        <Text style={styles.songDuration}>{t('music.duration', { time: item.duration })}</Text>
      </View>
      
      <View style={styles.songActions}>
        <Text style={styles.songPrice}>{item.price} ETH</Text>
        <TouchableOpacity
          style={[styles.playButton, item.isPlaying && styles.playButtonActive]}
          onPress={() => togglePlayPause(item.id)}
        >
          <Text style={styles.playButtonText}>
            {item.isPlaying ? '⏸️' : '▶️'}
          </Text>
        </TouchableOpacity>
      </View>
    </TouchableOpacity>
  );

  return (
    <SafeAreaView style={styles.container}>
      <StatusBar style="light" />
      
      {/* Fondo con gradiente */}
      <LinearGradient
        colors={theme.gradients.background}
        style={styles.background}
        start={{ x: 0, y: 0 }}
        end={{ x: 1, y: 1 }}
      />

      {/* Header */}
      <View style={styles.header}>
        <View style={styles.headerLeft}>
          <Text style={styles.welcomeText}>{t('home.welcome')}</Text>
          <Text style={styles.username}>@{user.username}</Text>
        </View>
        
        <View style={styles.headerRight}>
          {/* Balance */}
          <LinearGradient
            colors={theme.gradients.gold}
            style={styles.balanceContainer}
            start={{ x: 0, y: 0 }}
            end={{ x: 1, y: 0 }}
          >
            <Text style={styles.balanceLabel}>{t('wallet.balance')}</Text>
            <Text style={styles.balanceAmount}>{userBalance} ETH</Text>
          </LinearGradient>
          
          {/* Configuración */}
          <TouchableOpacity
            style={styles.settingsButton}
            onPress={() => setShowLanguageSelector(true)}
          >
            <Text style={styles.settingsIcon}>⚙️</Text>
          </TouchableOpacity>
        </View>
      </View>

      {/* Contenido principal */}
      <ScrollView style={styles.content} showsVerticalScrollIndicator={false}>
        
        {/* Sección de música destacada */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>{t('home.featuredMusic')}</Text>
          
          <ScrollView horizontal showsHorizontalScrollIndicator={false}>
            <View style={styles.featuredContainer}>
              {songs.slice(0, 3).map((song) => (
                <TouchableOpacity key={song.id} style={styles.featuredItem}>
                  <LinearGradient
                    colors={[theme.primary, theme.colors.primaryLight]}
                    style={styles.featuredGradient}
                    start={{ x: 0, y: 0 }}
                    end={{ x: 1, y: 1 }}
                  >
                    <Image source={{ uri: song.cover }} style={styles.featuredCover} />
                    <Text style={styles.featuredTitle}>{song.title}</Text>
                    <Text style={styles.featuredArtist}>{song.artist}</Text>
                  </LinearGradient>
                </TouchableOpacity>
              ))}
            </View>
          </ScrollView>
        </View>

        {/* Sección de tendencias */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>{t('home.trending')}</Text>
          
          <FlatList
            data={songs}
            renderItem={SongItem}
            keyExtractor={(item) => item.id}
            scrollEnabled={false}
            ItemSeparatorComponent={() => <View style={styles.separator} />}
          />
        </View>

        {/* Navegación inferior simulada */}
        <View style={styles.bottomNavigation}>
          <LinearGradient
            colors={theme.gradients.surface}
            style={styles.navGradient}
            start={{ x: 0, y: 0 }}
            end={{ x: 1, y: 0 }}
          >
            <TouchableOpacity style={[styles.navItem, styles.navItemActive]}>
              <Text style={styles.navIcon}>🏠</Text>
              <Text style={styles.navText}>{t('navigation.home')}</Text>
            </TouchableOpacity>
            
            <TouchableOpacity style={styles.navItem}>
              <Text style={styles.navIcon}>🎵</Text>
              <Text style={styles.navText}>{t('navigation.music')}</Text>
            </TouchableOpacity>
            
            <TouchableOpacity style={styles.navItem}>
              <Text style={styles.navIcon}>💰</Text>
              <Text style={styles.navText}>{t('navigation.wallet')}</Text>
            </TouchableOpacity>
            
            <TouchableOpacity style={styles.navItem}>
              <Text style={styles.navIcon}>👤</Text>
              <Text style={styles.navText}>{t('navigation.profile')}</Text>
            </TouchableOpacity>
          </LinearGradient>
        </View>

      </ScrollView>

      {/* Selector de idioma */}
      <LanguageSelector
        visible={showLanguageSelector}
        onClose={() => setShowLanguageSelector(false)}
        onLanguageChange={() => {
          console.log('Language changed in HomeScreen');
        }}
      />
    </SafeAreaView>
  );
}

// Función para crear estilos usando el tema
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
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
    paddingHorizontal: theme.spacing.lg,
    paddingTop: theme.spacing.lg,
    paddingBottom: theme.spacing.md,
  },
  headerLeft: {
    flex: 1,
  },
  welcomeText: {
    ...theme.styles.textSecondary,
    fontSize: 16,
  },
  username: {
    ...theme.styles.titleMedium,
    marginTop: theme.spacing.xs,
  },
  headerRight: {
    alignItems: 'flex-end',
    gap: theme.spacing.sm,
  },
  balanceContainer: {
    paddingHorizontal: theme.spacing.md,
    paddingVertical: theme.spacing.sm,
    borderRadius: theme.borderRadius.lg,
    alignItems: 'center',
    minWidth: 100,
  },
  balanceLabel: {
    fontSize: 12,
    color: theme.text,
    fontWeight: '500',
  },
  balanceAmount: {
    fontSize: 16,
    color: theme.text,
    fontWeight: '700',
    marginTop: 2,
  },
  settingsButton: {
    width: 40,
    height: 40,
    borderRadius: 20,
    backgroundColor: theme.colors.glassLight,
    justifyContent: 'center',
    alignItems: 'center',
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
  },
  settingsIcon: {
    fontSize: 20,
  },
  content: {
    flex: 1,
    paddingHorizontal: theme.spacing.lg,
  },
  section: {
    marginBottom: theme.spacing.xl,
  },
  sectionTitle: {
    ...theme.styles.titleSmall,
    marginBottom: theme.spacing.md,
  },
  featuredContainer: {
    flexDirection: 'row',
    gap: theme.spacing.md,
  },
  featuredItem: {
    width: 140,
  },
  featuredGradient: {
    borderRadius: theme.borderRadius.lg,
    padding: theme.spacing.md,
    alignItems: 'center',
    ...theme.shadows.md,
  },
  featuredCover: {
    width: 60,
    height: 60,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.sm,
  },
  featuredTitle: {
    ...theme.styles.textPrimary,
    fontSize: 14,
    fontWeight: '600',
    textAlign: 'center',
  },
  featuredArtist: {
    ...theme.styles.textSecondary,
    fontSize: 12,
    textAlign: 'center',
    marginTop: 2,
  },
  songItem: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: theme.colors.glassLight,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.lg,
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
  },
  songCover: {
    width: 60,
    height: 60,
    borderRadius: theme.borderRadius.md,
    marginRight: theme.spacing.md,
  },
  songInfo: {
    flex: 1,
  },
  songTitle: {
    ...theme.styles.textPrimary,
    fontSize: 16,
    fontWeight: '600',
  },
  songArtist: {
    ...theme.styles.textSecondary,
    fontSize: 14,
    marginTop: 2,
  },
  songDuration: {
    ...theme.styles.textMuted,
    fontSize: 12,
    marginTop: 2,
  },
  songActions: {
    alignItems: 'flex-end',
    gap: theme.spacing.sm,
  },
  songPrice: {
    color: theme.colors.accentGold,
    fontSize: 14,
    fontWeight: '600',
  },
  playButton: {
    width: 40,
    height: 40,
    borderRadius: 20,
    backgroundColor: theme.accent,
    justifyContent: 'center',
    alignItems: 'center',
    ...theme.shadows.md,
  },
  playButtonActive: {
    backgroundColor: theme.colors.success,
  },
  playButtonText: {
    fontSize: 16,
  },
  separator: {
    height: theme.spacing.md,
  },
  bottomNavigation: {
    marginTop: theme.spacing.xl,
    marginBottom: theme.spacing.lg,
  },
  navGradient: {
    flexDirection: 'row',
    borderRadius: theme.borderRadius.xl,
    padding: theme.spacing.sm,
    ...theme.shadows.lg,
  },
  navItem: {
    flex: 1,
    alignItems: 'center',
    paddingVertical: theme.spacing.sm,
    borderRadius: theme.borderRadius.lg,
  },
  navItemActive: {
    backgroundColor: theme.colors.glassLight,
  },
  navIcon: {
    fontSize: 20,
    marginBottom: 4,
  },
  navText: {
    ...theme.styles.textMuted,
    fontSize: 12,
    fontWeight: '500',
  },
}); 