import { Redirect } from 'expo-router';

import { useSession } from '@/context/auth';

export default function Index() {
  const { hasSession } = useSession();

  if (hasSession === undefined) {
    return null;
  }

  if (!hasSession) {
    return <Redirect href="/login" />;
  }

  return <Redirect href="/(protected)" />;
}
