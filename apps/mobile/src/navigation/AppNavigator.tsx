import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';

// Importar pantallas existentes
import LoginScreen from '../../app/login';
import RoleSelectionScreen from '../../app/role-selection';
import ArtistDashboardScreen from '../presentation/screens/ArtistDashboardScreen';
import FanDashboardScreen from '../presentation/screens/FanDashboardScreen';
import MusicExploreScreen from '../presentation/screens/MusicExploreScreen';

// Tipos para TypeScript
export type RootStackParamList = {
  Login: undefined;
  RoleSelection: { user: any; token: string };
  ArtistDashboard: { user: any; token: string };
  FanDashboard: { user: any; token: string };
  MusicExplore: { user: any; token: string };
};

const Stack = createNativeStackNavigator<RootStackParamList>();

export default function AppNavigator() {
  return (
    <NavigationContainer>
      <Stack.Navigator 
        initialRouteName="Login"
        screenOptions={{
          headerShown: false, // Sin headers por defecto
          animation: 'slide_from_right', // Animación suave
        }}
      >
        <Stack.Screen 
          name="Login" 
          component={LoginScreen} 
        />
        <Stack.Screen 
          name="RoleSelection" 
          component={RoleSelectionScreen} 
        />
        <Stack.Screen 
          name="ArtistDashboard" 
          component={ArtistDashboardScreen} 
        />
        <Stack.Screen 
          name="FanDashboard" 
          component={FanDashboardScreen} 
        />
        <Stack.Screen 
          name="MusicExplore" 
          component={MusicExploreScreen} 
        />
      </Stack.Navigator>
    </NavigationContainer>
  );
} 