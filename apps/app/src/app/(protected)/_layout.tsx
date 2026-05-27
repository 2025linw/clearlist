import { Redirect, Stack } from 'expo-router';

import { useSession } from '@/context/auth';

export default function RootLayout() {
  const { hasSession } = useSession();

  if (!hasSession) {
    return <Redirect href="/login" />;
  }

  return (
    <Stack
      screenOptions={{
        headerShown: false,
      }}
    />
  );
}
