import { useRouter } from 'expo-router';
import { ReactElement, cloneElement } from 'react';
import { StyleProp, StyleSheet, View, ViewStyle } from 'react-native';

import { useTheme } from '@contexts/theme';
import { Theme } from '@contexts/theme/types';
import { useBreakpoints } from '@contexts/theme/useBreakpoints';

import Button from '@components/primitives/button';
import Icon, { IconProps } from '@components/primitives/icon';
import Typography from '@components/primitives/typography';

const HEADER_HEIGHT = 56;

const BUTTON_SIZE = 56;
const BUTTON_ICON_SIZE = 40;

type HeaderProps = {
  text?: string;
  icon?: ReactElement<IconProps>;
  style?: StyleProp<ViewStyle>;
};

export default function Header(props: HeaderProps) {
  const router = useRouter();
  const theme = useTheme();
  const styles = buildStyles(theme);
  const { gtTablet } = useBreakpoints();

  const showBack = router.canGoBack() && !gtTablet;

  const icon = props.icon
    ? cloneElement(props.icon, {
        size: theme.components.Typography.variants.h1.fontSize + 5.5,
      })
    : null;

  return (
    <View style={styles.container}>
      <View style={styles.side}>
        {showBack && (
          <Button
            scheme="tertiary"
            rounded
            icon={
              <Icon
                name="arrow-back-circle"
                color={theme.palette.primary}
                size={BUTTON_ICON_SIZE}
              />
            }
            onPress={() => {
              if (!router.canGoBack()) {
                router.dismissTo('/');
                return;
              }

              router.back();
            }}
            style={styles.button}
          />
        )}
      </View>

      <View style={styles.content}>
        {icon}

        {props.text && <Typography variant="h1">{props.text}</Typography>}
      </View>

      <View style={styles.side} />
    </View>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    container: {
      height: HEADER_HEIGHT,

      marginBottom: theme.spacings.x2,

      flexDirection: 'row',
      alignItems: 'center',
      justifyContent: 'space-between',
    },
    side: {
      width: HEADER_HEIGHT,
      aspectRatio: 1,
    },
    content: {
      flexDirection: 'row',
      alignItems: 'center',
      gap: 10,
    },
    button: {
      width: BUTTON_SIZE,
      aspectRatio: 1,
    },
  });
}
