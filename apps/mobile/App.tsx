import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import LoginScreen from './src/presentation/screens/LoginScreen';
import HomeScreen from './src/presentation/screens/HomeScreen';
import OnboardingScreen from './src/presentation/screens/OnboardingScreen';

// Required polyfills for crypto and URL in React Native
import 'react-native-get-random-values';
import 'react-native-url-polyfill/auto';

// Initialize i18n
import './src/localization/config/i18n';

const Stack = createNativeStackNavigator();
const Tab = createBottomTabNavigator();

// Main tab navigator after login
function MainTabNavigator() {
  return (
    <Tab.Navigator screenOptions={{ headerShown: false }}>
      <Tab.Screen 
        name="Home" 
        component={HomeScreen} 
        options={{
          tabBarIcon: () => '🏠',
          tabBarLabel: 'Home'
        }}
      />
      <Tab.Screen 
        name="Music" 
        component={HomeScreen} // Temporal, reemplazar con MusicPlayerScreen
        options={{
          tabBarIcon: () => '🎵',
          tabBarLabel: 'Music'
        }}
      />
      <Tab.Screen 
        name="Wallet" 
        component={HomeScreen} // Temporal, reemplazar con WalletScreen
        options={{
          tabBarIcon: () => '💰',
          tabBarLabel: 'Wallet'
        }}
      />
      <Tab.Screen 
        name="Profile" 
        component={HomeScreen} // Temporal, reemplazar con ProfileScreen
        options={{
          tabBarIcon: () => '👤',
          tabBarLabel: 'Profile'
        }}
      />
    </Tab.Navigator>
  );
}

export default function App() {
  return (
    <NavigationContainer>
      <Stack.Navigator screenOptions={{ headerShown: false }}>
        <Stack.Screen name="Login" component={LoginScreen} />
        <Stack.Screen name="Onboarding" component={OnboardingScreen} />
        <Stack.Screen name="Main" component={MainTabNavigator} />
      </Stack.Navigator>
    </NavigationContainer>
  );
}
