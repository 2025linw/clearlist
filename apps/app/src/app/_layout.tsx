import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

import { SplashScreen, Stack } from 'expo-router';
import { useEffect } from 'react';
import { StyleSheet } from 'react-native';
import { GestureHandlerRootView } from 'react-native-gesture-handler';
import { SafeAreaProvider } from 'react-native-safe-area-context';

import { Provider as AuthProvider, useSession } from '@/context/auth';
import { Provider as ErrorProvider } from '@/context/error';
import { Provider as ThemeProvider, useThemeContext } from '@/context/theme';

SplashScreen.preventAutoHideAsync();

// Intialize TanStack Query in global scope
const queryClient = new QueryClient();

export default function App() {
  return (
    <GestureHandlerRootView style={styles.rootContainer}>
      <SafeAreaProvider>
        <QueryClientProvider client={queryClient}>
          <ThemeProvider>
            <ErrorProvider>
              <AuthProvider>
                <AppInner />
              </AuthProvider>
            </ErrorProvider>
          </ThemeProvider>
        </QueryClientProvider>
      </SafeAreaProvider>
    </GestureHandlerRootView>
  );
}

function AppInner() {
  const { loaded: authLoaded } = useSession();
  const { loaded: themeLoaded } = useThemeContext();

  // Check when
  useEffect(() => {
    if (themeLoaded && authLoaded) {
      SplashScreen.hide();
    }
  }, [themeLoaded, authLoaded]);
  if (!themeLoaded) return null;

  return <Stack screenOptions={{ headerShown: false }} />;
}

const styles = StyleSheet.create({
  rootContainer: {
    flex: 1,
  },
});
