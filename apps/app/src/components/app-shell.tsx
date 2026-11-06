import { Slot, Stack } from 'expo-router';
import { type PropsWithChildren, useState } from 'react';
import { StyleSheet, View } from 'react-native';

import { useBreakpoints } from '@hooks/use-breakpoint';

import ListNavigator from '@components/navigation/list-navigator';

type AppShellProps = PropsWithChildren;

export default function AppShell({ children }: AppShellProps) {
  const { gtDesktop, gtTablet } = useBreakpoints();

  const [sidebarWidth, setSidebarWidth] = useState(240);

  return gtTablet ? (
      <View style={styles.container}>
        <ListNavigator
          type={gtDesktop ? 'desktop' : 'tablet'}
          width={sidebarWidth}
          onWidthChange={setSidebarWidth}
        />

        <View style={styles.content}>
          <Slot />
        </View>
      </View>
  ) : (
      <Stack
        screenOptions={{
          headerShown: false,
        }}
      />
  );
}
const styles = StyleSheet.create({
  container: {
    flex: 1,
    flexDirection: 'row',
  },
  content: {
    flex: 1,
  },
});
