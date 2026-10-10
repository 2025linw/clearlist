import { useRouter } from 'expo-router';
import { Keyboard, StyleSheet, View } from 'react-native';

import {
  useColorTheme,
  useTheme,
  useThemeContext,
  useThemeMode,
} from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';
import { useBreakpoints } from '@hooks/use-breakpoint';
import { type authClient } from '@lib/auth-client';

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
  const { getAllColorThemes } = useThemeContext();
  const [colorTheme, setColorTheme] = useColorTheme();
  const [themeMode, setThemeMode] = useThemeMode();
  const { gtTablet } = useBreakpoints();

  const styles = buildStyles(theme, !gtTablet);
  const colorThemes = getAllColorThemes();

  return (
    <Layout
      headerText={'Settings'}
      showBackButton={true}
      style={styles.container}
    >
      <View onTouchEnd={Keyboard.dismiss}>
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

        <View>
          <Typography variant="h2">Theme Mode</Typography>
          <Button
            scheme={themeMode === 'system' ? 'secondary' : 'tertiary'}
            hasBorder
            icon={<Icon name="laptop-outline" />}
            onPress={() => setThemeMode('system')}
          >
            System
          </Button>
          <Button
            scheme={themeMode === 'light' ? 'secondary' : 'tertiary'}
            hasBorder
            icon={<Icon name="sunny" />}
            onPress={() => setThemeMode('light')}
          >
            Light
          </Button>
          <Button
            scheme={themeMode === 'dark' ? 'secondary' : 'tertiary'}
            hasBorder
            icon={<Icon name="moon" />}
            onPress={() => setThemeMode('dark')}
          >
            Dark
          </Button>

          <Typography variant="h2">Color Theme</Typography>
          {colorThemes.map(([colorThemeName, colorValue]) => (
            <Button
              key={colorThemeName}
              scheme={colorTheme === colorThemeName ? 'secondary' : 'tertiary'}
              hasBorder
              icon={
                <Icon
                  name="square"
                  color={colorValue}
                />
              }
              onPress={() => setColorTheme(colorThemeName)}
            >
              {colorThemeName
                .toLowerCase()
                .replace(/(^\w|\s\w)/g, (match) => match.toUpperCase())}
            </Button>
          ))}
        </View>

        <HorizontalDivider />

        {__DEV__ && (
          <Button onPress={() => router.push('/settings/debug')}>
            App Debug
          </Button>
        )}

        <Button onPress={logout}>Logout</Button>
      </View>
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
