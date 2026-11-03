import { useSession } from '@contexts/auth';

import NotFoundScreen from '#/not-found';
import AppShell from '@components/app-shell';

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
