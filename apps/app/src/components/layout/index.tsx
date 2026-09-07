import { useRouter } from 'expo-router';
import { PropsWithChildren, ReactElement, cloneElement } from 'react';
import { StyleProp, StyleSheet, View, ViewStyle } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';
import { useBreakpoints } from '@/context/theme/useBreakpoints';

import Icon, { IconProps } from '@/components/icon';
import Button from '@/components/primitives/button';
import Typography from '@/components/primitives/typography';

type LayoutProps = PropsWithChildren<{
  showBackButton?: boolean;
  hasOptions?: boolean;
  headerText?: string;
  headerIcon?: ReactElement<IconProps>;
  style?: StyleProp<ViewStyle>;
}>;

export default function Layout({
  children,
  showBackButton = false,
  hasOptions = false,
  ...props
}: LayoutProps) {
  const router = useRouter();

  const theme = useTheme();
  const styles = buildStyles(theme);
  const { gtTablet } = useBreakpoints();

  const showBack = (showBackButton && router.canGoBack()) || !gtTablet;
  const hasHeader = showBack || props.headerText || props.headerIcon;

  let headerIcon = undefined;
  if (props.headerIcon) {
    headerIcon = cloneElement(props.headerIcon, {
      size: theme.components.Typography.variants.h1.fontSize + 5.5,
    });
  }

  return (
    <SafeAreaView
      edges={['top', 'bottom']}
      style={styles.layoutContainer}
    >
      {hasHeader && (
        <View style={styles.header}>
          {showBack && (
            <Button
              scheme="tertiary"
              icon={
                <Icon
                  name="arrow-back-circle"
                  color={theme.palette.navigation}
                  size={40}
                />
              }
              onPress={() => {
                if (!router.canGoBack()) {
                  router.dismissTo('/');
                  return;
                }

                router.back();
              }}
              style={styles.headerButton}
            />
          )}

          {headerIcon}

          {props.headerText && (
            <Typography variant="h1">{props.headerText}</Typography>
          )}

          {hasOptions && (
            <Button
              scheme="tertiary"
              icon={
                <Icon
                  name="ellipsis-horizontal-circle"
                  color={theme.palette.primary}
                  size={40}
                />
              }
              style={styles.headerButton}
            />
          )}
        </View>
      )}

      <View style={[styles.container, props.style]}>{children}</View>
    </SafeAreaView>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    layoutContainer: {
      flex: 1,

      padding: theme.spacings.x4,

      backgroundColor: theme.palette.background,
    },
    header: {
      height: 40,

      marginBottom: theme.spacings.x2,

      flexDirection: 'row',
      alignItems: 'center',
      gap: 10,
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
    container: {
      flex: 1,
    },
  });
}
