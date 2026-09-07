import { StyleSheet } from 'react-native';

import LoginForm, { State } from '@/components/auth/login-form';
import Layout from '@/components/layout';

type LoginScreenProps = {
  type: State;
};

export default function LoginScreen(props: LoginScreenProps) {
  return (
    <Layout
      headerText={props.type === 'login' ? 'Login' : 'Register'}
      style={styles.backdrop}
    >
      <LoginForm type={props.type} />
    </Layout>
  );
}

const styles = StyleSheet.create({
  backdrop: {
    flex: 1,

    padding: '5%',

    justifyContent: 'center',
    alignItems: 'center',
  },
});
