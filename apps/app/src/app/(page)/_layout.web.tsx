import { Redirect, Slot } from 'expo-router';
import { useState } from 'react';
import { StyleSheet, View } from 'react-native';

import { useSession } from '@contexts/auth';

import ListNavigator from '@components/navigation/list-navigator';

export default function WebRootLayout() {
  const { hasSession } = useSession();

  const [sidebarWidth, setSidebarWidth] = useState(240);

  if (!hasSession) {
    return <Redirect href="/login" />;
  }

  return (
    <View style={styles.container}>
      <ListNavigator
        mode="desktop"
        width={sidebarWidth}
        onWidthChange={setSidebarWidth}
      />

      <View style={styles.content}>
        <Slot />
      </View>
    </View>
  );
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
