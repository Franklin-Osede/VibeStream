import React, { useState, useEffect, useRef } from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  StyleSheet,
  Dimensions,
  Image,
  ScrollView,
  Alert,
  Animated,
  PanGestureHandler,
  State,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { Ionicons } from '@expo/vector-icons';
import { SafeAreaView } from 'react-native-safe-area-context';
import Slider from '@react-native-community/slider';

const { width, height } = Dimensions.get('window');

interface MusicPlayerScreenProps {
  navigation: any;
  route: {
    params: {
      song: any;
      user: any;
    };
  };
}

export default function MusicPlayerScreen({ navigation, route }: MusicPlayerScreenProps) {
  const { song, user } = route.params;
  
  // Estado del reproductor
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(song?.duration || 180);
  const [isLiked, setIsLiked] = useState(false);
  const [isReposted, setIsReposted] = useState(false);
  const [volume, setVolume] = useState(0.8);
  const [isShuffled, setIsShuffled] = useState(false);
  const [repeatMode, setRepeatMode] = useState<'none' | 'one' | 'all'>('none');
  
  // Animaciones
  const rotateAnim = useRef(new Animated.Value(0)).current;
  const slideAnim = useRef(new Animated.Value(0)).current;
  
  // Estado de blockchain
  const [vibersEarned, setVibersEarned] = useState(0);
  const [zkProofs, setZkProofs] = useState(0);
  const [isEarning, setIsEarning] = useState(false);

  useEffect(() => {
    // Iniciar animación de rotación del disco
    if (isPlaying) {
      Animated.loop(
        Animated.timing(rotateAnim, {
          toValue: 1,
          duration: 3000,
          useNativeDriver: true,
        })
      ).start();
    } else {
      rotateAnim.stopAnimation();
    }
  }, [isPlaying]);

  // Simular ganancia de Vibers mientras se reproduce
  useEffect(() => {
    let interval: NodeJS.Timeout;
    if (isPlaying && isEarning) {
      interval = setInterval(() => {
        setVibersEarned(prev => prev + 0.1);
        setZkProofs(prev => prev + 1);
      }, 1000);
    }
    return () => clearInterval(interval);
  }, [isPlaying, isEarning]);

  const togglePlayPause = () => {
    setIsPlaying(!isPlaying);
    setIsEarning(!isEarning);
  };

  const handleLike = () => {
    setIsLiked(!isLiked);
    // Aquí conectarías con el backend
  };

  const handleRepost = () => {
    setIsReposted(!isReposted);
    // Aquí conectarías con el backend
  };

  const handleBuyShares = () => {
    Alert.alert(
      '🔗 Comprar Participaciones',
      `¿Comprar participaciones fraccionadas de "${song?.title}"?`,
      [
        { text: 'Cancelar', style: 'cancel' },
        { 
          text: 'Comprar', 
          onPress: () => {
            // Navegar a pantalla de trading
            navigation.navigate('TradingScreen', { song });
          }
        },
      ]
    );
  };

  const handleBuyNFT = () => {
    Alert.alert(
      '🎨 Comprar NFT',
      `¿Comprar NFT exclusivo de "${song?.title}"?`,
      [
        { text: 'Cancelar', style: 'cancel' },
        { 
          text: 'Comprar', 
          onPress: () => {
            // Navegar a marketplace de NFTs
            navigation.navigate('NFTMarketplace', { song });
          }
        },
      ]
    );
  };

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const spin = rotateAnim.interpolate({
    inputRange: [0, 1],
    outputRange: ['0deg', '360deg'],
  });

  return (
    <SafeAreaView style={styles.container}>
      <LinearGradient
        colors={['#0F0F1E', '#1A1A2E', '#16213E']}
        style={styles.gradient}
      >
        {/* Header */}
        <View style={styles.header}>
          <TouchableOpacity onPress={() => navigation.goBack()}>
            <Ionicons name="chevron-down" size={30} color="#FFFFFF" />
          </TouchableOpacity>
          <Text style={styles.headerTitle}>Reproduciendo</Text>
          <TouchableOpacity>
            <Ionicons name="ellipsis-horizontal" size={24} color="#FFFFFF" />
          </TouchableOpacity>
        </View>

        <ScrollView style={styles.content} showsVerticalScrollIndicator={false}>
          {/* Album Art */}
          <View style={styles.albumContainer}>
            <Animated.View
              style={[
                styles.albumArt,
                { transform: [{ rotate: spin }] }
              ]}
            >
              <Image
                source={{ uri: song?.imageUrl || 'https://via.placeholder.com/300' }}
                style={styles.albumImage}
              />
            </Animated.View>
          </View>

          {/* Song Info */}
          <View style={styles.songInfo}>
            <Text style={styles.songTitle}>{song?.title || 'Título de la canción'}</Text>
            <Text style={styles.artistName}>{song?.artist || 'Artista'}</Text>
            
            {/* Blockchain Info */}
            <View style={styles.blockchainInfo}>
              <View style={styles.blockchainItem}>
                <Text style={styles.blockchainLabel}>Vibers Ganados</Text>
                <Text style={styles.blockchainValue}>{vibersEarned.toFixed(1)}</Text>
              </View>
              <View style={styles.blockchainItem}>
                <Text style={styles.blockchainLabel}>ZK Proofs</Text>
                <Text style={styles.blockchainValue}>{zkProofs}</Text>
              </View>
            </View>
          </View>

          {/* Progress Bar */}
          <View style={styles.progressContainer}>
            <Slider
              style={styles.progressBar}
              minimumValue={0}
              maximumValue={duration}
              value={currentTime}
              onValueChange={setCurrentTime}
              minimumTrackTintColor="#6C5CE7"
              maximumTrackTintColor="#3A3A4A"
              thumbStyle={styles.thumb}
            />
            <View style={styles.timeContainer}>
              <Text style={styles.timeText}>{formatTime(currentTime)}</Text>
              <Text style={styles.timeText}>{formatTime(duration)}</Text>
            </View>
          </View>

          {/* Main Controls */}
          <View style={styles.mainControls}>
            <TouchableOpacity style={styles.controlButton}>
              <Ionicons name="shuffle" size={24} color={isShuffled ? "#6C5CE7" : "#94A3B8"} />
            </TouchableOpacity>
            
            <TouchableOpacity style={styles.controlButton}>
              <Ionicons name="play-skip-back" size={28} color="#FFFFFF" />
            </TouchableOpacity>
            
            <TouchableOpacity style={styles.playButton} onPress={togglePlayPause}>
              <Ionicons 
                name={isPlaying ? "pause" : "play"} 
                size={40} 
                color="#FFFFFF" 
              />
            </TouchableOpacity>
            
            <TouchableOpacity style={styles.controlButton}>
              <Ionicons name="play-skip-forward" size={28} color="#FFFFFF" />
            </TouchableOpacity>
            
            <TouchableOpacity style={styles.controlButton}>
              <Ionicons 
                name={repeatMode === 'none' ? 'repeat' : repeatMode === 'one' ? 'repeat-one' : 'repeat'} 
                size={24} 
                color={repeatMode !== 'none' ? "#6C5CE7" : "#94A3B8"} 
              />
            </TouchableOpacity>
          </View>

          {/* Action Buttons */}
          <View style={styles.actionButtons}>
            <TouchableOpacity style={styles.actionButton} onPress={handleLike}>
              <Ionicons 
                name={isLiked ? "heart" : "heart-outline"} 
                size={24} 
                color={isLiked ? "#FF6B6B" : "#94A3B8"} 
              />
            </TouchableOpacity>
            
            <TouchableOpacity style={styles.actionButton} onPress={handleRepost}>
              <Ionicons 
                name={isReposted ? "repeat" : "repeat-outline"} 
                size={24} 
                color={isReposted ? "#6C5CE7" : "#94A3B8"} 
              />
            </TouchableOpacity>
            
            <TouchableOpacity style={styles.actionButton}>
              <Ionicons name="share-outline" size={24} color="#94A3B8" />
            </TouchableOpacity>
            
            <TouchableOpacity style={styles.actionButton}>
              <Ionicons name="download-outline" size={24} color="#94A3B8" />
            </TouchableOpacity>
          </View>

          {/* VibeStream Features */}
          <View style={styles.featuresContainer}>
            <Text style={styles.featuresTitle}>Funcionalidades VibeStream</Text>
            
            {/* Trading Fraccional */}
            <TouchableOpacity style={styles.featureCard} onPress={handleBuyShares}>
              <LinearGradient
                colors={['#6C5CE7', '#A29BFE']}
                style={styles.featureGradient}
              >
                <Text style={styles.featureIcon}>📈</Text>
                <View style={styles.featureContent}>
                  <Text style={styles.featureTitle}>Comprar Participaciones</Text>
                  <Text style={styles.featureDescription}>
                    Invierte en esta canción y recibe royalties automáticos
                  </Text>
                </View>
              </LinearGradient>
            </TouchableOpacity>

            {/* NFT Collection */}
            <TouchableOpacity style={styles.featureCard} onPress={handleBuyNFT}>
              <LinearGradient
                colors={['#00D4FF', '#66E5FF']}
                style={styles.featureGradient}
              >
                <Text style={styles.featureIcon}>🎨</Text>
                <View style={styles.featureContent}>
                  <Text style={styles.featureTitle}>Colección NFT</Text>
                  <Text style={styles.featureDescription}>
                    NFTs exclusivos con regalías automáticas
                  </Text>
                </View>
              </LinearGradient>
            </TouchableOpacity>

            {/* VR Concert */}
            <TouchableOpacity style={styles.featureCard}>
              <LinearGradient
                colors={['#FF6B6B', '#FD79A8']}
                style={styles.featureGradient}
              >
                <Text style={styles.featureIcon}>🥽</Text>
                <View style={styles.featureContent}>
                  <Text style={styles.featureTitle}>Concierto VR</Text>
                  <Text style={styles.featureDescription}>
                    Próximo evento en realidad virtual
                  </Text>
                </View>
              </LinearGradient>
            </TouchableOpacity>
          </View>
        </ScrollView>
      </LinearGradient>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  gradient: {
    flex: 1,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: 20,
    paddingVertical: 10,
  },
  headerTitle: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: '600',
  },
  content: {
    flex: 1,
    paddingHorizontal: 20,
  },
  albumContainer: {
    alignItems: 'center',
    marginVertical: 30,
  },
  albumArt: {
    width: 280,
    height: 280,
    borderRadius: 140,
    elevation: 10,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 10 },
    shadowOpacity: 0.3,
    shadowRadius: 20,
  },
  albumImage: {
    width: '100%',
    height: '100%',
    borderRadius: 140,
  },
  songInfo: {
    alignItems: 'center',
    marginBottom: 30,
  },
  songTitle: {
    color: '#FFFFFF',
    fontSize: 24,
    fontWeight: 'bold',
    textAlign: 'center',
    marginBottom: 8,
  },
  artistName: {
    color: '#94A3B8',
    fontSize: 18,
    textAlign: 'center',
    marginBottom: 20,
  },
  blockchainInfo: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    width: '100%',
    marginTop: 15,
  },
  blockchainItem: {
    alignItems: 'center',
  },
  blockchainLabel: {
    color: '#94A3B8',
    fontSize: 12,
    marginBottom: 4,
  },
  blockchainValue: {
    color: '#6C5CE7',
    fontSize: 16,
    fontWeight: 'bold',
  },
  progressContainer: {
    marginBottom: 30,
  },
  progressBar: {
    width: '100%',
    height: 40,
  },
  thumb: {
    backgroundColor: '#6C5CE7',
    width: 20,
    height: 20,
  },
  timeContainer: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginTop: 10,
  },
  timeText: {
    color: '#94A3B8',
    fontSize: 14,
  },
  mainControls: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 30,
    paddingHorizontal: 20,
  },
  controlButton: {
    padding: 10,
  },
  playButton: {
    width: 80,
    height: 80,
    borderRadius: 40,
    backgroundColor: '#6C5CE7',
    justifyContent: 'center',
    alignItems: 'center',
    elevation: 5,
    shadowColor: '#6C5CE7',
    shadowOffset: { width: 0, height: 5 },
    shadowOpacity: 0.3,
    shadowRadius: 10,
  },
  actionButtons: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    marginBottom: 40,
  },
  actionButton: {
    padding: 15,
  },
  featuresContainer: {
    marginBottom: 30,
  },
  featuresTitle: {
    color: '#FFFFFF',
    fontSize: 20,
    fontWeight: 'bold',
    marginBottom: 20,
    textAlign: 'center',
  },
  featureCard: {
    marginBottom: 15,
    borderRadius: 15,
    overflow: 'hidden',
  },
  featureGradient: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: 20,
  },
  featureIcon: {
    fontSize: 30,
    marginRight: 15,
  },
  featureContent: {
    flex: 1,
  },
  featureTitle: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: 'bold',
    marginBottom: 4,
  },
  featureDescription: {
    color: '#FFFFFF',
    fontSize: 14,
    opacity: 0.9,
  },
}); 