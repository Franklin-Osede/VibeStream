import React from 'react';
import { StatusBar } from 'expo-status-bar';
import 'expo-linking'; // Importación explícita para asegurar que se registre

// Required polyfills for crypto and URL in React Native
import 'react-native-get-random-values';
import 'react-native-url-polyfill/auto';

// Importar el navigator principal
import AppNavigator from './src/navigation/AppNavigator';

export default function App() {
  return (
    <>
      <StatusBar style="auto" />
      <AppNavigator />
    </>
  );
}
