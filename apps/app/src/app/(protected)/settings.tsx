import { useRouter } from 'expo-router';
import { StyleSheet, View } from 'react-native';

import { useSession, useSessionApi } from '@/context/auth';
import { useThemeMode } from '@/context/theme';

import FormField from '@/components/forms/form-field';
import Layout from '@/components/layout';
import Button from '@/components/primitives/button';
import Typography from '@/components/primitives/typography';

export default function Index() {
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
            text="System"
            scheme={themeMode === 'system' ? 'primary' : 'default'}
            style={styles.button}
            onPress={() => setThemeMode('system')}
          />
          <Button
            text="Light"
            scheme={themeMode === 'light' ? 'primary' : 'default'}
            style={styles.button}
            onPress={() => setThemeMode('light')}
          />
          <Button
            text="Dark"
            scheme={themeMode === 'dark' ? 'primary' : 'default'}
            style={styles.button}
            onPress={() => setThemeMode('dark')}
          />
        </View>
      </FormField>

      <Button
        text="Debug (Text)"
        onPress={() => router.navigate('/settings/typography-debug')}
      />
      <Button
        text="Debug (Button)"
        onPress={() => router.navigate('/settings/button-debug')}
      />
      <Button
        text="Logout"
        onPress={() => logout().finally(() => router.navigate('/login'))}
      />
    </Layout>
  ) : (
    <Layout>
      <Typography>Loading...</Typography>
    </Layout>
  );
}

const styles = StyleSheet.create({
  buttonRow: {
    width: '100%',

    flexDirection: 'row',
    justifyContent: 'center',
  },
  button: {
    flex: 1,
  },
});
