import { Slot, Stack } from 'expo-router';
import { useState } from 'react';
import { StyleSheet, View } from 'react-native';

import { useTheme } from '@contexts/theme';
import { useBreakpoints } from '@hooks/use-breakpoint';

import ListNavigator from '@components/navigation/list-navigator';

export default function RootLayout() {
  const theme = useTheme();
  const { gtTablet, gtDesktop } = useBreakpoints();

  const [sidebarWidth, setSidebarWidth] = useState(240);

  if (gtTablet) {
    return (
      <View style={styles.container}>
        <ListNavigator
          mode={gtDesktop ? 'desktop' : 'tablet'}
          width={sidebarWidth}
          onWidthChange={setSidebarWidth}
        />

        <View
          style={[styles.content, { paddingHorizontal: theme.spacings.x16 }]}
        >
          <Slot />
        </View>
      </View>
    );
  } else {
    return (
      <Stack
        screenOptions={{
          headerShown: false,
          contentStyle: {
            paddingHorizontal: theme.spacings.x2,
          },
        }}
      />
    );
  }
}

const styles = StyleSheet.create({
  container: {
    height: '100%',

    flexDirection: 'row',
  },
  content: {
    flex: 1,
  },
});
