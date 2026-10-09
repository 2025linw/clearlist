import { type ReactElement } from 'react';
import { cloneElement } from 'react';
import {
  type ColorValue,
  Platform,
  type PressableProps,
  type StyleProp,
  type TextStyle,
} from 'react-native';
import { Pressable, StyleSheet, View } from 'react-native';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';

import Icon, { type IconProps } from '@components/primitives/icon';
import Typography from '@components/primitives/typography';

import { type ElementProps } from './types';

type ButtonSchemes = keyof Omit<
  Theme['components']['Button']['scheme'],
  'disabled'
>;

type ButtonContent =
  | {
      children: string;
      icon?: ReactElement<IconProps>;
    }
  | {
      children?: never;
      icon: ReactElement<IconProps>;
    };

export type ButtonProps = Omit<PressableProps, 'children' | ElementProps> &
  ButtonContent & {
    scheme?: ButtonSchemes;
    hasBorder?: boolean;
    rounded?: boolean;
    textStyle?: StyleProp<TextStyle>;
    testOnly_hovered?: null | boolean | undefined;
  };

export default function Button({
  scheme = 'primary',
  children,
  disabled,
  style,
  ...props
}: ButtonProps) {
  const theme = useTheme();
  const styles = buildStyles(
    theme,
    disabled ? 'disabled' : scheme,
    props.rounded,
    props.hasBorder,
    {
      size: props.icon?.props.size,
      color: props.icon?.props.color,
    },
  );

  const iconComponentStyle = theme.components.Button.icon;
  const icon = props.icon
    ? cloneElement(props.icon, {
        size: props.icon?.props.size ?? iconComponentStyle.size,
        color: styles.icon.color,
      })
    : undefined;

  const iconOnly = !!icon && !children;

  return (
    <Pressable
      {...props}
      disabled={disabled}
      style={(state) => [
        styles.container,
        iconOnly && styles.iconContainer,
        (props.testOnly_hovered || (Platform.OS === 'web' && state.hovered)) &&
          styles.hoveredStyle,
        state.pressed && styles.pressedStyle,
        typeof style === 'function' ? style(state) : style,
      ]}
      role="button"
    >
      {icon}

      {children && (
        <Typography
          variant="button"
          style={[styles.typography, props.textStyle]}
          selectable={false}
        >
          {children}
        </Typography>
      )}
    </Pressable>
  );
}

const BORDER_WIDTH = 1;
function buildStyles(
  theme: Theme,
  scheme: ButtonSchemes | 'disabled',
  rounded: boolean | undefined,
  hasBorder: boolean | undefined,
  iconStyle: {
    size?: number;
    color?: ColorValue;
  },
) {
  const componentStyle = theme.components.Button;
  const componentScheme = componentStyle.scheme[scheme];

  // Default border to true for secondary, otherwise honor hasBorder or false
  const useBorder =
    hasBorder !== undefined ? hasBorder : scheme === 'secondary' ? true : false;

  const padding = theme.spacings.x2;

  // Container size (width and height) for icon only button
  const iconOnlySize =
    (iconStyle.size ?? componentStyle.icon.size) +
    2 * padding +
    (useBorder ? 2 * BORDER_WIDTH : 0);

  return StyleSheet.create({
    container: {
      padding,

      flexDirection: 'row',
      alignItems: 'center',
      gap: theme.spacings.x2,

      borderWidth: useBorder ? BORDER_WIDTH : 0,
      borderRadius: rounded ? theme.rounded.full : theme.rounded.base,
      borderColor: componentScheme.borderColor,

      backgroundColor: componentScheme.backgroundColor,
    },
    iconContainer: {
      width: iconOnlySize,
      height: iconOnlySize,

      justifyContent: 'center',
    },
    hoveredStyle: componentScheme.hovered,
    pressedStyle: componentScheme.pressed,
    typography: {
      color: componentStyle.scheme[scheme].color,
      userSelect: 'none',
    },
    icon: {
      color: iconStyle.color ?? componentStyle.scheme[scheme].color,
    },
  });
}

export function Demo() {
  return (
    // eslint-disable-next-line
    <View style={{ gap: 16, paddingHorizontal: 10 }}>
      <Button>Default</Button>
      <Button icon={<Icon name="home-outline" />} />
      <Button icon={<Icon name="add" />}>Button with Icon</Button>

      <Button scheme="primary">Primary</Button>
      <Button scheme="secondary">Secondary</Button>
      <Button scheme="tertiary">Tertiary</Button>

      <Button
        scheme="success"
        icon={<Icon name="checkmark-circle" />}
      >
        Success
      </Button>
      <Button
        scheme="danger"
        icon={<Icon name="warning" />}
      >
        Danger
      </Button>
      <Button disabled>Disabled</Button>
    </View>
  );
}
