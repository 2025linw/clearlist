import { useRouter } from 'expo-router';
import { useState } from 'react';
import { StyleSheet, View } from 'react-native';

import { useSessionApi } from '@/context/auth';
import { useTheme } from '@/context/theme';

import FormField from '@/components/forms/form-field';
import Button from '@/components/primitives/button';
import TextInput from '@/components/primitives/text-input';
import Typography from '@/components/primitives/typography';
import Box from '@/components/styling/box';

export type State = 'login' | 'register';

type Props = {
  type: State;
};

export default function LoginForm(props: Props) {
  const router = useRouter();
  const theme = useTheme();
  const { createAccount, login } = useSessionApi();

  const [isLoading, setLoading] = useState(false);
  const [errorText, setErrorText] = useState('');

  const [email, setEmail] = useState('will@email.com');
  const [password, setPassword] = useState('testpass');
  const [isPasswordFocused, setPasswordFocused] = useState(false);

  return (
    <View style={styles.container}>
      <Box style={[styles.loginBox, { backgroundColor: theme.palette.subtle }]}>
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
              autoComplete="new-password"
              autoCorrect={false}
              secureTextEntry={!isPasswordFocused}
              onFocus={() => setPasswordFocused(true)}
              onBlur={() => setPasswordFocused(false)}
            />
          </FormField>
        </View>

        <View style={styles.loginField}>
          <Button
            text={props.type === 'login' ? 'Login' : 'Register'}
            scheme="primary"
            disabled={isLoading}
            onPress={async () => {
              if (isLoading) return;
              setLoading(true);

              try {
                if (props.type === 'login') {
                  await login({ email, password });
                } else {
                  await createAccount({ email, password });
                }

                router.replace('/');
              } catch (e) {
                setErrorText(`${JSON.stringify(e)}`);
              } finally {
                setLoading(false);
              }
            }}
          />
        </View>
      </Box>

      <Button
        text={
          props.type === 'login'
            ? "Don't have an account? Register"
            : 'Have an account? Login'
        }
        onPress={() =>
          router.replace(props.type === 'login' ? '/register' : '/login')
        }
      />

      <View style={styles.msgBox}>
        <Typography palette="danger">{errorText}</Typography>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    width: '100%',
    padding: 25,

    justifyContent: 'space-between',
  },
  loginBox: {
    padding: 25,
  },
  inputContainer: {
    justifyContent: 'center',
    alignItems: 'center',

    gap: 10,
  },
  loginField: {
    padding: 5,
    gap: 10,
  },
  msgBox: {
    height: '10%',
  },
});
