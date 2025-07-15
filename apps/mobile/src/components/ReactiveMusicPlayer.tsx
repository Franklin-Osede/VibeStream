import React, { useState } from 'react';
import { View, Text, TouchableOpacity, StyleSheet, Dimensions } from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { Ionicons } from '@expo/vector-icons';
import Slider from '@react-native-community/slider';

const { width } = Dimensions.get('window');

interface Song {
  id: string;
  title: string;
  artist: string;
  duration: number;
  imageUrl?: string;
}

interface MusicPlayerProps {
  onMinimize?: () => void;
  onMaximize?: () => void;
  currentSong?: Song;
  onPlayPause?: () => void;
  onNext?: () => void;
  onPrevious?: () => void;
  onSeek?: (time: number) => void;
}

export const ReactiveMusicPlayer = ({ 
  onMinimize, 
  onMaximize, 
  currentSong,
  onPlayPause,
  onNext,
  onPrevious,
  onSeek
}: MusicPlayerProps) => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(currentSong?.duration || 0);

  // Mock song data
  const mockSong: Song = {
    id: '1',
    title: 'Midnight Vibes',
    artist: 'Luna Echo',
    duration: 180, // 3 minutes
    imageUrl: 'https://via.placeholder.com/300'
  };

  const song = currentSong || mockSong;

  const handlePlayPause = () => {
    setIsPlaying(!isPlaying);
    onPlayPause?.();
  };

  const handleSeek = (value: number) => {
    setCurrentTime(value);
    onSeek?.(value);
  };

  const handleNext = () => {
    onNext?.();
  };

  const handlePrevious = () => {
    onPrevious?.();
  };

  const formatTime = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  if (!song) {
    return null;
  }

  return (
    <LinearGradient
      colors={['#667eea', '#764ba2']}
      style={styles.container}
    >
      <View style={styles.header}>
        <TouchableOpacity onPress={onMinimize} style={styles.minimizeButton}>
          <Ionicons name="chevron-down" size={24} color="white" />
        </TouchableOpacity>
        <Text style={styles.nowPlayingText}>Now Playing</Text>
        <TouchableOpacity style={styles.moreButton}>
          <Ionicons name="ellipsis-horizontal" size={24} color="white" />
        </TouchableOpacity>
      </View>

      <View style={styles.artworkContainer}>
        <View style={styles.artwork} />
      </View>

      <View style={styles.songInfo}>
        <Text style={styles.songTitle} numberOfLines={1}>
          {song.title}
        </Text>
        <Text style={styles.artistName} numberOfLines={1}>
          {song.artist}
        </Text>
      </View>

      <View style={styles.progressContainer}>
        <Slider
          style={styles.progressSlider}
          minimumValue={0}
          maximumValue={duration}
          value={currentTime}
          onSlidingComplete={handleSeek}
          minimumTrackTintColor="#ffffff"
          maximumTrackTintColor="rgba(255,255,255,0.3)"
          thumbTintColor="#ffffff"
        />
        <View style={styles.timeContainer}>
          <Text style={styles.timeText}>{formatTime(currentTime)}</Text>
          <Text style={styles.timeText}>{formatTime(duration)}</Text>
        </View>
      </View>

      <View style={styles.controls}>
        <TouchableOpacity onPress={handlePrevious} style={styles.controlButton}>
          <Ionicons name="play-skip-back" size={32} color="white" />
        </TouchableOpacity>

        <TouchableOpacity onPress={handlePlayPause} style={styles.playButton}>
          <Ionicons 
            name={isPlaying ? "pause" : "play"} 
            size={40} 
            color="#667eea" 
          />
        </TouchableOpacity>

        <TouchableOpacity onPress={handleNext} style={styles.controlButton}>
          <Ionicons name="play-skip-forward" size={32} color="white" />
        </TouchableOpacity>
      </View>

      <View style={styles.actions}>
        <TouchableOpacity style={styles.actionButton}>
          <Ionicons name="heart-outline" size={24} color="white" />
        </TouchableOpacity>
        <TouchableOpacity style={styles.actionButton}>
          <Ionicons name="share-outline" size={24} color="white" />
        </TouchableOpacity>
        <TouchableOpacity style={styles.actionButton}>
          <Ionicons name="list" size={24} color="white" />
        </TouchableOpacity>
      </View>
    </LinearGradient>
  );
};

const styles = StyleSheet.create({
  container: {
    position: 'absolute',
    bottom: 0,
    left: 0,
    right: 0,
    height: 300,
    padding: 20,
    borderTopLeftRadius: 20,
    borderTopRightRadius: 20,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 20,
  },
  minimizeButton: {
    padding: 5,
  },
  nowPlayingText: {
    color: 'white',
    fontSize: 16,
    fontWeight: '600',
  },
  moreButton: {
    padding: 5,
  },
  artworkContainer: {
    alignItems: 'center',
    marginBottom: 20,
  },
  artwork: {
    width: 200,
    height: 200,
    borderRadius: 20,
    backgroundColor: 'rgba(255,255,255,0.2)',
  },
  songInfo: {
    alignItems: 'center',
    marginBottom: 20,
  },
  songTitle: {
    color: 'white',
    fontSize: 20,
    fontWeight: 'bold',
    marginBottom: 5,
  },
  artistName: {
    color: 'rgba(255,255,255,0.8)',
    fontSize: 16,
  },
  progressContainer: {
    marginBottom: 20,
  },
  progressSlider: {
    width: '100%',
    height: 40,
  },
  sliderThumb: {
    backgroundColor: 'white',
  },
  timeContainer: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginTop: 5,
  },
  timeText: {
    color: 'rgba(255,255,255,0.8)',
    fontSize: 12,
  },
  controls: {
    flexDirection: 'row',
    justifyContent: 'center',
    alignItems: 'center',
    marginBottom: 20,
  },
  controlButton: {
    padding: 10,
    marginHorizontal: 20,
  },
  playButton: {
    width: 60,
    height: 60,
    borderRadius: 30,
    backgroundColor: 'white',
    justifyContent: 'center',
    alignItems: 'center',
    marginHorizontal: 20,
  },
  actions: {
    flexDirection: 'row',
    justifyContent: 'space-around',
  },
  actionButton: {
    padding: 10,
  },
}); 