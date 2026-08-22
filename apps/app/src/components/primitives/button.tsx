import { ReactElement } from 'react';
import { Pressable, PressableProps, StyleSheet, View } from 'react-native';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';

import Icon, { IconProps } from '@/components/icon';
import Typography from '@/components/primitives/typography';

type ButtonSchemes = keyof Omit<
  Theme['components']['Button']['scheme'],
  'disabled'
>;

export type ButtonProps = PressableProps & {
  icon?: ReactElement<IconProps>;
  children?: string;
  scheme?: ButtonSchemes;
};

export default function Button({
  scheme = 'default',
  icon,
  children,
  disabled,
  ...pressableProps
}: ButtonProps) {
  const theme = useTheme();
  const styles = buildStyles(theme, disabled ? 'disabled' : scheme);

  return (
    <Pressable
      {...pressableProps}
      disabled={disabled}
      style={({ pressed }) => [
        styles.container,
        pressed && styles.pressedStyle,
        pressableProps.style,
      ]}
    >
      {icon}

      {children && (
        <Typography
          variant="button"
          style={styles.typography}
          selectable={false}
        >
          {children}
        </Typography>
      )}
    </Pressable>
  );
}

function buildStyles(theme: Theme, scheme: ButtonSchemes | 'disabled') {
  const componentStyle = theme.components.Button;

  return StyleSheet.create({
    container: {
      padding: theme.spacings.lg,

      flexDirection: 'row',
      alignItems: 'center',
      gap: theme.spacings.lg,

      backgroundColor: componentStyle.scheme[scheme].backgroundColor,
      borderWidth: 1,
      borderRadius: theme.rounded.base,
      borderColor: componentStyle.scheme[scheme].borderColor,
    },
    pressedStyle: {},
    typographyContainer: {
      paddingVertical: theme.spacings.lg,
    },
    typography: {
      color: componentStyle.scheme[scheme].color,
      userSelect: 'none',
    },
  });
}

export function Demo() {
  return (
    /* eslint-disable react-native/no-inline-styles */
    <View style={{ gap: 16 }}>
      <Button>Default</Button>
      <Button scheme="primary">Primary</Button>
      <Button scheme="secondary">Secondary</Button>
      <Button scheme="tertiary">Tertiary</Button>
      <Button scheme="danger">Danger</Button>
      <Button disabled>Disabled</Button>
      <Button icon={<Icon name="home-outline" />} />
    </View>
    /* eslint-enable react-native/no-inline-styles */
  );
}
