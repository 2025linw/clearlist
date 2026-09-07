import { Redirect, Stack } from 'expo-router';
import { useState } from 'react';
import { StyleSheet, View } from 'react-native';

import { useSession } from '@/context/auth';
import { useBreakpoints } from '@/context/theme/useBreakpoints';

import ListNavigator from '@/components/navigation/list-navigator';

export default function RootLayout() {
  const { hasSession } = useSession();

  const { gtTablet } = useBreakpoints();

  const [sidebarWidth, setSidebarWidth] = useState(240);

  if (!hasSession) {
    return <Redirect href="/login" />;
  }

  if (gtTablet) {
    return (
      <View style={style.container}>
        <ListNavigator
          mode="tablet"
          width={sidebarWidth}
          onWidthChange={setSidebarWidth}
        />

        <View style={style.content}>
          <Stack
            screenOptions={{
              headerShown: false,
              animation: 'none',
            }}
          />
        </View>
      </View>
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

const style = StyleSheet.create({
  container: {
    height: '100%',

    flexDirection: 'row',
  },
  content: {
    flex: 1,
  },
});
