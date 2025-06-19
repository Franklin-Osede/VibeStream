import { StatusBar } from 'expo-status-bar';
import { Slot } from 'expo-router';

// Required polyfills for crypto and URL in React Native
import 'react-native-get-random-values';
import 'react-native-url-polyfill/auto';

// Eliminado inicialización de i18n porque lo estamos manejando más simple
// import './src/localization/config/i18n';

export default function App() {
  return (
    <>
      <StatusBar style="auto" />
      <Slot />
    </>
  );
}
