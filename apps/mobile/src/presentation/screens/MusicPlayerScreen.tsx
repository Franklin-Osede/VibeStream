import React, { useState, useEffect, useRef } from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  StyleSheet,
  Image,
  Dimensions,
  Animated,
  Alert,
} from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { LinearGradient } from 'expo-linear-gradient';
import { useTheme } from '../../theme';

const { width, height } = Dimensions.get('window');

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

interface MusicPlayerScreenProps {
  navigation: any;
  route: {
    params: {
      track: Track;
      user: any;
      token: string;
    };
  };
}

const MusicPlayerScreen: React.FC<MusicPlayerScreenProps> = ({ 
  navigation, 
  route 
}) => {
  const { track, user } = route.params;
  
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [totalTime, setTotalTime] = useState(245); // 4:05 en segundos
  const [earnedFromTrack, setEarnedFromTrack] = useState(0);
  const [listenSessionId, setListenSessionId] = useState<string | null>(null);
  const [zkProofGenerated, setZkProofGenerated] = useState(false);
  
  const theme = useTheme();
  const progressAnim = useRef(new Animated.Value(0)).current;
  const heartBeatAnim = useRef(new Animated.Value(1)).current;
  const pulseAnim = useRef(new Animated.Value(0)).current;

  useEffect(() => {
    if (isPlaying) {
      startListenSession();
      const interval = setInterval(() => {
        setCurrentTime(prev => {
          if (prev >= totalTime) {
            setIsPlaying(false);
            endListenSession();
            return totalTime;
          }
          
          // Simular ganancias cada 30 segundos
          if (prev % 30 === 0 && prev > 0) {
            const baseEarning = 0.5;
            const multiplier = track.campaign?.active ? track.campaign.multiplier : 1;
            const earned = baseEarning * multiplier;
            setEarnedFromTrack(prevEarned => prevEarned + earned);
            
            // Animate earnings
            Animated.sequence([
              Animated.timing(pulseAnim, {
                toValue: 1,
                duration: 300,
                useNativeDriver: true,
              }),
              Animated.timing(pulseAnim, {
                toValue: 0,
                duration: 300,
                useNativeDriver: true,
              }),
            ]).start();
          }
          
          return prev + 1;
        });
      }, 1000);

      return () => clearInterval(interval);
    }
  }, [isPlaying]);

  useEffect(() => {
    // Animar progreso
    Animated.timing(progressAnim, {
      toValue: currentTime / totalTime,
      duration: 100,
      useNativeDriver: false,
    }).start();
  }, [currentTime]);

  const startListenSession = async () => {
    try {
      // Simular inicio de sesión de escucha
      const sessionId = `session_${Date.now()}`;
      setListenSessionId(sessionId);
      
      console.log('Listen session started:', sessionId);
      
      // Simular heartbeat animation
      Animated.loop(
        Animated.sequence([
          Animated.timing(heartBeatAnim, {
            toValue: 1.1,
            duration: 1000,
            useNativeDriver: true,
          }),
          Animated.timing(heartBeatAnim, {
            toValue: 1,
            duration: 1000,
            useNativeDriver: true,
          }),
        ])
      ).start();
      
    } catch (error) {
      console.error('Error starting listen session:', error);
    }
  };

  const endListenSession = async () => {
    if (!listenSessionId) return;
    
    try {
      // Simular generación de prueba ZK
      setZkProofGenerated(true);
      
      setTimeout(() => {
        Alert.alert(
          '🎉 Sesión Completada',
          `Has ganado ${earnedFromTrack.toFixed(2)} $VIBERS!\n\nPrueba ZK generada y verificada ✅`,
          [
            {
              text: 'Genial!',
              onPress: () => setZkProofGenerated(false)
            }
          ]
        );
      }, 1000);
      
      console.log('Listen session ended:', listenSessionId);
      setListenSessionId(null);
      
    } catch (error) {
      console.error('Error ending listen session:', error);
    }
  };

  const handlePlayPause = () => {
    setIsPlaying(!isPlaying);
  };

  const handleSeek = (position: number) => {
    setCurrentTime(Math.floor(position * totalTime));
  };

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

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

      {/* Header */}
      <View style={styles.header}>
        <TouchableOpacity 
          style={styles.backButton}
          onPress={() => navigation.goBack()}
        >
          <Text style={styles.backIcon}>←</Text>
        </TouchableOpacity>
        
        <Text style={styles.headerTitle}>Reproduciendo</Text>
        
        <TouchableOpacity style={styles.moreButton}>
          <Text style={styles.moreIcon}>⋯</Text>
        </TouchableOpacity>
      </View>

      {/* Album Art */}
      <View style={styles.albumContainer}>
        <Animated.View style={[
          styles.albumImageContainer,
          {
            transform: [{ scale: heartBeatAnim }]
          }
        ]}>
          <Image source={{ uri: track.image }} style={styles.albumImage} />
          
          {/* Campaign Badge */}
          {track.campaign?.active && (
            <View style={styles.campaignOverlay}>
              <LinearGradient
                colors={[theme.colors.accentPink, theme.colors.primaryLight]}
                style={styles.campaignBadge}
              >
                <Text style={styles.campaignText}>
                  ×{track.campaign.multiplier}
                </Text>
              </LinearGradient>
            </View>
          )}
          
          {/* Listen-to-Earn Indicator */}
          {isPlaying && (
            <View style={styles.earningIndicator}>
              <Animated.View style={[
                styles.earningPulse,
                {
                  opacity: pulseAnim,
                  transform: [{ scale: pulseAnim }]
                }
              ]}>
                <Text style={styles.earningText}>+$VIBERS</Text>
              </Animated.View>
            </View>
          )}
        </Animated.View>
      </View>

      {/* Track Info */}
      <View style={styles.trackInfo}>
        <Text style={styles.trackTitle}>{track.title}</Text>
        <Text style={styles.trackArtist}>{track.artist}</Text>
        
        {/* Earnings Display */}
        <View style={styles.earningsContainer}>
          <LinearGradient
            colors={[theme.primary, theme.colors.primaryLight]}
            style={styles.earningsCard}
          >
            <Text style={styles.earningsLabel}>Ganado en esta sesión</Text>
            <Text style={styles.earningsAmount}>
              +{earnedFromTrack.toFixed(2)} $VIBERS
            </Text>
            {track.campaign?.active && (
              <Text style={styles.earningsMultiplier}>
                🔥 ×{track.campaign.multiplier} multiplicador activo
              </Text>
            )}
          </LinearGradient>
        </View>
      </View>

      {/* Progress Bar */}
      <View style={styles.progressContainer}>
        <View style={styles.progressBar}>
          <Animated.View 
            style={[
              styles.progressFill,
              {
                width: progressAnim.interpolate({
                  inputRange: [0, 1],
                  outputRange: ['0%', '100%'],
                })
              }
            ]}
          />
        </View>
        
        <View style={styles.timeContainer}>
          <Text style={styles.timeText}>{formatTime(currentTime)}</Text>
          <Text style={styles.timeText}>{formatTime(totalTime)}</Text>
        </View>
      </View>

      {/* Controls */}
      <View style={styles.controlsContainer}>
        <TouchableOpacity style={styles.controlButton}>
          <Text style={styles.controlIcon}>⏮</Text>
        </TouchableOpacity>
        
        <TouchableOpacity 
          style={[styles.playButton, isPlaying && styles.playButtonActive]}
          onPress={handlePlayPause}
        >
          <LinearGradient
            colors={
              isPlaying 
                ? [theme.colors.accentPink, theme.colors.primaryLight]
                : [theme.primary, theme.colors.primaryLight]
            }
            style={styles.playGradient}
          >
            <Text style={styles.playIcon}>
              {isPlaying ? '⏸' : '▶️'}
            </Text>
          </LinearGradient>
        </TouchableOpacity>
        
        <TouchableOpacity style={styles.controlButton}>
          <Text style={styles.controlIcon}>⏭</Text>
        </TouchableOpacity>
      </View>

      {/* ZK Proof Status */}
      {zkProofGenerated && (
        <View style={styles.zkProofContainer}>
          <LinearGradient
            colors={[theme.colors.success, theme.colors.primaryLight]}
            style={styles.zkProofBadge}
          >
            <Text style={styles.zkProofText}>
              ✅ Prueba ZK generada y verificada
            </Text>
          </LinearGradient>
        </View>
      )}

      {/* Action Buttons */}
      <View style={styles.actionButtons}>
        <TouchableOpacity style={styles.actionButton}>
          <Text style={styles.actionIcon}>♡</Text>
          <Text style={styles.actionText}>Me gusta</Text>
        </TouchableOpacity>
        
        <TouchableOpacity style={styles.actionButton}>
          <Text style={styles.actionIcon}>↗</Text>
          <Text style={styles.actionText}>Compartir</Text>
        </TouchableOpacity>
        
        <TouchableOpacity style={styles.actionButton}>
          <Text style={styles.actionIcon}>💎</Text>
          <Text style={styles.actionText}>Comprar NFT</Text>
        </TouchableOpacity>
      </View>
    </View>
  );
};

