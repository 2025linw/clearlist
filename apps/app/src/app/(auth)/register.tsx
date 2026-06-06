import { Redirect } from 'expo-router';

import { useSession } from '@/context/auth';

import LoginScreen from '@/screens/login-screen';

export default function LoginPage() {
  const { hasSession } = useSession();
  if (hasSession) {
    return <Redirect href="/" />;
  }

  return <LoginScreen type="register" />;
}
