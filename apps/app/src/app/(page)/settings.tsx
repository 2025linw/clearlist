import { useRouter } from 'expo-router';
import { StyleSheet, View } from 'react-native';

import { useSession, useSessionApi } from '@/context/auth';
import { useThemeMode } from '@/context/theme';

import FormField from '@/components/forms/form-field';
import Layout from '@/components/layout';
import Button from '@/components/primitives/button';
import HorizontalDivider from '@/components/primitives/horizontal-divider';
import Typography from '@/components/primitives/typography';

export default function SettingsPage() {
  const router = useRouter();
  const { logout } = useSessionApi();
  const { hasSession } = useSession();
  const [themeMode, setThemeMode] = useThemeMode();

  return hasSession ? (
    <Layout
      headerText={'Settings'}
      showBackButton={true}
    >
      <FormField label="Mode">
        <View style={styles.buttonRow}>
          <Button
            scheme={themeMode === 'system' ? 'primary' : 'default'}
            style={styles.button}
            onPress={() => setThemeMode('system')}
          >
            System
          </Button>
          <Button
            scheme={themeMode === 'light' ? 'primary' : 'default'}
            style={styles.button}
            onPress={() => setThemeMode('light')}
          >
            Light
          </Button>
          <Button
            scheme={themeMode === 'dark' ? 'primary' : 'default'}
            style={styles.button}
            onPress={() => setThemeMode('dark')}
          >
            Dark
          </Button>
        </View>
      </FormField>

      {__DEV__ && (
        <>
          <HorizontalDivider />

          <Button onPress={() => router.navigate('/settings/typography-debug')}>
            Debug (Text)
          </Button>
          <Button onPress={() => router.navigate('/settings/button-debug')}>
            Debug (Button)
          </Button>
          <Button
            onPress={() => router.navigate('/settings/notification-debug')}
          >
            Debug (Notification)
          </Button>
        </>
      )}

      <HorizontalDivider />

      <Button onPress={() => logout().finally(() => router.navigate('/login'))}>
        Logout
      </Button>
    </Layout>
  ) : (
    <Layout>
      <Typography>Loading...</Typography>
    </Layout>
  );
}

const styles = StyleSheet.create({
  buttonRow: {
    display: 'flex',
    flexDirection: 'row',
    // justifyContent: 'space-evenly',
    // alignItems: 'stretch',
  },
  button: {
    flex: 1,
  },
});
