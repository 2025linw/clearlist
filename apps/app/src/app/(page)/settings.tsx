import { useRouter } from 'expo-router';

import { useSessionApi } from '@contexts/auth';
import { authClient } from '@lib/auth-client';

import SettingsScreen from '@screens/settings/settings-screen';

export default function SettingsPage() {
  const router = useRouter();
  const { logout: logoutApi } = useSessionApi();

  const session = authClient.useSession();

  function logout() {
    logoutApi().finally(() => {
      router.replace('/login');
    });
  }

  return (
    <SettingsScreen
      user={session.data?.user}
      logout={logout}
    />
  );
}
