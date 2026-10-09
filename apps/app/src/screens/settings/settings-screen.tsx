import { useRouter } from 'expo-router';
import { StyleSheet, View } from 'react-native';

import { useColorTheme, useTheme, useThemeMode } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';
import { useBreakpoints } from '@hooks/use-breakpoint';
import { type authClient } from '@lib/auth-client';

import FormField from '@components/forms/form-field';
import Layout from '@components/layout';
import Button from '@components/primitives/button';
import HorizontalDivider from '@components/primitives/horizontal-divider';
import Icon from '@components/primitives/icon';
import TextInput from '@components/primitives/text-input';
import Typography from '@components/primitives/typography';

type SettingsScreenProps = {
  user?: typeof authClient.$Infer.Session.user;
  logout?: () => void;
};

export default function SettingsScreen({ user, logout }: SettingsScreenProps) {
  const router = useRouter();
  const theme = useTheme();
  const [colorTheme, setColorTheme] = useColorTheme();
  const [themeMode, setThemeMode] = useThemeMode();
  const { gtTablet } = useBreakpoints();

  const styles = buildStyles(theme, !gtTablet);

  return (
    <Layout
      headerText={'Settings'}
      showBackButton={true}
      style={styles.container}
    >
      <View>
        <Typography variant="h1">{`Hello, ${user?.name}!`}</Typography>

        <View>
          <Typography variant="h2">Username</Typography>
          <TextInput defaultValue={user?.name} />

          <Typography variant="h2">Email</Typography>
          <TextInput defaultValue={user?.email} />

          <Typography variant="h2">Password</Typography>
          <TextInput
            placeholder="Enter New Password"
            secureTextEntry
          />
        </View>
      </View>

      <HorizontalDivider />

      <View></View>

      <HorizontalDivider />

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

          <Button onPress={() => router.push('/settings/debug')}>
            App Debug
          </Button>
        </>
      )}

      <HorizontalDivider />

      <Button onPress={logout}>Logout</Button>
    </Layout>
  );
}

function buildStyles(theme: Theme, isMobile: boolean) {
  return StyleSheet.create({
    container: {
      paddingHorizontal: isMobile ? theme.spacings.x2 : theme.spacings.x10,
    },
    buttonRow: {
      display: 'flex',
      flexDirection: 'row',
    },
    button: {
      flex: 1,
    },
  });
}
