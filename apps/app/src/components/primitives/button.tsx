import { ReactNode } from 'react';
import {
  Pressable,
  PressableProps,
  StyleSheet,
  TextStyle,
  View,
  ViewStyle,
} from 'react-native';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';

import Typography from '@/components/primitives/typography';

type ButtonSchemes = keyof Omit<Theme['components']['Button'], 'disabled'>;

export type ButtonProps = PressableProps & {
  text: string;
  scheme?: ButtonSchemes;
  leftIcon?: ReactNode;
};

export default function Button({
  text,
  scheme = 'default',
  leftIcon,
  ...pressableProps
}: ButtonProps) {
  const theme = useTheme();

  const styles = buildStyle(
    theme,
    pressableProps.disabled ? 'disabled' : scheme,
  );

  return (
    <Pressable
      {...pressableProps}
      disabled={pressableProps.disabled}
    >
      <View style={styles.container}>
        {leftIcon && <View style={styles.leftIcon}>{leftIcon}</View>}

        <Typography
          variant="button"
          style={styles.typography}
        >
          {text}
        </Typography>
      </View>
    </Pressable>
  );
}

type ButtonStyle = {
  container: ViewStyle;
  typography: TextStyle;
  leftIcon: ViewStyle;
};

function buildStyle(
  theme: Theme,
  scheme: ButtonSchemes | 'disabled',
): ButtonStyle {
  const componentStyle = theme.components.Button;

  return StyleSheet.create({
    container: {
      flexDirection: 'row',
      alignItems: 'center',
      borderRadius: theme.rounded.base,
      paddingVertical: theme.spacings.lg,
      paddingHorizontal: theme.spacings.xl,

      backgroundColor: componentStyle[scheme].backgroundColor,
      borderColor: componentStyle[scheme].borderColor,
    },
    typography: {
      color: componentStyle[scheme].color,
    },
    leftIcon: {
      marginRight: theme.spacings.base,
    },
  });
}

export function Demo() {
  return (
    /* eslint-disable react-native/no-inline-styles */
    <View style={{ gap: 16 }}>
      <Button text="Default" />
      <Button
        text="Primary"
        scheme="primary"
      />
      <Button
        text="Danger  "
        scheme="danger"
      />
      <Button
        text="Disabled"
        disabled
      />
    </View>
    /* eslint-enable react-native/no-inline-styles */
  );
}
