import { Redirect, Slot } from 'expo-router';
import { useState } from 'react';
import { StyleSheet, View } from 'react-native';

import { useSession } from '@/context/auth';

import Sidebar from '@/components/navigation/sidebar';

export default function WebRootLayout() {
  const { hasSession } = useSession();

  const [sidebarWidth, setSidebarWidth] = useState(240);

  if (!hasSession) {
    return <Redirect href="/login" />;
  }

  return (
    <View style={style.container}>
      <Sidebar
        width={sidebarWidth}
        onWidthChange={setSidebarWidth}
      />

      <View style={style.content}>
        <Slot />
      </View>
    </View>
  );
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
