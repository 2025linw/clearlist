import { StyleSheet, Text, View } from 'react-native';

import LoginForm, { State } from '@components/auth/login-form';
import Layout from '@components/layout';

import { API_URL } from '@/constants';

type AuthScreenProps = {
  type: State;
};

export default function AuthScreen(props: AuthScreenProps) {
  return (
    <Layout
      headerText={props.type === 'login' ? 'Login' : 'Register'}
      style={styles.backdrop}
    >
      <LoginForm type={props.type} />

      {__DEV__ && (
        <View style={styles.footer}>
          <Text>{API_URL}</Text>
        </View>
      )}
    </Layout>
  );
}

const styles = StyleSheet.create({
  backdrop: {
    padding: '5%',

    justifyContent: 'center',
    alignItems: 'center',
  },
  footer: {
    position: 'absolute',
    bottom: 0,
  },
});
