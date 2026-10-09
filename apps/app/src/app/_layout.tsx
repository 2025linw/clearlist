import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

import { SplashScreen, Stack } from 'expo-router';
import { useEffect } from 'react';
import { StyleSheet } from 'react-native';
import { GestureHandlerRootView } from 'react-native-gesture-handler';
import { SafeAreaProvider } from 'react-native-safe-area-context';

import { Provider as AuthProvider, useSession } from '@contexts/auth';
import { Provider as ThemeProvider, useThemeContext } from '@contexts/theme';
import { useAppFonts } from '@hooks/use-fonts';
import ToastRenderer from '@lib/toast';

SplashScreen.preventAutoHideAsync();

const queryClient = new QueryClient();

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

function AppInner() {
  const { loaded: authLoaded } = useSession();
  const { theme, loaded: themeLoaded } = useThemeContext();
  const { fontsLoaded, fontError } = useAppFonts();

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
    <>
      <Stack
        screenOptions={{
          headerShown: false,
          contentStyle: {
            backgroundColor: theme.palette.background,
          },
        }}
      >
        <Stack.Screen
          name="(public)/login"
          options={{ animation: 'none' }}
        />
        <Stack.Screen
          name="(public)/register"
          options={{ animation: 'none' }}
        />

        <Stack.Screen
          name="(page)"
          options={{ title: 'Home' }}
        />
      </Stack>

      <ToastRenderer />
    </>
  );
}

const styles = StyleSheet.create({
  root: {
    flex: 1,
  },
});
