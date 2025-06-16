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
    hasActiveCampaign: true,
  },
  {
    id: '2',
    title: 'Midnight Vibes',
    artist: 'Crypto Beats',
    duration: '4:15',
    price: '0.08',
    cover: 'https://via.placeholder.com/80x80/ff6b6b/ffffff?text=MV',
    isPlaying: false,
    hasActiveCampaign: false,
  },
  {
    id: '3',
    title: 'Digital Soul',
    artist: 'Blockchain Symphony',
    duration: '3:28',
    price: '0.12',
    cover: 'https://via.placeholder.com/80x80/00d4ff/ffffff?text=DS',
    isPlaying: true,
    hasActiveCampaign: true,
  },
  {
    id: '4',
    title: 'Future Bass',
    artist: 'Decentralized',
    duration: '5:03',
    price: '0.15',
    cover: 'https://via.placeholder.com/80x80/ffd93d/000000?text=FB',
    isPlaying: false,
    hasActiveCampaign: false,
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
  hasActiveCampaign?: boolean;
}

export default function HomeScreen({ navigation, route }: any) {
  const { user } = route.params;
  const { t } = useTranslation();
  const theme = useTheme();
  const [showLanguageSelector, setShowLanguageSelector] = useState(false);
  const [songs, setSongs] = useState(MOCK_SONGS);
  
  // Estados para earnings tracking
  const [userBalance] = useState('12.47'); // Mock balance en USD
  const [todayEarnings] = useState('2.34');
  const [weekEarnings] = useState('18.92');
  const [totalListenTime] = useState('47'); // horas esta semana

  const styles = createStyles(theme);

  // Función para reproducir/pausar música
  const togglePlayPause = (songId: string) => {
    setSongs(songs.map(song => ({
      ...song,
      isPlaying: song.id === songId ? !song.isPlaying : false
    })));
  };

  // Componente para cada canción CON earnings
  const SongItem = ({ item }: { item: Song }) => (
    <TouchableOpacity style={styles.songItem}>
      <Image source={{ uri: item.cover }} style={styles.songCover} />
      
      <View style={styles.songInfo}>
        <Text style={styles.songTitle}>{item.title}</Text>
        <Text style={styles.songArtist}>{item.artist}</Text>
        <Text style={styles.songDuration}>Duración: {item.duration}</Text>
        
        {/* EARNINGS DISPLAY - Elemento diferenciador */}
        <View style={styles.earningsRow}>
          <Text style={styles.earningsText}>💰 Ganas: $0.02</Text>
          {item.hasActiveCampaign && (
            <View style={styles.boostBadge}>
              <Text style={styles.boostText}>🚀 2x BOOST</Text>
            </View>
          )}
        </View>
      </View>
      
      <View style={styles.songActions}>
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

      {/* Header CON EARNINGS PROMINENTES */}
      <View style={styles.header}>
        <View style={styles.headerLeft}>
          <Text style={styles.welcomeText}>¡Bienvenido!</Text>
          <Text style={styles.username}>@{user.username}</Text>
          
          {/* Stats rápidas */}
          <View style={styles.quickStats}>
            <Text style={styles.quickStatsText}>🎵 {totalListenTime}h esta semana</Text>
          </View>
        </View>
        
        <View style={styles.headerRight}>
          {/* Earnings Balance - MUY PROMINENTE */}
          <LinearGradient
            colors={theme.gradients.gold}
            style={styles.earningsContainer}
            start={{ x: 0, y: 0 }}
            end={{ x: 1, y: 0 }}
          >
            <Text style={styles.earningsLabel}>💰 Disponible</Text>
            <Text style={styles.earningsAmount}>${userBalance}</Text>
            <TouchableOpacity style={styles.withdrawButton}>
              <Text style={styles.withdrawText}>Retirar</Text>
            </TouchableOpacity>
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

      {/* NUEVA SECCIÓN: Daily Earnings Tracker */}
      <View style={styles.dailyEarningsSection}>
        <LinearGradient
          colors={[theme.colors.success + '20', theme.colors.success + '10']}
          style={styles.dailyEarningsCard}
          start={{ x: 0, y: 0 }}
          end={{ x: 1, y: 0 }}
        >
          <View style={styles.earningsRow}>
            <View style={styles.earningsColumn}>
              <Text style={styles.earningsTitle}>Hoy</Text>
              <Text style={styles.earningsValue}>+${todayEarnings}</Text>
            </View>
            <View style={styles.earningsColumn}>
              <Text style={styles.earningsTitle}>Esta semana</Text>
              <Text style={styles.earningsValue}>+${weekEarnings}</Text>
            </View>
            <TouchableOpacity style={styles.earningsButton}>
              <Text style={styles.earningsButtonText}>Ver detalles 📊</Text>
            </TouchableOpacity>
          </View>
          
          <Text style={styles.motivationalText}>
            ¡Sigue escuchando para ganar más! 🎯
          </Text>
        </LinearGradient>
      </View>

      {/* Contenido principal */}
      <ScrollView style={styles.content} showsVerticalScrollIndicator={false}>
        
        {/* Sección de música que MÁS PAGA */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>🔥 Gana más escuchando</Text>
            <Text style={styles.sectionSubtitle}>Canciones con boost activo</Text>
          </View>
          
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
                    
                    {/* EARNINGS BADGE */}
                    <View style={styles.earningsBadge}>
                      <Text style={styles.earningsBadgeText}>💰 $0.04</Text>
                    </View>
                  </LinearGradient>
                </TouchableOpacity>
              ))}
            </View>
          </ScrollView>
        </View>

        {/* Sección de tendencias CON EARNINGS */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Tendencias</Text>
          
          <FlatList
            data={songs}
            renderItem={SongItem}
            keyExtractor={(item) => item.id}
            scrollEnabled={false}
            ItemSeparatorComponent={() => <View style={styles.separator} />}
          />
        </View>

        {/* NUEVA SECCIÓN: Motivational Call-to-Action */}
        <View style={styles.section}>
          <LinearGradient
            colors={[theme.primary + '15', theme.accent + '15']}
            style={styles.motivationalCard}
            start={{ x: 0, y: 0 }}
            end={{ x: 1, y: 1 }}
          >
            <Text style={styles.motivationalTitle}>🎯 Desafío semanal</Text>
            <Text style={styles.motivationalDescription}>
              Escucha 10 horas más esta semana y gana $5 extra
            </Text>
            <View style={styles.progressContainer}>
              <Text style={styles.progressText}>Progreso: {totalListenTime}/57 horas</Text>
              <View style={styles.progressBar}>
                <View style={[styles.progressFill, { width: `${(parseInt(totalListenTime) / 57) * 100}%` }]} />
              </View>
            </View>
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
  earningsText: {
    color: theme.colors.success,
    fontSize: 12,
    fontWeight: '600',
  },
  boostBadge: {
    backgroundColor: theme.colors.accentPink,
    paddingHorizontal: theme.spacing.sm,
    paddingVertical: 2,
    borderRadius: theme.borderRadius.sm,
    marginLeft: theme.spacing.sm,
  },
  boostText: {
    color: theme.text,
    fontSize: 10,
    fontWeight: '700',
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
  quickStats: {
    marginTop: theme.spacing.xs,
  },
  quickStatsText: {
    ...theme.styles.textSecondary,
    fontSize: 14,
  },
  earningsContainer: {
    paddingHorizontal: theme.spacing.md,
    paddingVertical: theme.spacing.sm,
    borderRadius: theme.borderRadius.lg,
    alignItems: 'center',
    minWidth: 100,
  },
  earningsLabel: {
    fontSize: 12,
    color: theme.text,
    fontWeight: '500',
  },
  earningsAmount: {
    fontSize: 16,
    color: theme.text,
    fontWeight: '700',
    marginTop: 2,
  },
  withdrawButton: {
    padding: theme.spacing.sm,
    borderRadius: theme.borderRadius.md,
    backgroundColor: theme.colors.glassLight,
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
  },
  withdrawText: {
    ...theme.styles.textSecondary,
    fontSize: 14,
  },
  dailyEarningsSection: {
    marginBottom: theme.spacing.xl,
  },
  dailyEarningsCard: {
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.lg,
    ...theme.shadows.lg,
  },
  earningsRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: theme.spacing.sm,
  },
  earningsColumn: {
    flex: 1,
    alignItems: 'center',
  },
  earningsTitle: {
    ...theme.styles.textSecondary,
    fontSize: 14,
    fontWeight: '500',
  },
  earningsValue: {
    ...theme.styles.titleMedium,
    fontSize: 16,
    fontWeight: '700',
  },
  earningsButton: {
    padding: theme.spacing.sm,
    borderRadius: theme.borderRadius.md,
    backgroundColor: theme.colors.glassLight,
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
  },
  earningsButtonText: {
    ...theme.styles.textSecondary,
    fontSize: 14,
  },
  motivationalText: {
    ...theme.styles.textSecondary,
    fontSize: 14,
    textAlign: 'center',
  },
  motivationalCard: {
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.lg,
    ...theme.shadows.lg,
  },
  motivationalTitle: {
    ...theme.styles.titleSmall,
    marginBottom: theme.spacing.sm,
  },
  motivationalDescription: {
    ...theme.styles.textSecondary,
    fontSize: 14,
  },
  progressContainer: {
    marginBottom: theme.spacing.sm,
  },
  progressText: {
    ...theme.styles.textSecondary,
    fontSize: 14,
  },
  progressBar: {
    height: 10,
    backgroundColor: theme.colors.glassLight,
    borderRadius: 5,
    overflow: 'hidden',
  },
  progressFill: {
    height: '100%',
    backgroundColor: theme.colors.success,
  },
  sectionHeader: {
    marginBottom: theme.spacing.md,
  },
  sectionSubtitle: {
    ...theme.styles.textSecondary,
    fontSize: 14,
  },
  earningsBadge: {
    padding: theme.spacing.xs,
    borderRadius: theme.borderRadius.md,
    backgroundColor: theme.colors.success,
    marginTop: theme.spacing.sm,
  },
  earningsBadgeText: {
    ...theme.styles.textPrimary,
    fontSize: 12,
    fontWeight: '600',
  },
}); 