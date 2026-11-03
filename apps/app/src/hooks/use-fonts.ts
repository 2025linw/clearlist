import { useFonts } from 'expo-font';
import { Platform } from 'react-native';

const webFonts = {
  'Inter-Regular': require('../../assets/fonts/Inter-Regular.otf'),
  'Inter-Italic': require('../../assets/fonts/Inter-Italic.otf'),
  'Inter-Medium': require('../../assets/fonts/Inter-Medium.otf'),
  'Inter-Bold': require('../../assets/fonts/Inter-Bold.otf'),
  'Inter-BoldItalic': require('../../assets/fonts/Inter-BoldItalic.otf'),
  'Inter-Black': require('../../assets/fonts/Inter-Black.otf'),
};

export function useAppFonts() {
  const [loaded, error] = useFonts(Platform.OS === 'web' ? webFonts : {});

  return {
    fontsLoaded: Platform.OS !== 'web' || loaded,
    fontError: error,
  };
}
