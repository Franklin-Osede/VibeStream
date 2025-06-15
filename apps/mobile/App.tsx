import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import LoginScreen from './src/presentation/screens/LoginScreen';
import HomeScreen from './src/presentation/screens/HomeScreen';

// Required polyfills for crypto and URL in React Native
import 'react-native-get-random-values';
import 'react-native-url-polyfill/auto';

// Initialize i18n
import './src/localization/config/i18n';

const Stack = createNativeStackNavigator();

export default function App() {
  return (
    <NavigationContainer>
      <Stack.Navigator screenOptions={{ headerShown: false }}>
        <Stack.Screen name="Login" component={LoginScreen} />
        <Stack.Screen name="Home" component={HomeScreen} />
      </Stack.Navigator>
    </NavigationContainer>
  );
}
