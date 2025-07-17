import React, { useState, useEffect } from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  StyleSheet,
  ScrollView,
  Alert,
  Dimensions,
  Image,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { Ionicons } from '@expo/vector-icons';
import { SafeAreaView } from 'react-native-safe-area-context';

const { width, height } = Dimensions.get('window');

interface VREventsScreenProps {
  navigation: any;
  route: {
    params: {
      user: any;
    };
  };
}

export default function VREventsScreen({ navigation, route }: VREventsScreenProps) {
  const { user } = route.params;
  
  // Estado de eventos VR
  const [selectedTab, setSelectedTab] = useState<'upcoming' | 'live' | 'past'>('upcoming');
  const [selectedEvent, setSelectedEvent] = useState(null);
  
  // Datos mock de eventos VR
  const [vrEvents, setVrEvents] = useState([
    {
      id: '1',
      title: 'Neon Dreams VR Concert',
      artist: 'Cyber Collective',
      description: 'Experience the future of music in virtual reality with stunning visual effects and immersive audio',
      startTime: new Date('2024-03-15T20:00:00Z'),
      endTime: new Date('2024-03-15T22:00:00Z'),
      price: 0.1,
      currency: 'ETH',
      attendees: 150,
      maxAttendees: 200,
      imageUrl: 'https://via.placeholder.com/300x200',
      isLive: false,
      isJoined: false,
      vrPlatform: 'Meta Quest',
      genre: 'Electronic',
      specialFeatures: ['Holographic Effects', 'Spatial Audio', 'Interactive Elements'],
    },
    {
      id: '2',
      title: 'Summer Vibes VR Festival',
      artist: 'Beach Waves',
      description: 'A tropical VR experience with beach vibes and chill music',
      startTime: new Date('2024-03-20T19:00:00Z'),
      endTime: new Date('2024-03-20T21:00:00Z'),
      price: 0.05,
      currency: 'ETH',
      attendees: 89,
      maxAttendees: 150,
      imageUrl: 'https://via.placeholder.com/300x200',
      isLive: false,
      isJoined: true,
      vrPlatform: 'Meta Quest',
      genre: 'Chill',
      specialFeatures: ['Beach Environment', 'Sunset Views', 'Relaxing Atmosphere'],
    },
    {
      id: '3',
      title: 'Rock Revolution VR',
      artist: 'Thunder Strike',
      description: 'Epic rock concert in a virtual stadium with pyrotechnics',
      startTime: new Date('2024-03-10T21:00:00Z'),
      endTime: new Date('2024-03-10T23:00:00Z'),
      price: 0.15,
      currency: 'ETH',
      attendees: 200,
      maxAttendees: 200,
      imageUrl: 'https://via.placeholder.com/300x200',
      isLive: true,
      isJoined: false,
      vrPlatform: 'Meta Quest',
      genre: 'Rock',
      specialFeatures: ['Stadium Environment', 'Pyrotechnics', 'Crowd Interaction'],
    },
  ]);

  const handleJoinEvent = (event: any) => {
    if (event.attendees >= event.maxAttendees) {
      Alert.alert('Evento Lleno', 'Este evento ya no tiene cupos disponibles');
      return;
    }

    Alert.alert(
      '🥽 Unirse al Evento VR',
      `¿Unirte a "${event.title}" por ${event.price} ${event.currency}?`,
      [
        { text: 'Cancelar', style: 'cancel' },
        { 
          text: 'Unirse', 
          onPress: () => {
            // Aquí conectarías con el backend
            const updatedEvents = vrEvents.map(e => 
              e.id === event.id 
                ? { ...e, attendees: e.attendees + 1, isJoined: true }
                : e
            );
            setVrEvents(updatedEvents);
            Alert.alert('✅ Éxito', 'Te has unido al evento VR');
          }
        },
      ]
    );
  };

  const handleEnterEvent = (event: any) => {
    if (!event.isJoined) {
      Alert.alert('Error', 'Debes unirte al evento primero');
      return;
    }

    Alert.alert(
      '🥽 Entrar al Evento VR',
      `¿Entrar a "${event.title}"?`,
      [
        { text: 'Cancelar', style: 'cancel' },
        { 
          text: 'Entrar', 
          onPress: () => {
            // Aquí conectarías con el backend de VR
            navigation.navigate('VREventRoom', { event, user });
          }
        },
      ]
    );
  };

  const formatDate = (date: Date) => {
    const now = new Date();
    const diffTime = date.getTime() - now.getTime();
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
    
    if (diffDays === 0) return 'Hoy';
    if (diffDays === 1) return 'Mañana';
    if (diffDays < 0) return 'Pasado';
    return `En ${diffDays} días`;
  };

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString('es-ES', { 
      hour: '2-digit', 
      minute: '2-digit' 
    });
  };

  const getFilteredEvents = () => {
    const now = new Date();
    switch (selectedTab) {
      case 'upcoming':
        return vrEvents.filter(event => event.startTime > now && !event.isLive);
      case 'live':
        return vrEvents.filter(event => event.isLive);
      case 'past':
        return vrEvents.filter(event => event.endTime < now);
      default:
        return vrEvents;
    }
  };

  const renderEventCard = (event: any) => (
    <View key={event.id} style={styles.eventCard}>
      <Image source={{ uri: event.imageUrl }} style={styles.eventImage} />
      
      <View style={styles.eventInfo}>
        <View style={styles.eventHeader}>
          <View style={styles.eventTitleContainer}>
            <Text style={styles.eventTitle}>{event.title}</Text>
            <Text style={styles.eventArtist}>por {event.artist}</Text>
          </View>
          <View style={styles.eventStatus}>
            {event.isLive && (
              <View style={styles.liveBadge}>
                <Text style={styles.liveText}>EN VIVO</Text>
              </View>
            )}
            {event.isJoined && (
              <View style={styles.joinedBadge}>
                <Ionicons name="checkmark-circle" size={16} color="#4ECDC4" />
                <Text style={styles.joinedText}>Unido</Text>
              </View>
            )}
          </View>
        </View>

        <Text style={styles.eventDescription} numberOfLines={2}>
          {event.description}
        </Text>

        <View style={styles.eventDetails}>
          <View style={styles.detailItem}>
            <Ionicons name="calendar" size={16} color="#94A3B8" />
            <Text style={styles.detailText}>{formatDate(event.startTime)}</Text>
          </View>
          <View style={styles.detailItem}>
            <Ionicons name="time" size={16} color="#94A3B8" />
            <Text style={styles.detailText}>{formatTime(event.startTime)}</Text>
          </View>
          <View style={styles.detailItem}>
            <Ionicons name="people" size={16} color="#94A3B8" />
            <Text style={styles.detailText}>
              {event.attendees}/{event.maxAttendees}
            </Text>
          </View>
        </View>

        <View style={styles.eventFeatures}>
          <Text style={styles.featuresLabel}>Características:</Text>
          <View style={styles.featuresList}>
            {event.specialFeatures.slice(0, 2).map((feature: string, index: number) => (
              <View key={index} style={styles.featureTag}>
                <Text style={styles.featureText}>{feature}</Text>
              </View>
            ))}
          </View>
        </View>

        <View style={styles.eventActions}>
          <View style={styles.priceContainer}>
            <Text style={styles.priceLabel}>Precio</Text>
            <Text style={styles.priceValue}>{event.price} {event.currency}</Text>
          </View>
          
          {event.isLive ? (
            <TouchableOpacity
              style={[styles.actionButton, styles.enterButton]}
              onPress={() => handleEnterEvent(event)}
            >
              <Ionicons name="play" size={16} color="#FFFFFF" />
              <Text style={styles.actionButtonText}>Entrar</Text>
            </TouchableOpacity>
          ) : event.isJoined ? (
            <TouchableOpacity
              style={[styles.actionButton, styles.enteredButton]}
              onPress={() => handleEnterEvent(event)}
            >
              <Ionicons name="checkmark" size={16} color="#FFFFFF" />
              <Text style={styles.actionButtonText}>Listo</Text>
            </TouchableOpacity>
          ) : (
            <TouchableOpacity
              style={[styles.actionButton, styles.joinButton]}
              onPress={() => handleJoinEvent(event)}
            >
              <Ionicons name="add" size={16} color="#FFFFFF" />
              <Text style={styles.actionButtonText}>Unirse</Text>
            </TouchableOpacity>
          )}
        </View>
      </View>
    </View>
  );

  return (
    <SafeAreaView style={styles.container}>
      <LinearGradient
        colors={['#0F0F1E', '#1A1A2E', '#16213E']}
        style={styles.gradient}
      >
        {/* Header */}
        <View style={styles.header}>
          <TouchableOpacity onPress={() => navigation.goBack()}>
            <Ionicons name="arrow-back" size={24} color="#FFFFFF" />
          </TouchableOpacity>
          <Text style={styles.headerTitle}>Eventos VR</Text>
          <TouchableOpacity>
            <Ionicons name="add-circle-outline" size={24} color="#FFFFFF" />
          </TouchableOpacity>
        </View>

        {/* Tabs */}
        <View style={styles.tabs}>
          <TouchableOpacity
            style={[styles.tab, selectedTab === 'upcoming' && styles.activeTab]}
            onPress={() => setSelectedTab('upcoming')}
          >
            <Text style={[styles.tabText, selectedTab === 'upcoming' && styles.activeTabText]}>
              Próximos
            </Text>
          </TouchableOpacity>
          <TouchableOpacity
            style={[styles.tab, selectedTab === 'live' && styles.activeTab]}
            onPress={() => setSelectedTab('live')}
          >
            <Text style={[styles.tabText, selectedTab === 'live' && styles.activeTabText]}>
              En Vivo
            </Text>
          </TouchableOpacity>
          <TouchableOpacity
            style={[styles.tab, selectedTab === 'past' && styles.activeTab]}
            onPress={() => setSelectedTab('past')}
          >
            <Text style={[styles.tabText, selectedTab === 'past' && styles.activeTabText]}>
              Pasados
            </Text>
          </TouchableOpacity>
        </View>

        {/* Events List */}
        <ScrollView style={styles.eventsContainer} showsVerticalScrollIndicator={false}>
          {getFilteredEvents().length === 0 ? (
            <View style={styles.emptyState}>
              <Ionicons name="vr" size={64} color="#94A3B8" />
              <Text style={styles.emptyTitle}>No hay eventos {selectedTab}</Text>
              <Text style={styles.emptyDescription}>
                {selectedTab === 'upcoming' && 'No hay eventos programados próximamente'}
                {selectedTab === 'live' && 'No hay eventos en vivo en este momento'}
                {selectedTab === 'past' && 'No hay eventos pasados para mostrar'}
              </Text>
            </View>
          ) : (
            getFilteredEvents().map(renderEventCard)
          )}
        </ScrollView>

        {/* VR Info Banner */}
        <View style={styles.vrInfoBanner}>
          <LinearGradient
            colors={['#6C5CE7', '#A29BFE']}
            style={styles.bannerGradient}
          >
            <Ionicons name="information-circle" size={20} color="#FFFFFF" />
            <Text style={styles.bannerText}>
              Compatible con Meta Quest y otros dispositivos VR
            </Text>
          </LinearGradient>
        </View>
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
    paddingVertical: 15,
  },
  headerTitle: {
    color: '#FFFFFF',
    fontSize: 18,
    fontWeight: 'bold',
  },
  tabs: {
    flexDirection: 'row',
    paddingHorizontal: 20,
    marginBottom: 20,
  },
  tab: {
    flex: 1,
    paddingVertical: 12,
    alignItems: 'center',
    borderBottomWidth: 2,
    borderBottomColor: 'transparent',
  },
  activeTab: {
    borderBottomColor: '#6C5CE7',
  },
  tabText: {
    color: '#94A3B8',
    fontSize: 14,
    fontWeight: '500',
  },
  activeTabText: {
    color: '#6C5CE7',
  },
  eventsContainer: {
    flex: 1,
    paddingHorizontal: 20,
  },
  eventCard: {
    backgroundColor: '#2D2D3A',
    borderRadius: 15,
    marginBottom: 20,
    overflow: 'hidden',
  },
  eventImage: {
    width: '100%',
    height: 150,
  },
  eventInfo: {
    padding: 15,
  },
  eventHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
    marginBottom: 10,
  },
  eventTitleContainer: {
    flex: 1,
    marginRight: 10,
  },
  eventTitle: {
    color: '#FFFFFF',
    fontSize: 18,
    fontWeight: 'bold',
    marginBottom: 4,
  },
  eventArtist: {
    color: '#94A3B8',
    fontSize: 14,
  },
  eventStatus: {
    alignItems: 'flex-end',
  },
  liveBadge: {
    backgroundColor: '#FF6B6B',
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 12,
    marginBottom: 5,
  },
  liveText: {
    color: '#FFFFFF',
    fontSize: 10,
    fontWeight: 'bold',
  },
  joinedBadge: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: 'rgba(78, 205, 196, 0.2)',
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 12,
  },
  joinedText: {
    color: '#4ECDC4',
    fontSize: 10,
    fontWeight: '600',
    marginLeft: 4,
  },
  eventDescription: {
    color: '#E2E8F0',
    fontSize: 14,
    lineHeight: 20,
    marginBottom: 15,
  },
  eventDetails: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: 15,
  },
  detailItem: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  detailText: {
    color: '#94A3B8',
    fontSize: 12,
    marginLeft: 4,
  },
  eventFeatures: {
    marginBottom: 15,
  },
  featuresLabel: {
    color: '#94A3B8',
    fontSize: 12,
    marginBottom: 8,
  },
  featuresList: {
    flexDirection: 'row',
    flexWrap: 'wrap',
  },
  featureTag: {
    backgroundColor: 'rgba(108, 92, 231, 0.2)',
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 8,
    marginRight: 8,
    marginBottom: 4,
  },
  featureText: {
    color: '#6C5CE7',
    fontSize: 10,
    fontWeight: '500',
  },
  eventActions: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  priceContainer: {
    alignItems: 'flex-start',
  },
  priceLabel: {
    color: '#94A3B8',
    fontSize: 12,
    marginBottom: 2,
  },
  priceValue: {
    color: '#FFFFFF',
    fontSize: 16,
    fontWeight: 'bold',
  },
  actionButton: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingHorizontal: 16,
    paddingVertical: 8,
    borderRadius: 20,
  },
  joinButton: {
    backgroundColor: '#6C5CE7',
  },
  enterButton: {
    backgroundColor: '#4ECDC4',
  },
  enteredButton: {
    backgroundColor: '#FFD93D',
  },
  actionButtonText: {
    color: '#FFFFFF',
    fontSize: 14,
    fontWeight: '600',
    marginLeft: 4,
  },
  emptyState: {
    alignItems: 'center',
    justifyContent: 'center',
    paddingVertical: 60,
  },
  emptyTitle: {
    color: '#FFFFFF',
    fontSize: 18,
    fontWeight: 'bold',
    marginTop: 20,
    marginBottom: 10,
  },
  emptyDescription: {
    color: '#94A3B8',
    fontSize: 14,
    textAlign: 'center',
    paddingHorizontal: 40,
  },
  vrInfoBanner: {
    paddingHorizontal: 20,
    paddingBottom: 20,
  },
  bannerGradient: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: 15,
    borderRadius: 12,
  },
  bannerText: {
    color: '#FFFFFF',
    fontSize: 14,
    marginLeft: 10,
    flex: 1,
  },
}); 