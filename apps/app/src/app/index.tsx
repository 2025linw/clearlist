import { Redirect } from 'expo-router';

import { useSession } from '@contexts/auth';

export default function Index() {
  const { hasSession } = useSession();
  if (!hasSession) {
    return <Redirect href="/login" />;
  }

  return <Redirect href="/(page)" />;
}
