import React, { useEffect } from 'react';
import { router } from 'expo-router';

export default function Index() {
  useEffect(() => {
    // Redirigir automáticamente al login cuando se carga la app
    router.replace('/login');
  }, []);

  return null; // No renderiza nada ya que redirige inmediatamente
} 