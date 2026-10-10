import { useRouter } from 'expo-router';
import { useState } from 'react';
import { StyleSheet, View } from 'react-native';

import {
  useColorTheme,
  useTheme,
  useThemeContext,
  useThemeMode,
} from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';
import { useBreakpoints } from '@hooks/use-breakpoint';
import { useToast } from '@hooks/use-toast';
import { authClient } from '@lib/auth-client';

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
  const toast = useToast();
  const { getAllColorThemes } = useThemeContext();
  const [colorTheme, setColorTheme] = useColorTheme();
  const [themeMode, setThemeMode] = useThemeMode();
  const { gtTablet } = useBreakpoints();

  const [newPassword, setNewPassword] = useState('');
  const [currentPassword, setCurrentPassword] = useState('');

  const styles = buildStyles(theme, !gtTablet);
  const colorThemes = getAllColorThemes();

  function changePassword() {
    if (newPassword.length < 8) {
      toast.error('New password must be longer than 8 characters');

      return;
    }

    authClient
      .changePassword({
        newPassword: newPassword,
        currentPassword: currentPassword,
      })
      .then(
        (res) => {
          if (res.error) {
            toast.warn('Error in request: ' + res.error.message);
          }
        },
        (err) => {
          toast.error('Error sending request: ' + err);
        },
      )
      .finally(() => {
        setCurrentPassword('');
        setNewPassword('');
      });
  }

  return (
    <Layout
      headerText={'Settings'}
      showBackButton={true}
      style={styles.container}
      scrollable
    >
      <Typography variant="h1">{`Hello, ${user?.name}!`}</Typography>

      <HorizontalDivider />

      <View style={styles.panel}>
        <View>
          <Typography
            variant="h2"
            style={styles.inputHeader}
          >
            Username
          </Typography>
          <TextInput defaultValue={user?.name} />
        </View>

        <View>
          <Typography
            variant="h2"
            style={styles.inputHeader}
          >
            Email
          </Typography>
          <TextInput defaultValue={user?.email} />
        </View>

        <View>
          <Typography
            variant="h2"
            style={styles.inputHeader}
          >
            Password
          </Typography>

          <View style={styles.inputPanel}>
            <TextInput
              value={newPassword}
              onChangeText={setNewPassword}
              placeholder="Enter New Password"
              secureTextEntry
              maxLength={128}
            />

            {!!newPassword && (
              <>
                <TextInput
                  value={currentPassword}
                  onChangeText={setCurrentPassword}
                  placeholder="Enter Current Password"
                  secureTextEntry
                />
                <Button onPress={changePassword}>Change Password</Button>
              </>
            )}
          </View>
        </View>
      </View>

      <HorizontalDivider />

      <View style={styles.panel}>
        <View>
          <Typography
            variant="h2"
            style={styles.inputHeader}
          >
            Theme Mode
          </Typography>

          <View style={styles.inputPanel}>
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
          </View>
        </View>

        <View>
          <Typography
            variant="h2"
            style={styles.inputHeader}
          >
            Color Theme
          </Typography>

          <View style={styles.inputPanel}>
            {colorThemes.map(([colorThemeName, colorValue]) => (
              <Button
                key={colorThemeName}
                scheme={
                  colorTheme === colorThemeName ? 'secondary' : 'tertiary'
                }
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
        </View>
      </View>

      <HorizontalDivider />

      <View style={styles.panel}>
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
    panel: {
      gap: theme.spacings.x4,
    },
    inputHeader: {
      marginBottom: theme.spacings.x1,
    },
    inputPanel: {
      gap: theme.spacings.x2,
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
