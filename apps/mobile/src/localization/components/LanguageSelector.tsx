import React, { useState } from 'react';
import {
  View,
  Text,
  TouchableOpacity,
  Modal,
  StyleSheet,
  FlatList,
  Alert,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { useTranslation } from '../hooks/useTranslation';

interface LanguageSelectorProps {
  visible: boolean;
  onClose: () => void;
  onLanguageChange?: (languageCode: string) => void;
}

export const LanguageSelector: React.FC<LanguageSelectorProps> = ({
  visible,
  onClose,
  onLanguageChange,
}) => {
  const { getCurrentLanguage, getSupportedLanguages, changeLanguage, t } = useTranslation();
  const [selectedLanguage, setSelectedLanguage] = useState(getCurrentLanguage());

  const handleLanguageSelect = async (languageCode: string) => {
    try {
      setSelectedLanguage(languageCode);
      await changeLanguage(languageCode);
      onLanguageChange?.(languageCode);
      onClose();
    } catch (error) {
      Alert.alert(
        t('common.error'),
        'Error changing language. Please try again.'
      );
    }
  };

  const renderLanguageItem = ({ item }: { item: any }) => {
    const isSelected = item.code === selectedLanguage;
    
    return (
      <TouchableOpacity
        style={[styles.languageItem, isSelected && styles.selectedLanguageItem]}
        onPress={() => handleLanguageSelect(item.code)}
      >
        <View style={styles.languageContent}>
          <Text style={styles.languageFlag}>{item.flag}</Text>
          <Text style={[styles.languageName, isSelected && styles.selectedLanguageName]}>
            {item.name}
          </Text>
        </View>
        {isSelected && (
          <View style={styles.checkmark}>
            <Text style={styles.checkmarkText}>✓</Text>
          </View>
        )}
      </TouchableOpacity>
    );
  };

  return (
    <Modal
      visible={visible}
      transparent
      animationType="slide"
      onRequestClose={onClose}
    >
      <View style={styles.modalOverlay}>
        <View style={styles.modalContainer}>
          <LinearGradient
            colors={['#6c5ce7', '#a29bfe']}
            style={styles.header}
            start={{ x: 0, y: 0 }}
            end={{ x: 1, y: 0 }}
          >
            <Text style={styles.headerTitle}>
              {t('settings.language')}
            </Text>
            <TouchableOpacity onPress={onClose} style={styles.closeButton}>
              <Text style={styles.closeButtonText}>✕</Text>
            </TouchableOpacity>
          </LinearGradient>

          <View style={styles.content}>
            <FlatList
              data={getSupportedLanguages()}
              renderItem={renderLanguageItem}
              keyExtractor={(item) => item.code}
              showsVerticalScrollIndicator={false}
              style={styles.languageList}
            />
          </View>
        </View>
      </View>
    </Modal>
  );
};

const styles = StyleSheet.create({
  modalOverlay: {
    flex: 1,
    backgroundColor: 'rgba(0, 0, 0, 0.5)',
    justifyContent: 'flex-end',
  },
  modalContainer: {
    backgroundColor: '#1a1a2e',
    borderTopLeftRadius: 20,
    borderTopRightRadius: 20,
    maxHeight: '60%',
    overflow: 'hidden',
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: 20,
  },
  headerTitle: {
    fontSize: 20,
    fontWeight: '700',
    color: '#fff',
  },
  closeButton: {
    width: 30,
    height: 30,
    borderRadius: 15,
    backgroundColor: 'rgba(255, 255, 255, 0.2)',
    justifyContent: 'center',
    alignItems: 'center',
  },
  closeButtonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: 'bold',
  },
  content: {
    padding: 20,
  },
  languageList: {
    maxHeight: 300,
  },
  languageItem: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingVertical: 15,
    paddingHorizontal: 20,
    marginBottom: 10,
    borderRadius: 12,
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderWidth: 1,
    borderColor: 'rgba(255, 255, 255, 0.1)',
  },
  selectedLanguageItem: {
    backgroundColor: 'rgba(108, 92, 231, 0.2)',
    borderColor: '#6c5ce7',
  },
  languageContent: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  languageFlag: {
    fontSize: 24,
    marginRight: 15,
  },
  languageName: {
    fontSize: 16,
    fontWeight: '500',
    color: '#d1d5db',
  },
  selectedLanguageName: {
    color: '#fff',
    fontWeight: '600',
  },
  checkmark: {
    width: 24,
    height: 24,
    borderRadius: 12,
    backgroundColor: '#6c5ce7',
    justifyContent: 'center',
    alignItems: 'center',
  },
  checkmarkText: {
    color: '#fff',
    fontSize: 12,
    fontWeight: 'bold',
  },
}); 