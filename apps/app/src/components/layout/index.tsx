import { useRouter } from 'expo-router';
import { type PropsWithChildren, type ReactElement } from 'react';
import { type StyleProp, StyleSheet, View, type ViewStyle } from 'react-native';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';
import { useBreakpoints } from '@hooks/use-breakpoint';

import { type IconProps } from '@components/primitives/icon';

import Header from './header';

type LayoutProps = PropsWithChildren<{
  showBackButton?: boolean;
  showOptionButton?: boolean;
  headerText?: string;
  headerIcon?: ReactElement<IconProps>;
  style?: StyleProp<ViewStyle>;
}>;

export default function Layout({
  children,
  showBackButton = false,
  showOptionButton: hasOptions = false,
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
    <View style={styles.container}>
      {hasHeader && (
        <Header
          text={props.headerText}
          icon={props.headerIcon}
        />
      )}

      <View style={[styles.content, props.style]}>{children}</View>
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
  });
}
