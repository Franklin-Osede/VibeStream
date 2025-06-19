// Archivo simplificado para desarrollo - OAuth mock
// TODO: Instalar expo-auth-session y expo-web-browser para OAuth real

export interface GoogleUser {
  id: string;
  email: string;
  name: string;
  picture: string;
  verified_email: boolean;
}

export const useGoogleAuth = () => {
  // Nueva función para simulación (desarrollo)
  const signInWithGoogleMock = async (): Promise<GoogleUser | null> => {
    // Simular delay de OAuth
    await new Promise(resolve => setTimeout(resolve, 1500));
    
    // Datos mock del usuario Google
    return {
      id: `google_${Date.now()}`,
      email: 'usuario.mock@gmail.com',
      name: 'Usuario Mock Google',
      picture: 'https://lh3.googleusercontent.com/a/default-user=s96-c',
      verified_email: true,
    };
  };

  // Función placeholder para OAuth real (TODO: implementar)
  const signInWithGoogle = async (): Promise<GoogleUser | null> => {
    console.log('OAuth real no implementado aún, usando mock');
    return signInWithGoogleMock();
  };

  return {
    signInWithGoogle,
    signInWithGoogleMock,
    request: null, // TODO: implementar con expo-auth-session
    response: null, // TODO: implementar con expo-auth-session
  };
};

// Hook para Microsoft Auth (placeholder)
export const useMicrosoftAuth = () => {
  const signInWithMicrosoft = async (): Promise<GoogleUser | null> => {
    // Simular delay de OAuth
    await new Promise(resolve => setTimeout(resolve, 1500));
    
    return {
      id: `microsoft_${Date.now()}`,
      email: 'usuario.microsoft@outlook.com',
      name: 'Usuario Microsoft',
      picture: 'https://graph.microsoft.com/v1.0/me/photo/$value',
      verified_email: true,
    };
  };

  return {
    signInWithMicrosoft,
    request: null,
    response: null,
  };
};
