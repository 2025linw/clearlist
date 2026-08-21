import { Redirect, Slot } from 'expo-router';
import { StyleSheet, View } from 'react-native';

import { useSession } from '@/context/auth';

import Sidebar from '@/components/sidebar';

export default function WebRootLayout() {
  const { hasSession } = useSession();
  if (!hasSession) {
    return <Redirect href="/login" />;
  }

  return (
    <View style={style.container}>
      <Sidebar style={style.sidenav} />

      <View style={style.main}>
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
  sidenav: {
    width: '20%',
  },
  main: {
    width: '80%',
  },
});
