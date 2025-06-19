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

interface ArtistDashboardScreenProps {
  navigation: any;
  route: {
    params: {
      user: any;
      token: string;
    };
  };
}

const ArtistDashboardScreen: React.FC<ArtistDashboardScreenProps> = ({ 
  navigation, 
  route 
}) => {
  const [balance, setBalance] = useState(0);
  const [totalEarnings, setTotalEarnings] = useState(0);
  const [activeCampaigns, setActiveCampaigns] = useState(0);
  const theme = useTheme();

  useEffect(() => {
    // Cargar datos del artista
    loadArtistData();
  }, []);

  const loadArtistData = async () => {
    // Aquí cargarías los datos reales del API
    // Por ahora simulamos datos
    setBalance(1250.75);
    setTotalEarnings(5680.90);
    setActiveCampaigns(3);
  };

  const handleUploadMusic = () => {
    Alert.alert(
      'Subir Música',
      'Esta función te permitirá subir música a la blockchain',
      [
        { text: 'Cancelar', style: 'cancel' },
        { text: 'Continuar', onPress: () => console.log('Navegar a upload') }
      ]
    );
  };

  const handleCreateCampaign = () => {
    Alert.alert(
      'Crear Campaña NFT',
      'Crea una campaña promocional como NFT con multiplicadores de recompensas',
      [
        { text: 'Cancelar', style: 'cancel' },
        { text: 'Crear', onPress: () => console.log('Navegar a crear campaña') }
      ]
    );
  };

  const handleCreateRoyaltyShare = () => {
    Alert.alert(
      'Vender Participaciones',
      'Vende participaciones fraccionadas de tus canciones',
      [
        { text: 'Cancelar', style: 'cancel' },
        { text: 'Crear', onPress: () => console.log('Navegar a crear shares') }
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
          <Text style={styles.roleText}>Artista</Text>
        </View>

        {/* Balance Cards */}
        <View style={styles.balanceContainer}>
          <View style={styles.balanceCard}>
            <LinearGradient
              colors={[theme.primary, theme.colors.primaryLight]}
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
              <Text style={styles.statLabel}>Ganancias Totales</Text>
              <Text style={styles.statValue}>${totalEarnings.toFixed(2)}</Text>
            </View>
            <View style={styles.statCard}>
              <Text style={styles.statLabel}>Campañas Activas</Text>
              <Text style={styles.statValue}>{activeCampaigns}</Text>
            </View>
          </View>
        </View>

        {/* Main Actions */}
        <View style={styles.actionsContainer}>
          <Text style={styles.sectionTitle}>Acciones Principales</Text>
          
          {/* Upload Music */}
          <TouchableOpacity style={styles.actionCard} onPress={handleUploadMusic}>
            <LinearGradient
              colors={[theme.colors.glassLight, theme.colors.glassMedium]}
              style={styles.actionGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <Text style={styles.actionIcon}>🎵</Text>
              <View style={styles.actionContent}>
                <Text style={styles.actionTitle}>Subir Música</Text>
                <Text style={styles.actionDescription}>
                  Sube tus canciones a la blockchain y configura royalties automáticos
                </Text>
              </View>
            </LinearGradient>
          </TouchableOpacity>

          {/* Create Campaign NFT */}
          <TouchableOpacity style={styles.actionCard} onPress={handleCreateCampaign}>
            <LinearGradient
              colors={[theme.colors.glassLight, theme.colors.glassMedium]}
              style={styles.actionGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <Text style={styles.actionIcon}>🚀</Text>
              <View style={styles.actionContent}>
                <Text style={styles.actionTitle}>Crear Campaña NFT</Text>
                <Text style={styles.actionDescription}>
                  Lanza campañas promocionales con multiplicadores de recompensas
                </Text>
              </View>
            </LinearGradient>
          </TouchableOpacity>

          {/* Create Royalty Shares */}
          <TouchableOpacity style={styles.actionCard} onPress={handleCreateRoyaltyShare}>
            <LinearGradient
              colors={[theme.colors.glassLight, theme.colors.glassMedium]}
              style={styles.actionGradient}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 1 }}
            >
              <Text style={styles.actionIcon}>💎</Text>
              <View style={styles.actionContent}>
                <Text style={styles.actionTitle}>Vender Participaciones</Text>
                <Text style={styles.actionDescription}>
                  Vende shares fraccionadas de tus canciones a fans/inversores
                </Text>
              </View>
            </LinearGradient>
          </TouchableOpacity>
        </View>

        {/* Recent Activity */}
        <View style={styles.activityContainer}>
          <Text style={styles.sectionTitle}>Actividad Reciente</Text>
          
          <View style={styles.activityCard}>
            <Text style={styles.activityText}>
              • Nueva escucha de "My Song" (+2.5 $VIBERS)
            </Text>
            <Text style={styles.activityTime}>Hace 5 min</Text>
          </View>
          
          <View style={styles.activityCard}>
            <Text style={styles.activityText}>
              • Campaña "Summer Vibes" generó 150 shares
            </Text>
            <Text style={styles.activityTime}>Hace 1 hora</Text>
          </View>
          
          <View style={styles.activityCard}>
            <Text style={styles.activityText}>
              • Royalty share vendida: 5% de "Beat Drop"
            </Text>
            <Text style={styles.activityTime}>Hace 3 horas</Text>
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

export default ArtistDashboardScreen; 