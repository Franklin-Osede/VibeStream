import React from 'react';
import { Stack } from 'expo-router';

export default function RootLayout() {
  return (
    <Stack>
      <Stack.Screen name="login" options={{ headerShown: false }} />
      <Stack.Screen name="role-selection" options={{ headerShown: false }} />
      <Stack.Screen name="artist-dashboard" options={{ headerShown: false }} />
      <Stack.Screen name="fan-dashboard" options={{ headerShown: false }} />
    </Stack>
  );
} 