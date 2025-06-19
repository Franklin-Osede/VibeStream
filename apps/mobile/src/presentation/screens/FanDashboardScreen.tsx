import React, { useState, useEffect } from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  StyleSheet,
  ScrollView,
  Alert,
  Dimensions,
} from 'react-native';
import { StatusBar } from 'expo-status-bar';
import { LinearGradient } from 'expo-linear-gradient';
import { useTheme } from '../../theme';

const { width } = Dimensions.get('window');

interface FanDashboardScreenProps {
  navigation: any;
  route: {
    params: {
      user: any;
      token: string;
    };
  };
}

const FanDashboardScreen: React.FC<FanDashboardScreenProps> = ({ 
  navigation, 
  route 
}) => {
  const [balance, setBalance] = useState(0);
  const [totalEarned, setTotalEarned] = useState(0);
  const [ownedNFTs, setOwnedNFTs] = useState(0);
  const [isListening, setIsListening] = useState(false);
  const theme = useTheme();

  useEffect(() => {
    // Cargar datos del fan
    loadFanData();
  }, []);

  const loadFanData = async () => {
    // Aquí cargarías los datos reales del API
    // Por ahora simulamos datos
    setBalance(850.25);
    setTotalEarned(1200.50);
    setOwnedNFTs(12);
  };

  const handleStartListening = () => {
    navigation.navigate('MusicExplore', {
      user: route.params.user,
      token: route.params.token
    });
  };

  const handleBuyNFT = () => {
    Alert.alert(
      'Comprar NFT',
      'Explora y compra NFTs exclusivos de tus artistas favoritos',
      [
        { text: 'Cancelar', style: 'cancel' },
        { text: 'Explorar', onPress: () => console.log('Navegar a NFT marketplace') }
      ]
    );
  };

  const handleBuyShares = () => {
    Alert.alert(
      'Invertir en Participaciones',
      'Compra shares fraccionadas de canciones y recibe royalties',
      [
        { text: 'Cancelar', style: 'cancel' },
        { text: 'Invertir', onPress: () => console.log('Navegar a shares marketplace') }
      ]
    );
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

      <ScrollView contentContainerStyle={styles.scrollContent}>
        {/* Header */}
        <View style={styles.header}>
          <Text style={styles.welcomeText}>
            ¡Hola, {route.params.user.username}!
          </Text>
          <Text style={styles.roleText}>Fan</Text>
          
          {isListening && (
            <View style={styles.listeningIndicator}>
              <Text style={styles.listeningText}>🎧 Escuchando... +0.5 $VIBERS/min</Text>
            </View>
          )}
        </View>

        {/* Balance Cards */}
        <View style={styles.balanceContainer}>
          <View style={styles.balanceCard}>
            <LinearGradient
              colors={[theme.colors.accentPink, theme.colors.primaryLight]}
              style={styles.balanceGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <Text style={styles.balanceLabel}>Balance $VIBERS</Text>
              <Text style={styles.balanceAmount}>{balance.toFixed(2)}</Text>
            </LinearGradient>
          </View>

          <View style={styles.statsContainer}>
            <View style={styles.statCard}>
              <Text style={styles.statLabel}>Ganado Total</Text>
              <Text style={styles.statValue}>{totalEarned.toFixed(2)}</Text>
            </View>
            <View style={styles.statCard}>
              <Text style={styles.statLabel}>NFTs Poseídos</Text>
              <Text style={styles.statValue}>{ownedNFTs}</Text>
            </View>
          </View>
        </View>

        {/* Main Actions */}
        <View style={styles.actionsContainer}>
          <Text style={styles.sectionTitle}>Acciones Principales</Text>
          
          {/* Listen to Earn */}
          <TouchableOpacity style={styles.actionCard} onPress={handleStartListening}>
            <LinearGradient
              colors={[theme.colors.glassLight, theme.colors.glassMedium]}
              style={styles.actionGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <Text style={styles.actionIcon}>🎧</Text>
              <View style={styles.actionContent}>
                <Text style={styles.actionTitle}>Listen-to-Earn</Text>
                <Text style={styles.actionDescription}>
                  Escucha música y gana $VIBERS automáticamente con pruebas ZK
                </Text>
              </View>
            </LinearGradient>
          </TouchableOpacity>

          {/* Buy NFTs */}
          <TouchableOpacity style={styles.actionCard} onPress={handleBuyNFT}>
            <LinearGradient
              colors={[theme.colors.glassLight, theme.colors.glassMedium]}
              style={styles.actionGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <Text style={styles.actionIcon}>🖼️</Text>
              <View style={styles.actionContent}>
                <Text style={styles.actionTitle}>Comprar NFTs</Text>
                <Text style={styles.actionDescription}>
                  Colecciona NFTs exclusivos y campañas promocionales de artistas
                </Text>
              </View>
            </LinearGradient>
          </TouchableOpacity>

          {/* Buy Royalty Shares */}
          <TouchableOpacity style={styles.actionCard} onPress={handleBuyShares}>
            <LinearGradient
              colors={[theme.colors.glassLight, theme.colors.glassMedium]}
              style={styles.actionGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <Text style={styles.actionIcon}>💰</Text>
              <View style={styles.actionContent}>
                <Text style={styles.actionTitle}>Invertir en Participaciones</Text>
                <Text style={styles.actionDescription}>
                  Compra shares fraccionadas y recibe royalties de canciones
                </Text>
              </View>
            </LinearGradient>
          </TouchableOpacity>
        </View>

        {/* Trending Now */}
        <View style={styles.trendingContainer}>
          <Text style={styles.sectionTitle}>Tendencias</Text>
          
          {/* Trending Campaign */}
          <View style={styles.trendingCard}>
            <LinearGradient
              colors={[theme.primary, theme.colors.primaryLight]}
              style={styles.trendingGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <Text style={styles.trendingIcon}>🔥</Text>
              <View style={styles.trendingContent}>
                <Text style={styles.trendingTitle}>Campaña "Summer Beats"</Text>
                <Text style={styles.trendingDescription}>
                  ×2.5 multiplicador activo • 250 $VIBERS ganados
                </Text>
              </View>
            </LinearGradient>
          </View>
          
          {/* Top Share */}
          <View style={styles.trendingCard}>
            <LinearGradient
              colors={[theme.colors.accentPink, theme.colors.primaryLight]}
              style={styles.trendingGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <Text style={styles.trendingIcon}>📈</Text>
              <View style={styles.trendingContent}>
                <Text style={styles.trendingTitle}>Share "Electric Dreams"</Text>
                <Text style={styles.trendingDescription}>
                  +15% este mes • Disponible: 5% del total
                </Text>
              </View>
            </LinearGradient>
          </View>
        </View>

        {/* Recent Activity */}
        <View style={styles.activityContainer}>
          <Text style={styles.sectionTitle}>Actividad Reciente</Text>
          
          <View style={styles.activityCard}>
            <Text style={styles.activityText}>
              • Escuchaste "Midnight Groove" (+5.2 $VIBERS)
            </Text>
            <Text style={styles.activityTime}>Hace 5 min</Text>
          </View>
          
          <View style={styles.activityCard}>
            <Text style={styles.activityText}>
              • Compraste NFT "Neon Nights Campaign"
            </Text>
            <Text style={styles.activityTime}>Hace 2 horas</Text>
          </View>
          
          <View style={styles.activityCard}>
            <Text style={styles.activityText}>
              • Royalty recibido: 2.1 $VIBERS de "Beat Drop"
            </Text>
            <Text style={styles.activityTime}>Hace 1 día</Text>
          </View>
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
    alignItems: 'center',
    marginBottom: theme.spacing.xl,
  },
  welcomeText: {
    ...theme.styles.titleLarge,
    color: theme.text,
    marginBottom: theme.spacing.sm,
  },
  roleText: {
    ...theme.styles.titleSmall,
    color: theme.colors.accentPink,
    fontWeight: 'bold',
  },
  listeningIndicator: {
    backgroundColor: theme.primary,
    paddingHorizontal: theme.spacing.lg,
    paddingVertical: theme.spacing.sm,
    borderRadius: theme.borderRadius.md,
    marginTop: theme.spacing.md,
  },
  listeningText: {
    color: theme.text,
    fontWeight: 'bold',
    fontSize: 14,
  },
  balanceContainer: {
    marginBottom: theme.spacing.xl,
  },
  balanceCard: {
    borderRadius: theme.borderRadius.xl,
    marginBottom: theme.spacing.lg,
    ...theme.shadows.lg,
  },
  balanceGradient: {
    padding: theme.spacing.xl,
    borderRadius: theme.borderRadius.xl,
    alignItems: 'center',
  },
  balanceLabel: {
    fontSize: 16,
    color: theme.text,
    opacity: 0.8,
    marginBottom: theme.spacing.sm,
  },
  balanceAmount: {
    fontSize: 32,
    color: theme.text,
    fontWeight: 'bold',
  },
  statsContainer: {
    flexDirection: 'row',
    gap: theme.spacing.md,
  },
  statCard: {
    flex: 1,
    backgroundColor: theme.colors.glassLight,
    padding: theme.spacing.lg,
    borderRadius: theme.borderRadius.md,
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
    alignItems: 'center',
  },
  statLabel: {
    fontSize: 12,
    color: theme.textSecondary,
    marginBottom: theme.spacing.sm,
    textAlign: 'center',
  },
  statValue: {
    ...theme.styles.titleMedium,
    color: theme.text,
    fontWeight: 'bold',
  },
  actionsContainer: {
    marginBottom: theme.spacing.xl,
  },
  sectionTitle: {
    ...theme.styles.titleMedium,
    color: theme.text,
    marginBottom: theme.spacing.lg,
    fontWeight: 'bold',
  },
  actionCard: {
    borderRadius: theme.borderRadius.xl,
    marginBottom: theme.spacing.lg,
    ...theme.shadows.md,
  },
  actionGradient: {
    flexDirection: 'row',
    padding: theme.spacing.lg,
    borderRadius: theme.borderRadius.xl,
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
    alignItems: 'center',
  },
  actionIcon: {
    fontSize: 32,
    marginRight: theme.spacing.lg,
  },
  actionContent: {
    flex: 1,
  },
  actionTitle: {
    ...theme.styles.titleSmall,
    color: theme.text,
    marginBottom: theme.spacing.sm,
    fontWeight: 'bold',
  },
  actionDescription: {
    fontSize: 14,
    color: theme.textSecondary,
    lineHeight: 20,
  },
  trendingContainer: {
    marginBottom: theme.spacing.xl,
  },
  trendingCard: {
    borderRadius: theme.borderRadius.xl,
    marginBottom: theme.spacing.lg,
    ...theme.shadows.md,
  },
  trendingGradient: {
    flexDirection: 'row',
    padding: theme.spacing.lg,
    borderRadius: theme.borderRadius.xl,
    alignItems: 'center',
  },
  trendingIcon: {
    fontSize: 28,
    marginRight: theme.spacing.lg,
  },
  trendingContent: {
    flex: 1,
  },
  trendingTitle: {
    ...theme.styles.titleSmall,
    color: theme.text,
    marginBottom: theme.spacing.sm,
    fontWeight: 'bold',
  },
  trendingDescription: {
    fontSize: 14,
    color: theme.text,
    opacity: 0.8,
  },
  activityContainer: {
    marginBottom: theme.spacing.xl,
  },
  activityCard: {
    backgroundColor: theme.colors.glassLight,
    padding: theme.spacing.lg,
    borderRadius: theme.borderRadius.md,
    borderWidth: 1,
    borderColor: theme.colors.glassMedium,
    marginBottom: theme.spacing.md,
  },
  activityText: {
    fontSize: 14,
    color: theme.text,
    marginBottom: theme.spacing.sm,
  },
  activityTime: {
    fontSize: 12,
    color: theme.textMuted,
  },
});

export default FanDashboardScreen; 