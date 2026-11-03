import { type PropsWithChildren, type ReactElement } from 'react';
import {
  ScrollView,
  type StyleProp,
  StyleSheet,
  View,
  type ViewStyle,
} from 'react-native';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';

import { type IconProps } from '@components/primitive/icon';

import { HEADER_HEIGHT } from './constants';
import Header from './header';

type LayoutProps = PropsWithChildren<{
  showBackButton?: boolean;
  showOptionButton?: boolean;
  headerText?: string;
  headerIcon?: ReactElement<IconProps>;
  style?: StyleProp<ViewStyle>;
  safeArea?: boolean;
  scrollable?: boolean;
}>;

export default function Layout({
  children,
  showBackButton,
  showOptionButton,
  safeArea = true,
  scrollable = false,
  ...props
}: LayoutProps) {
  const theme = useTheme();
  const insets = useSafeAreaInsets();

  const hasHeader =
    showBackButton ||
    showOptionButton ||
    !!props.headerText ||
    !!props.headerIcon;
  const styles = buildStyles(theme, safeArea ? insets : undefined, hasHeader);

  return (
    <View style={styles.container}>
      {hasHeader && (
        <Header
          text={props.headerText}
          icon={props.headerIcon}
          showBackButton={showBackButton}
          style={styles.header}
        />
      )}

      {scrollable ? (
        <ScrollView
          style={styles.content}
          contentContainerStyle={[styles.scrollContent, props.style]}
          keyboardShouldPersistTaps="handled"
        >
          {children}
        </ScrollView>
      ) : (
        <View style={[styles.content, props.style]}>{children}</View>
      )}
    </View>
  );
}

function buildStyles(
  theme: Theme,
  insets: ReturnType<typeof useSafeAreaInsets> | undefined,
  hasHeader: boolean | undefined,
) {
  return StyleSheet.create({
    container: {
      flex: 1,

      paddingTop: insets && !hasHeader ? insets.top : 0,

      backgroundColor: theme.palette.background,
    },
    header: {
      paddingTop: insets?.top ?? 0,

      height: HEADER_HEIGHT + (insets?.top ?? 0),
    },
    content: {
      flex: 1,
    },
    scrollContent: {
      flexGrow: 1,
    },
  });
}
