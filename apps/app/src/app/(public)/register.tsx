import { Redirect } from 'expo-router';

import { useSession } from '@contexts/auth';

import AuthScreen from '#/auth/auth-screen';

export default function RegistrationPage() {
  const { hasSession } = useSession();
  if (hasSession) {
    return <Redirect href="/" />;
  }

  return <AuthScreen type="register" />;
}
