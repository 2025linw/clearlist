import { useRouter } from 'expo-router';
import { type PropsWithChildren, type ReactElement } from 'react';
import { type StyleProp, StyleSheet, View, type ViewStyle } from 'react-native';
import { type Edge, SafeAreaView } from 'react-native-safe-area-context';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';
import { useBreakpoints } from '@contexts/theme/useBreakpoints';

import { type IconProps } from '@components/primitives/icon';

import Header from './header';

type LayoutProps = PropsWithChildren<{
  showBackButton?: boolean;
  showOptionButton?: boolean;
  headerText?: string;
  headerIcon?: ReactElement<IconProps>;
  style?: StyleProp<ViewStyle>;
  safeAreaEdges?: Edge[];
}>;

export default function Layout({
  children,
  showBackButton = false,
  showOptionButton: hasOptions = false,
  safeAreaEdges = ['top', 'bottom'],
  ...props
}: LayoutProps) {
  const router = useRouter();

  const theme = useTheme();
  const styles = buildStyles(theme);
  const { gtTablet } = useBreakpoints();

  const showBack = showBackButton && router.canGoBack() && !gtTablet;
  const hasHeader =
    showBack || props.headerIcon || props.headerText || hasOptions;

  return (
    <SafeAreaView
      edges={safeAreaEdges}
      style={styles.container}
    >
      {hasHeader && (
        <Header
          text={props.headerText}
          icon={props.headerIcon}
          style={styles.header}
        />
      )}

      <View style={[styles.content, props.style]}>{children}</View>
    </SafeAreaView>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      flex: 1,

      backgroundColor: theme.palette.background,
    },
    header: {
      position: 'absolute',
      top: 0,
    },
    content: {
      flex: 1,
    },
    headerButton: {
      width: 56,
      height: 56,

      alignItems: 'center',
      justifyContent: 'center',
    },
    headerItem: {
      alignItems: 'center',
      justifyContent: 'center',
    },
  });
}