const createStyles = (theme: ReturnType<typeof useTheme>) => StyleSheet.create({
  container: {
    flex: 1,
    paddingHorizontal: theme.spacing.lg,
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
    alignItems: 'center',
    paddingTop: theme.spacing.xxl + theme.spacing.md,
    paddingBottom: theme.spacing.lg,
  },
  backButton: {
    backgroundColor: theme.colors.glassLight,
    width: 40,
    height: 40,
    borderRadius: 20,
    justifyContent: 'center',
    alignItems: 'center',
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
  },
  backIcon: {
    fontSize: 18,
    color: theme.text,
  },
  headerTitle: {
    ...theme.styles.titleMedium,
    color: theme.text,
    fontWeight: 'bold',
  },
  moreButton: {
    backgroundColor: theme.colors.glassLight,
    width: 40,
    height: 40,
    borderRadius: 20,
    justifyContent: 'center',
    alignItems: 'center',
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
  },
  moreIcon: {
    fontSize: 18,
    color: theme.text,
  },
  albumContainer: {
    alignItems: 'center',
    marginVertical: theme.spacing.xxl,
  },
  albumImageContainer: {
    position: 'relative',
    ...theme.shadows.xl,
  },
  albumImage: {
    width: width * 0.75,
    height: width * 0.75,
    borderRadius: theme.borderRadius.xl,
  },
  campaignOverlay: {
    position: 'absolute',
    top: theme.spacing.md,
    right: theme.spacing.md,
  },
  campaignBadge: {
    paddingHorizontal: theme.spacing.md,
    paddingVertical: theme.spacing.sm,
    borderRadius: theme.borderRadius.md,
  },
  campaignText: {
    color: theme.text,
    fontWeight: 'bold',
    fontSize: 14,
  },
  earningIndicator: {
    position: 'absolute',
    bottom: theme.spacing.md,
    left: theme.spacing.md,
  },
  earningPulse: {
    backgroundColor: theme.primary,
    paddingHorizontal: theme.spacing.md,
    paddingVertical: theme.spacing.sm,
    borderRadius: theme.borderRadius.md,
  },
  earningText: {
    color: theme.text,
    fontWeight: 'bold',
    fontSize: 12,
  },
  trackInfo: {
    alignItems: 'center',
    marginBottom: theme.spacing.xl,
  },
  trackTitle: {
    ...theme.styles.titleLarge,
    color: theme.text,
    fontWeight: 'bold',
    textAlign: 'center',
    marginBottom: theme.spacing.sm,
  },
  trackArtist: {
    ...theme.styles.titleSmall,
    color: theme.textSecondary,
    textAlign: 'center',
    marginBottom: theme.spacing.lg,
  },
  earningsContainer: {
    width: '100%',
    borderRadius: theme.borderRadius.md,
    ...theme.shadows.md,
  },
  earningsCard: {
    padding: theme.spacing.lg,
    borderRadius: theme.borderRadius.md,
    alignItems: 'center',
  },
  earningsLabel: {
    fontSize: 12,
    color: theme.text,
    opacity: 0.8,
    marginBottom: theme.spacing.xs,
  },
  earningsAmount: {
    fontSize: 20,
    color: theme.text,
    fontWeight: 'bold',
    marginBottom: theme.spacing.xs,
  },
  earningsMultiplier: {
    fontSize: 12,
    color: theme.text,
    fontWeight: 'bold',
  },
  progressContainer: {
    marginBottom: theme.spacing.xl,
  },
  progressBar: {
    height: 4,
    backgroundColor: theme.colors.glassMedium,
    borderRadius: 2,
    marginBottom: theme.spacing.md,
  },
  progressFill: {
    height: '100%',
    backgroundColor: theme.primary,
    borderRadius: 2,
  },
  timeContainer: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
  timeText: {
    fontSize: 12,
    color: theme.textMuted,
  },
  controlsContainer: {
    flexDirection: 'row',
    justifyContent: 'center',
    alignItems: 'center',
    gap: theme.spacing.xxl,
    marginBottom: theme.spacing.xl,
  },
  controlButton: {
    backgroundColor: theme.colors.glassLight,
    width: 50,
    height: 50,
    borderRadius: 25,
    justifyContent: 'center',
    alignItems: 'center',
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
  },
  controlIcon: {
    fontSize: 20,
    color: theme.text,
  },
  playButton: {
    width: 70,
    height: 70,
    borderRadius: 35,
    ...theme.shadows.lg,
  },
  playButtonActive: {
    transform: [{ scale: 1.05 }],
  },
  playGradient: {
    width: '100%',
    height: '100%',
    borderRadius: 35,
    justifyContent: 'center',
    alignItems: 'center',
  },
  playIcon: {
    fontSize: 24,
    color: theme.text,
  },
  zkProofContainer: {
    marginBottom: theme.spacing.lg,
    borderRadius: theme.borderRadius.md,
    ...theme.shadows.sm,
  },
  zkProofBadge: {
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    alignItems: 'center',
  },
  zkProofText: {
    color: theme.text,
    fontWeight: 'bold',
    fontSize: 14,
  },
  actionButtons: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    paddingBottom: theme.spacing.xxl,
  },
  actionButton: {
    alignItems: 'center',
    gap: theme.spacing.sm,
  },
  actionIcon: {
    fontSize: 24,
    color: theme.textSecondary,
  },
  actionText: {
    fontSize: 12,
    color: theme.textSecondary,
    fontWeight: '500',
  },
});

export default MusicPlayerScreen; 