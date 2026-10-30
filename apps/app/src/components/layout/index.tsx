import { type PropsWithChildren, type ReactElement } from 'react';
import {
  ScrollView,
  type StyleProp,
  StyleSheet,
  View,
  type ViewStyle,
} from 'react-native';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';

import { type IconProps } from '@components/primitive/icon';

import Header from './header';

type LayoutProps = PropsWithChildren<{
  showBackButton?: boolean;
  showOptionButton?: boolean;
  headerText?: string;
  headerIcon?: ReactElement<IconProps>;
  style?: StyleProp<ViewStyle>;
  scrollable?: boolean;
}>;

export default function Layout({
  children,
  showBackButton,
  showOptionButton,
  scrollable = false,
  ...props
}: LayoutProps) {
  const theme = useTheme();
  const styles = buildStyles(theme);

  const hasHeader =
    showBackButton || showOptionButton || props.headerText || props.headerIcon;

  return (
    <View style={styles.container}>
      {hasHeader && (
        <Header
          text={props.headerText}
          icon={props.headerIcon}
          showBackButton={showBackButton}
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

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      flex: 1,

      backgroundColor: theme.palette.background,
    },
    content: {
      flex: 1,
    },
    scrollContent: {
      flexGrow: 1,
    },
  });
}
