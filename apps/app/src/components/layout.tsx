import { useRouter } from 'expo-router';
import { PropsWithChildren, ReactNode } from 'react';
import { StyleProp, StyleSheet, View, ViewStyle } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';

import Button from '@/components/primitives/button';
import Typography from '@/components/primitives/typography';

type LayoutProps = PropsWithChildren & {
  showBackButton?: boolean;
  hasOptions?: boolean;
  headerText?: string;
  headerIcon?: ReactNode; // TODO: create Icon node,
  style?: StyleProp<ViewStyle>;
};

export default function Layout({
  children,
  showBackButton = false,
  hasOptions = false,
  ...props
}: LayoutProps) {
  const theme = useTheme();
  const styles = buildStyles(theme);
  const router = useRouter();

  const canGoBack = router.canGoBack() && showBackButton;

  return (
    <SafeAreaView
      edges={['top', 'bottom']}
      style={[StyleSheet.absoluteFill, styles.layoutContainer]}
    >
      {(canGoBack || props.headerText || props.headerIcon) && (
        <View style={styles.header}>
          <View style={styles.headerEle}>
            {canGoBack && (
              <Button
                iconName="arrow-back-circle"
                iconSize={40}
                iconColor={theme.palette.navigation}
                onPress={() => router.back()}
              />
            )}
          </View>

          {props.headerText && (
            <Typography variant="h1">{props.headerText}</Typography>
          )}

          <View style={styles.headerEle}>
            {hasOptions && (
              <Button
                iconName="ellipsis-horizontal-circle"
                iconSize={40}
                iconColor={theme.palette.primary}
                onPress={() => router.back()}
              />
            )}
          </View>
        </View>
      )}

      <View style={[styles.container, props.style]}>{children}</View>
    </SafeAreaView>
  );
}

function buildStyles(theme: Theme) {
  const styles = StyleSheet.create({
    layoutContainer: {
      backgroundColor: theme.palette.background,
    },
    header: {
      height: 40,

      marginBottom: theme.spacings.lg,
      paddingHorizontal: theme.spacings.lg,

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

      padding: 10,
    },
  });

  return styles;
}
