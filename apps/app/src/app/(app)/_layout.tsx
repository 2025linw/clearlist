import { Slot, Stack } from 'expo-router';

import { useBreakpoints } from '@hooks/use-breakpoint';

import AppShell from '@components/app-shell';

export default function RootLayout() {
  const { gtTablet } = useBreakpoints();

  if (gtTablet) {
    return (
      <AppShell>
        <Slot />
      </AppShell>
    );
  } else {
    return (
      <Stack
        screenOptions={{
          headerShown: false,
        }}
      />
    );
  }
}
