import { type PropsWithChildren, useState } from 'react';
import { StyleSheet, View } from 'react-native';

import { useBreakpoints } from '@hooks/use-breakpoint';

import ListNavigator from '@components/navigation/list-navigator';

type AppShellProps = PropsWithChildren;

export default function AppShell({ children }: AppShellProps) {
  const { gtDesktop } = useBreakpoints();

  const [sidebarWidth, setSidebarWidth] = useState(240);

  return (
    <View style={styles.container}>
      <ListNavigator
        type={gtDesktop ? 'desktop' : 'tablet'}
        width={sidebarWidth}
        onWidthChange={setSidebarWidth}
      />

      <View style={styles.content}>{children}</View>
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
