import { useAuthRequest, makeRedirectUri, AuthRequest } from 'expo-auth-session';
import * as WebBrowser from 'expo-web-browser';
import { Platform } from 'react-native';

// Configuración de Google OAuth
const discovery = {
  authorizationEndpoint: 'https://accounts.google.com/o/oauth2/v2/auth',
  tokenEndpoint: 'https://www.googleapis.com/oauth2/v4/token',
  revocationEndpoint: 'https://oauth2.googleapis.com/revoke',
};

// Para desarrollo - reemplazar con tus credenciales reales
const CLIENT_ID = {
  ios: 'YOUR_IOS_CLIENT_ID.googleusercontent.com',
  android: 'YOUR_ANDROID_CLIENT_ID.googleusercontent.com',
  web: 'YOUR_WEB_CLIENT_ID.googleusercontent.com',
};

WebBrowser.maybeCompleteAuthSession();

export interface GoogleUser {
  id: string;
  email: string;
  name: string;
  picture: string;
  verified_email: boolean;
}

export const useGoogleAuth = () => {
  const clientId = Platform.select({
    ios: CLIENT_ID.ios,
    android: CLIENT_ID.android,
    default: CLIENT_ID.web,
  });

  const [request, response, promptAsync] = useAuthRequest(
    {
      clientId: clientId!,
      scopes: ['openid', 'profile', 'email'],
      redirectUri: makeRedirectUri({
        scheme: 'com.vibestream.app', // Cambiar por tu bundle ID
        path: 'redirect',
      }),
    },
    discovery
  );

  const signInWithGoogle = async (): Promise<GoogleUser | null> => {
    try {
      if (!request) {
        console.warn('Google Auth request not ready');
        return null;
      }

      const result = await promptAsync();
      
      if (result.type === 'success' && result.authentication?.accessToken) {
        // Obtener información del usuario
        const userInfoResponse = await fetch(
          `https://www.googleapis.com/oauth2/v2/userinfo?access_token=${result.authentication.accessToken}`
        );
        
        if (userInfoResponse.ok) {
          const userInfo: GoogleUser = await userInfoResponse.json();
          return userInfo;
        }
      }
      
      return null;
    } catch (error) {
      console.error('Error signing in with Google:', error);
      return null;
    }
  };

  return {
    signInWithGoogle,
    request,
    response,
  };
};

// Hook para Microsoft Auth (similar estructura)
export const useMicrosoftAuth = () => {
  const [request, response, promptAsync] = useAuthRequest(
    {
      clientId: 'YOUR_MICROSOFT_CLIENT_ID',
      scopes: ['openid', 'profile', 'email'],
      redirectUri: makeRedirectUri({
        scheme: 'com.vibestream.app',
        path: 'redirect',
      }),
    },
    {
      authorizationEndpoint: 'https://login.microsoftonline.com/common/oauth2/v2.0/authorize',
      tokenEndpoint: 'https://login.microsoftonline.com/common/oauth2/v2.0/token',
    }
  );

  const signInWithMicrosoft = async (): Promise<GoogleUser | null> => {
    try {
      if (!request) return null;

      const result = await promptAsync();
      
      if (result.type === 'success' && result.authentication?.accessToken) {
        const userInfoResponse = await fetch(
          'https://graph.microsoft.com/v1.0/me',
          {
            headers: {
              Authorization: `Bearer ${result.authentication.accessToken}`,
            },
          }
        );
        
        if (userInfoResponse.ok) {
          const userInfo = await userInfoResponse.json();
          return {
            id: userInfo.id,
            email: userInfo.mail || userInfo.userPrincipalName,
            name: userInfo.displayName,
            picture: '', // Microsoft Graph requiere llamada separada para foto
            verified_email: true,
          };
        }
      }
      
      return null;
    } catch (error) {
      console.error('Error signing in with Microsoft:', error);
      return null;
    }
  };

  return {
    signInWithMicrosoft,
    request,
    response,
  };
}; 