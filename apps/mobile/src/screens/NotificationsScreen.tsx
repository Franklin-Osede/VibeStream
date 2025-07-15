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

interface Notification {
  id: string;
  type: 'like' | 'comment' | 'follow' | 'vr_event' | 'nft' | 'system' | 'trading';
  title: string;
  message: string;
  imageUrl?: string;
  timestamp: string;
  isRead: boolean;
  actionUrl?: string;
  metadata?: {
    artistName?: string;
    eventName?: string;
    nftName?: string;
    amount?: string;
    currency?: string;
  };
}

const mockNotifications: Notification[] = [
  {
    id: '1',
    type: 'like',
    title: 'Luna Echo liked your post',
    message: 'They liked your track "Midnight Vibes"',
    imageUrl: 'https://via.placeholder.com/50',
    timestamp: '2m ago',
    isRead: false,
    metadata: {
      artistName: 'Luna Echo',
    },
  },
  {
    id: '2',
    type: 'vr_event',
    title: 'VR Event Starting Soon',
    message: 'Neon Dreams VR Concert starts in 15 minutes',
    imageUrl: 'https://via.placeholder.com/50',
    timestamp: '5m ago',
    isRead: false,
    metadata: {
      eventName: 'Neon Dreams VR Concert',
    },
  },
  {
    id: '3',
    type: 'nft',
    title: 'New NFT Available',
    message: 'Genesis Collection #2 is now available for purchase',
    imageUrl: 'https://via.placeholder.com/50',
    timestamp: '10m ago',
    isRead: false,
    metadata: {
      nftName: 'Genesis Collection #2',
    },
  },
  {
    id: '4',
    type: 'follow',
    title: 'Cyber Collective started following you',
    message: 'They are now following your profile',
    imageUrl: 'https://via.placeholder.com/50',
    timestamp: '15m ago',
    isRead: true,
    metadata: {
      artistName: 'Cyber Collective',
    },
  },
  {
    id: '5',
    type: 'trading',
    title: 'Trading Opportunity',
    message: 'Your NFT "Genesis Collection #1" price increased by 25%',
    imageUrl: 'https://via.placeholder.com/50',
    timestamp: '1h ago',
    isRead: true,
    metadata: {
      nftName: 'Genesis Collection #1',
      amount: '25',
    },
  },
  {
    id: '6',
    type: 'comment',
    title: 'Stellar Sound commented on your track',
    message: '"Amazing vibes! Love the electronic elements"',
    imageUrl: 'https://via.placeholder.com/50',
    timestamp: '2h ago',
    isRead: true,
    metadata: {
      artistName: 'Stellar Sound',
    },
  },
  {
    id: '7',
    type: 'system',
    title: 'Welcome to VibeStream!',
    message: 'Your account has been successfully created. Start exploring!',
    timestamp: '1d ago',
    isRead: true,
  },
  {
    id: '8',
    type: 'vr_event',
    title: 'VR Event Reminder',
    message: 'Don\'t forget: Cosmic Journey VR starts tomorrow at 8 PM',
    imageUrl: 'https://via.placeholder.com/50',
    timestamp: '1d ago',
    isRead: true,
    metadata: {
      eventName: 'Cosmic Journey VR',
    },
  },
];

const notificationTypes = [
  { id: 'all', title: 'All', icon: 'notifications' },
  { id: 'social', title: 'Social', icon: 'people' },
  { id: 'events', title: 'Events', icon: 'glasses' },
  { id: 'nfts', title: 'NFTs', icon: 'diamond' },
  { id: 'trading', title: 'Trading', icon: 'trending-up' },
  { id: 'system', title: 'System', icon: 'settings' },
];

