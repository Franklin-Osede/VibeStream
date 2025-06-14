import Constants from 'expo-constants';
import AsyncStorage from '@react-native-async-storage/async-storage';

export interface ApiResponse<T> {
  data: T;
  success: boolean;
  message?: string;
}

export interface ApiError {
  message: string;
  status: number;
  details?: any;
}

export class ApiClient {
  private baseUrl: string;
  private token: string | null = null;

  constructor() {
    // Get API URL from expo config or fallback to localhost
    this.baseUrl = Constants.expoConfig?.extra?.apiUrl || 'http://localhost:3002/api/v1';
    this.loadToken();
  }

  private async loadToken(): Promise<void> {
    try {
      this.token = await AsyncStorage.getItem('@vibestream:token');
    } catch (error) {
      console.warn('Failed to load token from storage:', error);
    }
  }

  private async saveToken(token: string): Promise<void> {
    try {
      this.token = token;
      await AsyncStorage.setItem('@vibestream:token', token);
    } catch (error) {
      console.error('Failed to save token to storage:', error);
    }
  }

  private async removeToken(): Promise<void> {
    try {
      this.token = null;
      await AsyncStorage.removeItem('@vibestream:token');
    } catch (error) {
      console.error('Failed to remove token from storage:', error);
    }
  }

  private getHeaders(): HeadersInit {
    const headers: HeadersInit = {
      'Content-Type': 'application/json',
    };

    if (this.token) {
      headers['Authorization'] = `Bearer ${this.token}`;
    }

    return headers;
  }

  private async handleResponse<T>(response: Response): Promise<T> {
    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}));
      const error: ApiError = {
        message: errorData.message || `HTTP ${response.status}: ${response.statusText}`,
        status: response.status,
        details: errorData
      };
      throw error;
    }

    return response.json();
  }

  async get<T>(endpoint: string): Promise<T> {
    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      method: 'GET',
      headers: this.getHeaders(),
    });

    return this.handleResponse<T>(response);
  }

  async post<T>(endpoint: string, data?: any): Promise<T> {
    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: data ? JSON.stringify(data) : undefined,
    });

    return this.handleResponse<T>(response);
  }

  async put<T>(endpoint: string, data: any): Promise<T> {
    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      method: 'PUT',
      headers: this.getHeaders(),
      body: JSON.stringify(data),
    });

    return this.handleResponse<T>(response);
  }

  async delete<T>(endpoint: string): Promise<T> {
    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      method: 'DELETE',
      headers: this.getHeaders(),
    });

    return this.handleResponse<T>(response);
  }

  // Auth specific methods
  async setAuthToken(token: string): Promise<void> {
    await this.saveToken(token);
  }

  async clearAuthToken(): Promise<void> {
    await this.removeToken();
  }

  getAuthToken(): string | null {
    return this.token;
  }

  isAuthenticated(): boolean {
    return !!this.token;
  }

  // Health check
  async healthCheck(): Promise<{ status: string; service: string }> {
    return this.get('/health');
  }
} 