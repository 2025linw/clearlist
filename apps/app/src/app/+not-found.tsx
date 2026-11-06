import { useSession } from '@contexts/auth';

import AppShell from '@components/app-shell';

import NotFoundScreen from '@screens/not-found';

export default function NotFoundPage() {
  const { hasSession, loaded } = useSession();

  if (!loaded) return null;

  if (hasSession) {
    return (
      <AppShell>
        <NotFoundScreen />
      </AppShell>
    );
  }

  return <NotFoundScreen />;
}