export default function NotificationsScreen() {
  const [selectedType, setSelectedType] = React.useState('all');
  const [showUnreadOnly, setShowUnreadOnly] = React.useState(false);

  const filteredNotifications = React.useMemo(() => {
    let filtered = mockNotifications;

    // Filtrar por tipo
    if (selectedType !== 'all') {
      filtered = filtered.filter(notification => {
        switch (selectedType) {
          case 'social':
            return ['like', 'comment', 'follow'].includes(notification.type);
          case 'events':
            return notification.type === 'vr_event';
          case 'nfts':
            return notification.type === 'nft';
          case 'trading':
            return notification.type === 'trading';
          case 'system':
            return notification.type === 'system';
          default:
            return true;
        }
      });
    }

    // Filtrar por no leídas
    if (showUnreadOnly) {
      filtered = filtered.filter(notification => !notification.isRead);
    }

    return filtered;
  }, [selectedType, showUnreadOnly]);

  const getTypeIcon = (type: string) => {
    switch (type) {
      case 'like':
        return 'heart';
      case 'comment':
        return 'chatbubble';
      case 'follow':
        return 'person-add';
      case 'vr_event':
        return 'glasses';
      case 'nft':
        return 'diamond';
      case 'trading':
        return 'trending-up';
      case 'system':
        return 'settings';
      default:
        return 'notifications';
    }
  };

  const getTypeColor = (type: string) => {
    switch (type) {
      case 'like':
        return '#EF4444';
      case 'comment':
        return '#10B981';
      case 'follow':
        return '#8B5CF6';
      case 'vr_event':
        return '#F59E0B';
      case 'nft':
        return '#06B6D4';
      case 'trading':
        return '#059669';
      case 'system':
        return '#6B7280';
      default:
        return '#6B7280';
    }
  };

  const markAllAsRead = () => {
    // Implementar lógica para marcar todas como leídas
    console.log('Mark all as read');
  };

  const renderNotification = ({ item }: { item: Notification }) => (
    <TouchableOpacity 
      style={[
        styles.notificationItem,
        !item.isRead && styles.unreadNotification
      ]}
    >
      <View style={styles.notificationContent}>
        <View style={styles.notificationHeader}>
          <View style={styles.notificationIcon}>
            <Ionicons 
              name={getTypeIcon(item.type) as any} 
              size={20} 
              color={getTypeColor(item.type)} 
            />
          </View>
          <View style={styles.notificationText}>
            <Text style={styles.notificationTitle}>{item.title}</Text>
            <Text style={styles.notificationMessage}>{item.message}</Text>
            <Text style={styles.notificationTime}>{item.timestamp}</Text>
          </View>
          {item.imageUrl && (
            <Image source={{ uri: item.imageUrl }} style={styles.notificationImage} />
          )}
        </View>
        {!item.isRead && <View style={styles.unreadDot} />}
      </View>
    </TouchableOpacity>
  );

  const unreadCount = mockNotifications.filter(n => !n.isRead).length;

  return (
    <SafeAreaView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.title}>Notifications</Text>
        <View style={styles.headerActions}>
          {unreadCount > 0 && (
            <TouchableOpacity style={styles.markAllButton} onPress={markAllAsRead}>
              <Text style={styles.markAllText}>Mark all read</Text>
            </TouchableOpacity>
          )}
          <TouchableOpacity 
            style={[styles.filterButton, showUnreadOnly && styles.filterButtonActive]}
            onPress={() => setShowUnreadOnly(!showUnreadOnly)}
          >
            <Ionicons 
              name="funnel" 
              size={20} 
              color={showUnreadOnly ? '#FFFFFF' : '#6B7280'} 
            />
          </TouchableOpacity>
        </View>
      </View>

      {/* Contador de no leídas */}
      {unreadCount > 0 && (
        <View style={styles.unreadBadge}>
          <Text style={styles.unreadBadgeText}>{unreadCount} unread</Text>
        </View>
      )}

      {/* Filtros por tipo */}
      <ScrollView 
        horizontal 
        showsHorizontalScrollIndicator={false}
        style={styles.filtersContainer}
        contentContainerStyle={styles.filtersContent}
      >
        {notificationTypes.map((type) => (
          <TouchableOpacity
            key={type.id}
            style={[
              styles.filterTypeButton,
              selectedType === type.id && styles.filterTypeButtonActive
            ]}
            onPress={() => setSelectedType(type.id)}
          >
            <Ionicons 
              name={type.icon as any} 
              size={16} 
              color={selectedType === type.id ? '#FFFFFF' : '#6B7280'} 
            />
            <Text style={[
              styles.filterTypeText,
              selectedType === type.id && styles.filterTypeTextActive
            ]}>
              {type.title}
            </Text>
          </TouchableOpacity>
        ))}
      </ScrollView>

      {/* Lista de notificaciones */}
      <FlatList
        data={filteredNotifications}
        renderItem={renderNotification}
        keyExtractor={(item) => item.id}
        style={styles.notificationsList}
        contentContainerStyle={styles.notificationsListContent}
        showsVerticalScrollIndicator={false}
        ListEmptyComponent={
          <View style={styles.emptyState}>
            <Ionicons name="notifications-off" size={48} color="#9CA3AF" />
            <Text style={styles.emptyStateTitle}>No notifications</Text>
            <Text style={styles.emptyStateMessage}>
              {showUnreadOnly 
                ? 'You\'re all caught up!' 
                : 'You\'ll see notifications here when you get them'
              }
            </Text>
          </View>
        }
      />
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
  markAllButton: {
    marginRight: 12,
  },
  markAllText: {
    fontSize: 14,
    color: '#8B5CF6',
    fontWeight: '600',
  },
  filterButton: {
    padding: 8,
    borderRadius: 8,
    backgroundColor: '#F3F4F6',
  },
  filterButtonActive: {
    backgroundColor: '#8B5CF6',
  },
  unreadBadge: {
    backgroundColor: '#FEF3C7',
    paddingHorizontal: 16,
    paddingVertical: 8,
    marginHorizontal: 20,
    marginTop: 8,
    borderRadius: 16,
    alignSelf: 'flex-start',
  },
  unreadBadgeText: {
    fontSize: 12,
    fontWeight: '600',
    color: '#92400E',
  },
  filtersContainer: {
    borderBottomWidth: 1,
    borderBottomColor: '#E5E7EB',
  },
  filtersContent: {
    paddingHorizontal: 20,
    paddingVertical: 12,
  },
  filterTypeButton: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingHorizontal: 16,
    paddingVertical: 8,
    marginRight: 12,
    borderRadius: 20,
    backgroundColor: '#F3F4F6',
  },
  filterTypeButtonActive: {
    backgroundColor: '#8B5CF6',
  },
  filterTypeText: {
    fontSize: 14,
    fontWeight: '600',
    color: '#6B7280',
    marginLeft: 6,
  },
  filterTypeTextActive: {
    color: '#FFFFFF',
  },
  notificationsList: {
    flex: 1,
  },
  notificationsListContent: {
    padding: 20,
  },
  notificationItem: {
    marginBottom: 16,
    padding: 16,
    backgroundColor: '#F9FAFB',
    borderRadius: 12,
  },
  unreadNotification: {
    backgroundColor: '#F0F9FF',
    borderLeftWidth: 4,
    borderLeftColor: '#8B5CF6',
  },
  notificationContent: {
    flexDirection: 'row',
    alignItems: 'flex-start',
  },
  notificationHeader: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'flex-start',
  },
  notificationIcon: {
    width: 40,
    height: 40,
    borderRadius: 20,
    backgroundColor: '#F3F4F6',
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: 12,
  },
  notificationText: {
    flex: 1,
    marginRight: 12,
  },
  notificationTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#111827',
    marginBottom: 4,
  },
  notificationMessage: {
    fontSize: 14,
    color: '#6B7280',
    marginBottom: 4,
    lineHeight: 20,
  },
  notificationTime: {
    fontSize: 12,
    color: '#9CA3AF',
  },
  notificationImage: {
    width: 50,
    height: 50,
    borderRadius: 8,
  },
  unreadDot: {
    width: 8,
    height: 8,
    borderRadius: 4,
    backgroundColor: '#8B5CF6',
    marginLeft: 8,
  },
  emptyState: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    paddingVertical: 60,
  },
  emptyStateTitle: {
    fontSize: 18,
    fontWeight: '600',
    color: '#374151',
    marginTop: 16,
    marginBottom: 8,
  },
  emptyStateMessage: {
    fontSize: 14,
    color: '#6B7280',
    textAlign: 'center',
    paddingHorizontal: 40,
  },
}); 