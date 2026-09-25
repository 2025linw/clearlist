import { useRouter } from 'expo-router';
import { StyleSheet, View } from 'react-native';

import { useSession, useSessionApi } from '@contexts/auth';
import { useColorTheme, useThemeMode } from '@contexts/theme';

import FormField from '@components/forms/form-field';
import Layout from '@components/layout';
import Button from '@components/primitives/button';
import HorizontalDivider from '@components/primitives/horizontal-divider';
import Icon from '@components/primitives/icon';
import Typography from '@components/primitives/typography';

export default function SettingsPage() {
  const router = useRouter();
  const { logout } = useSessionApi();
  const { hasSession } = useSession();

  const [colorTheme, setColorTheme] = useColorTheme();
  const [themeMode, setThemeMode] = useThemeMode();

  return hasSession ? (
    <Layout
      headerText={'Settings'}
      showBackButton={true}
    >
      <FormField label="Theme">
        <View style={styles.buttonRow}>
          <Button
            scheme={colorTheme === 'default' ? 'primary' : 'secondary'}
            style={styles.button}
            onPress={() => setColorTheme('default')}
          >
            Default
          </Button>
        </View>
      </FormField>
      <FormField label="Mode">
        <View style={styles.buttonRow}>
          <Button
            icon={<Icon name="laptop-outline" />}
            scheme={themeMode === 'system' ? 'primary' : 'secondary'}
            style={styles.button}
            onPress={() => setThemeMode('system')}
          >
            System
          </Button>
          <Button
            icon={<Icon name="sunny" />}
            scheme={themeMode === 'light' ? 'primary' : 'secondary'}
            style={styles.button}
            onPress={() => setThemeMode('light')}
          >
            Light
          </Button>
          <Button
            icon={<Icon name="moon" />}
            scheme={themeMode === 'dark' ? 'primary' : 'secondary'}
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
          <Button onPress={() => router.navigate('/settings/spacing-debug')}>
            Debug (Spacing)
          </Button>
          <Button
            onPress={() => router.navigate('/settings/notification-debug')}
          >
            Debug (Notification)
          </Button>
          <Button onPress={() => router.navigate('/settings/date-debug')}>
            Debug (Time)
          </Button>
        </>
      )}

      <HorizontalDivider />

      <HorizontalDivider />

      <Button onPress={logout}>Logout</Button>
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
