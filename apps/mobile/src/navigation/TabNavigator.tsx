import React, { useEffect } from 'react';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import { Ionicons } from '@expo/vector-icons';

// Importar pantallas desde el barril
import { 
  HomeScreen, 
  TrendingScreen, 
  ExploreScreen, 
  LibraryScreen, 
  NotificationsScreen 
} from '../screens';
import { useUser } from '../contexts/UserContext';
import { TabParamList, User } from '../types';

const Tab = createBottomTabNavigator<TabParamList>();

interface TabNavigatorProps {
  route: {
    params: {
      user: User;
      token: string;
    };
  };
}

export default function TabNavigator({ route }: TabNavigatorProps) {
  const { setUser, setToken } = useUser();
  const { user, token } = route.params;

  useEffect(() => {
    // Configurar el usuario en el contexto cuando se monta el componente
    setUser(user);
    setToken(token);
  }, [user, token, setUser, setToken]);

  return (
    <Tab.Navigator
      screenOptions={({ route }) => ({
        tabBarIcon: ({ focused, color, size }) => {
          let iconName: keyof typeof Ionicons.glyphMap;

          if (route.name === 'Home') {
            iconName = focused ? 'home' : 'home-outline';
          } else if (route.name === 'Trending') {
            iconName = focused ? 'trending-up' : 'trending-up-outline';
          } else if (route.name === 'Explore') {
            iconName = focused ? 'search' : 'search-outline';
          } else if (route.name === 'Library') {
            iconName = focused ? 'library' : 'library-outline';
          } else if (route.name === 'Notifications') {
            iconName = focused ? 'notifications' : 'notifications-outline';
          } else {
            iconName = 'help-outline';
          }

          return <Ionicons name={iconName} size={size} color={color} />;
        },
        tabBarActiveTintColor: '#8B5CF6', // Purple para VibeStream
        tabBarInactiveTintColor: '#6B7280',
        tabBarStyle: {
          backgroundColor: '#FFFFFF',
          borderTopWidth: 1,
          borderTopColor: '#E5E7EB',
          paddingBottom: 5,
          paddingTop: 5,
          height: 60,
        },
        tabBarLabelStyle: {
          fontSize: 12,
          fontWeight: '500',
        },
        headerShown: false,
      })}
    >
      <Tab.Screen 
        name="Home" 
        component={HomeScreen}
        options={{
          tabBarLabel: 'Your Feed',
        }}
      />
      <Tab.Screen 
        name="Trending" 
        component={TrendingScreen}
        options={{
          tabBarLabel: 'Trending',
        }}
      />
      <Tab.Screen 
        name="Explore" 
        component={ExploreScreen}
        options={{
          tabBarLabel: 'Explore',
        }}
      />
      <Tab.Screen 
        name="Library" 
        component={LibraryScreen}
        options={{
          tabBarLabel: 'Library',
        }}
      />
      <Tab.Screen 
        name="Notifications" 
        component={NotificationsScreen}
        options={{
          tabBarLabel: 'Notifications',
        }}
      />
    </Tab.Navigator>
  );
} 
 