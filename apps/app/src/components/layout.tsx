import Ionicons from '@react-native-vector-icons/ionicons/static';
import { useRouter } from 'expo-router';
import { PropsWithChildren, ReactNode } from 'react';
import { Pressable, StyleSheet, View } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';

import { useTheme } from '@/context/theme';

import Typography from '@/components/primitives/typography';

type LayoutProps = PropsWithChildren & {
  showBackButton?: boolean;
  hasOptions?: boolean;
  headerText?: string;
  headerIcon?: ReactNode; // TODO: create Icon node
};

export default function Layout({ children, showBackButton = false, hasOptions = false, ...props }: LayoutProps) {
  const router = useRouter();

  const theme = useTheme();

  const canGoBack = router.canGoBack() && showBackButton;

  return (
    <SafeAreaView
      edges={['top', 'bottom']}
      style={[styles.backdrop, { backgroundColor: theme.palette.background }]}
    >
      {(canGoBack || props.headerText || props.headerIcon) && (
        <View style={styles.header}>
          <View style={styles.headerEle}>
            {canGoBack && (
              <Pressable onPress={() => router.back()}>
                <Ionicons
                  name="arrow-back-circle"
                  size={40}
                  color={theme.palette.navigation}
                />
              </Pressable>
            )}
          </View>

          {props.headerText && <Typography variant="h1">{props.headerText}</Typography>}

          <View style={styles.headerEle}>
            {hasOptions && (
              <Pressable>
                <Ionicons
                  name="ellipsis-horizontal-circle"
                  size={40}
                  color={theme.palette.primary}
                />
              </Pressable>
            )}
          </View>
        </View>
      )}

      <View style={styles.container}>{children}</View>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  backdrop: {
    height: '100%',
    width: '100%',
  },
  header: {
    height: 40,

    paddingHorizontal: 10,

    flexDirection: 'row',
    alignItems: 'center',
    gap: 10,
  },
  headerEle: {
    width: 40,

    alignItems: 'center',
    justifyContent: 'center',
  },
  container: {
    flex: 1,
  },
});
