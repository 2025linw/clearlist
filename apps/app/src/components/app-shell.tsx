import { Slot, Stack } from 'expo-router';
import { useState } from 'react';
import { StyleSheet, View } from 'react-native';

import Toasts from '@lib/toast';

import { useSession } from '@contexts/auth';
import { useBreakpoints } from '@hooks/use-breakpoint';

import SearchModal from '@components/modals/search-modal';
import ListNavigator from '@components/navigation/list-navigator';

export default function AppShell() {
  const { hasSession } = useSession();
  const { gtDesktop, gtTablet } = useBreakpoints();

  const [sidebarWidth, setSidebarWidth] = useState(240);

  return (
    <>
      {hasSession ? (
        <>
          {gtTablet ? (
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
                animation: 'default',
              }}
            />
          )}

          <SearchModal />
        </>
      ) : (
        <Stack
          screenOptions={{
            headerShown: false,
            animation: 'none',
          }}
        />
      )}

      <Toasts />
    </>
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
