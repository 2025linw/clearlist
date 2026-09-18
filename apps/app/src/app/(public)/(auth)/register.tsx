import { Redirect } from 'expo-router';

import { useSession } from '@/context/auth';

import AuthScreen from '@/screens/auth/auth-screen';

export default function RegistrationPage() {
  const { hasSession } = useSession();
  if (hasSession) {
    return <Redirect href="/" />;
  }

  return <AuthScreen type="register" />;
}
