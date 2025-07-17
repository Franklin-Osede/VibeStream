import React from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  StyleSheet,
  Image,
  ScrollView,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { Ionicons } from '@expo/vector-icons';

interface SideMenuProps {
  isVisible: boolean;
  onClose: () => void;
  navigation: any;
  user: any;
}

export default function SideMenu({ isVisible, onClose, navigation, user }: SideMenuProps) {
  if (!isVisible) return null;

  const menuItems = [
    {
      id: 'profile',
      title: 'Mi Perfil',
      icon: 'person',
      onPress: () => navigation.navigate('ProfileScreen'),
    },
    {
      id: 'messages',
      title: 'Mensajes',
      icon: 'chatbubbles',
      onPress: () => navigation.navigate('MessagesScreen'),
    },
    {
      id: 'wallet',
      title: 'Wallet',
      icon: 'wallet',
      onPress: () => navigation.navigate('WalletScreen'),
    },
    {
      id: 'rewards',
      title: 'Recompensas',
      icon: 'gift',
      onPress: () => navigation.navigate('RewardsScreen'),
    },
    {
      id: 'upload',
      title: 'Subir Contenido',
      icon: 'cloud-upload',
      onPress: () => navigation.navigate('UploadScreen'),
    },
    {
      id: 'settings',
      title: 'Configuración',
      icon: 'settings',
      onPress: () => navigation.navigate('SettingsScreen'),
    },
  ];

  return (
    <View style={styles.overlay}>
      <TouchableOpacity style={styles.backdrop} onPress={onClose} />
      <View style={styles.menuContainer}>
        <LinearGradient
          colors={['#0F0F1E', '#1A1A2E']}
          style={styles.menuGradient}
        >
          {/* Profile Section */}
          <View style={styles.profileSection}>
            <Image 
              source={{ uri: user?.avatar || 'https://via.placeholder.com/80' }} 
              style={styles.profileImage} 
            />
            <Text style={styles.profileName}>
              {user?.displayName || 'I love all music genres'}
            </Text>
            <Text style={styles.profileHandle}>
              @{user?.handle || 'musicworld10'}
            </Text>
            <View style={styles.profileStats}>
              <Text style={styles.statText}>0 Seguidores</Text>
              <Text style={styles.statText}>6 Siguiendo</Text>
            </View>
          </View>

          {/* Menu Items */}
          <ScrollView style={styles.menuItems} showsVerticalScrollIndicator={false}>
            {menuItems.map((item) => (
              <TouchableOpacity
                key={item.id}
                style={styles.menuItem}
                onPress={() => {
                  item.onPress();
                  onClose();
                }}
              >
                <Ionicons name={item.icon as any} size={24} color="#94A3B8" />
                <Text style={styles.menuItemText}>{item.title}</Text>
              </TouchableOpacity>
            ))}
          </ScrollView>

          {/* VibeStream Features */}
          <View style={styles.featuresSection}>
            <Text style={styles.featuresTitle}>Funcionalidades VibeStream</Text>
            
            <TouchableOpacity 
              style={styles.featureItem}
              onPress={() => {
                navigation.navigate('TradingScreen');
                onClose();
              }}
            >
              <Ionicons name="trending-up" size={20} color="#6C5CE7" />
              <Text style={styles.featureText}>Trading Fraccional</Text>
            </TouchableOpacity>

            <TouchableOpacity 
              style={styles.featureItem}
              onPress={() => {
                navigation.navigate('NFTMarketplace');
                onClose();
              }}
            >
              <Ionicons name="diamond" size={20} color="#00D4FF" />
              <Text style={styles.featureText}>NFT Marketplace</Text>
            </TouchableOpacity>

            <TouchableOpacity 
              style={styles.featureItem}
              onPress={() => {
                navigation.navigate('VREventsScreen');
                onClose();
              }}
            >
              <Ionicons name="vr" size={20} color="#FF6B6B" />
              <Text style={styles.featureText}>Eventos VR</Text>
            </TouchableOpacity>
          </View>

          {/* App Branding */}
          <View style={styles.brandingSection}>
            <View style={styles.logoContainer}>
              <View style={styles.logoIcon}>
                <Text style={styles.logoText}>V</Text>
              </View>
              <Text style={styles.brandText}>VIBESTREAM</Text>
            </View>
          </View>
        </LinearGradient>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  overlay: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    zIndex: 1000,
  },
  backdrop: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    backgroundColor: 'rgba(0,0,0,0.5)',
  },
  menuContainer: {
    position: 'absolute',
    top: 0,
    left: 0,
    width: width * 0.8,
    height: '100%',
  },
  menuGradient: {
    flex: 1,
    paddingTop: 60,
  },
  profileSection: {
    alignItems: 'center',
    paddingHorizontal: 20,
    paddingBottom: 30,
    borderBottomWidth: 1,
    borderBottomColor: '#2D2D3A',
  },
  profileImage: {
    width: 80,
    height: 80,
    borderRadius: 40,
    marginBottom: 15,
  },
  profileName: {
    fontSize: 18,
    fontWeight: '600',
    color: '#FFFFFF',
    textAlign: 'center',
    marginBottom: 5,
  },
  profileHandle: {
    fontSize: 14,
    color: '#94A3B8',
    marginBottom: 15,
  },
  profileStats: {
    flexDirection: 'row',
    gap: 20,
  },
  statText: {
    fontSize: 14,
    color: '#94A3B8',
  },
  menuItems: {
    flex: 1,
    paddingHorizontal: 20,
    paddingTop: 20,
  },
  menuItem: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingVertical: 15,
    paddingHorizontal: 10,
    borderRadius: 10,
    marginBottom: 5,
  },
  menuItemText: {
    fontSize: 16,
    color: '#FFFFFF',
    marginLeft: 15,
    fontWeight: '500',
  },
  featuresSection: {
    paddingHorizontal: 20,
    paddingVertical: 20,
    borderTopWidth: 1,
    borderTopColor: '#2D2D3A',
  },
  featuresTitle: {
    fontSize: 14,
    fontWeight: '600',
    color: '#6C5CE7',
    marginBottom: 15,
    textTransform: 'uppercase',
    letterSpacing: 1,
  },
  featureItem: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingVertical: 12,
    paddingHorizontal: 10,
    borderRadius: 8,
    marginBottom: 8,
    backgroundColor: 'rgba(108, 92, 231, 0.1)',
  },
  featureText: {
    fontSize: 14,
    color: '#FFFFFF',
    marginLeft: 12,
    fontWeight: '500',
  },
  brandingSection: {
    paddingHorizontal: 20,
    paddingVertical: 20,
    borderTopWidth: 1,
    borderTopColor: '#2D2D3A',
  },
  logoContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
  },
  logoIcon: {
    width: 30,
    height: 30,
    borderRadius: 15,
    backgroundColor: '#6C5CE7',
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: 10,
  },
  logoText: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#FFFFFF',
  },
  brandText: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#6C5CE7',
  },
}); 