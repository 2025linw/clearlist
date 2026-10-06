import { useRouter } from 'expo-router';
import { useCallback, useState } from 'react';
import { StyleSheet, View } from 'react-native';

import { useSessionApi } from '@contexts/auth';
import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';
import { useToast } from '@hooks/use-toast';

import FormField from '@components/forms/form-field';
import Button from '@components/primitives/button';
import TextInput from '@components/primitives/text-input';
import Typography from '@components/primitives/typography';

export type State = 'login' | 'register';

type LoginFormProps = {
  type: State;
};

export default function LoginForm(props: LoginFormProps) {
  const theme = useTheme();
  const styles = buildStyles(theme);
  const router = useRouter();
  const toast = useToast();

  const { createAccount, login: loginApi } = useSessionApi();

  const [isLoading, setLoading] = useState(false);

  const [email, setEmail] = useState('will@email.com');
  const [password, setPassword] = useState('testpass');
  const [isPasswordShown] = useState(false);
  // const [isPasswordShown, setPasswordShown] = useState(false);

  const register = useCallback(
    (info: { email: string; password: string }) => {
      createAccount(info)
        .then(
          (success) => {
            router.setParams({ animation: 'none' });

            if (success) router.replace('/(page)/main');
          },
          (err) => {
            toast.error(`Error registering: ${err}`);
          },
        )
        .finally(() => setLoading(false));
    },
    [router, toast, createAccount],
  );

  const login = useCallback(
    (info: { email: string; password: string }) => {
      loginApi(info)
        .then(
          (success) => {
            router.setParams({ animation: 'none' });

            if (success) router.replace('/(page)/main');
          },
          (err) => {
            toast.error(`Error logging in: ${err}`);
          },
        )
        .finally(() => setLoading(false));
    },
    [router, toast, loginApi],
  );

  return (
    <View style={styles.container}>
      <View style={[styles.box, styles.loginBox]}>
        <View style={styles.inputContainer}>
          <FormField
            label={'Email'}
            style={styles.loginField}
          >
            <TextInput
              style={styles.loginField}
              placeholder="Email"
              value={email}
              onChangeText={setEmail}
              autoCapitalize="none"
              autoComplete="email"
              autoCorrect={false}
            />
          </FormField>

          <FormField
            label={'Password'}
            style={styles.loginField}
          >
            <TextInput
              style={styles.loginField}
              placeholder="Password"
              value={password}
              onChangeText={setPassword}
              autoCapitalize="none"
              autoComplete={
                props.type === 'register' ? 'new-password' : 'current-password'
              }
              autoCorrect={false}
              secureTextEntry={!isPasswordShown}
            />
          </FormField>
        </View>

        <Button
          scheme="primary"
          disabled={isLoading}
          onPress={async () => {
            if (isLoading) return;
            setLoading(true);

            (props.type === 'register' ? register : login)({ email, password });
          }}
          style={styles.button}
        >
          {props.type === 'register' ? 'Register' : 'Login'}
        </Button>
      </View>

      <View style={styles.box}>
        <Typography>
          {props.type === 'register'
            ? 'Have an account?'
            : "Don't have an account?"}
        </Typography>

        <Button
          scheme="tertiary"
          hasBorder
          onPress={() =>
            router.replace(props.type === 'register' ? '/login' : '/register')
          }
          style={styles.button}
        >
          {props.type === 'register' ? 'Login' : 'Register'}
        </Button>
      </View>
    </View>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      width: '100%',
      padding: theme.spacings.x4,

      justifyContent: 'space-between',
    },
    box: {
      borderRadius: theme.rounded.lg,
      padding: theme.spacings.x4,
      gap: theme.spacings.x3,
    },
    loginBox: {
      backgroundColor: theme.palette.subtle,
    },
    inputContainer: {
      padding: theme.spacings.x2,

      justifyContent: 'center',
      alignItems: 'center',
      gap: 10,
    },
    loginField: {
      padding: 5,

      gap: 10,
    },
    button: {
      justifyContent: 'center',
    },
  });
}
