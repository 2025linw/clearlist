import { Redirect } from 'expo-router';

import { useSession } from '@/context/auth';

import LoginScreen from '@/screens/auth/login-screen';

export default function RegistrationPage() {
  const { hasSession } = useSession();
  if (hasSession) {
    return <Redirect href="/" />;
  }

  return <LoginScreen type="register" />;
}
