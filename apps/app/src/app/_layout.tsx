import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

import { SplashScreen } from 'expo-router';
import { useEffect } from 'react';
import { StyleSheet } from 'react-native';
import { GestureHandlerRootView } from 'react-native-gesture-handler';
import { SafeAreaProvider } from 'react-native-safe-area-context';

import AuthProvider, { useSession } from '@contexts/auth';
import SearchProvider from '@contexts/search';
import ThemeProvider, { useThemeContext } from '@contexts/theme';
import { useAppFonts } from '@hooks/use-fonts';

import AppShell from '@components/app-shell';

SplashScreen.preventAutoHideAsync();

const queryClient = new QueryClient();

function AppInner() {
  const { loaded: authLoaded } = useSession();
  const { loaded: themeLoaded } = useThemeContext();
  const { loaded: fontsLoaded, error: fontError } = useAppFonts();

  // Check when
  useEffect(() => {
    if (themeLoaded && authLoaded && fontsLoaded) {
      SplashScreen.hide();
    }
  }, [themeLoaded, authLoaded, fontsLoaded]);

  if (!(themeLoaded && authLoaded && fontsLoaded)) return null;

  if (fontError) {
    console.error('unable to load fonts');

    return null;
  }

  return (
    <SearchProvider>
      <AppShell />
    </SearchProvider>
  );
}

export default function App() {
  return (
    <SafeAreaProvider>
      <GestureHandlerRootView style={styles.root}>
        <QueryClientProvider client={queryClient}>
          <ThemeProvider>
            <AuthProvider>
              <AppInner />
            </AuthProvider>
          </ThemeProvider>
        </QueryClientProvider>
      </GestureHandlerRootView>
    </SafeAreaProvider>
  );
}

const styles = StyleSheet.create({
  root: {
    flex: 1,
  },
});
