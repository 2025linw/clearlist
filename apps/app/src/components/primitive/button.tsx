import { type ReactElement, type ReactNode } from 'react';
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

import Icon, { type IconProps } from '@components/primitive/icon';
import Typography from '@components/primitive/typography';

import { type ElementProps } from './types';

type ButtonSchemes = keyof Omit<
  Theme['components']['Button']['scheme'],
  'disabled'
>;

type ButtonContent =
  | {
      // Button with text (optional left and right icons)
      children: ReactNode;
      leftIcon?: ReactElement<IconProps>;
      rightIcon?: ReactElement<IconProps>;
    }
  | {
      // Icon only
      children?: never;
      leftIcon: ReactElement<IconProps>;
      rightIcon?: never;
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

  const iconOnly = !!props.leftIcon && !children;
  const styles = buildStyles(
    theme,
    disabled ? 'disabled' : scheme,
    props.rounded,
    props.hasBorder,
    iconOnly,
    {
      size: props.leftIcon?.props.size,
      color: props.leftIcon?.props.color,
    },
    {
      size: props.rightIcon?.props.size,
      color: props.rightIcon?.props.color,
    },
  );
  const iconComponentStyle = theme.components.Button.icon;
  const leftIcon = props.leftIcon
    ? cloneElement(props.leftIcon, {
        size: props.leftIcon?.props.size ?? iconComponentStyle.size,
        color: styles.leftIcon.color,
      })
    : undefined;
  const rightIcon = props.rightIcon
    ? cloneElement(props.rightIcon, {
        size: props.rightIcon?.props.size ?? iconComponentStyle.size,
        color: styles.leftIcon.color,
      })
    : undefined;

  return (
    <Pressable
      {...props}
      disabled={disabled}
      style={(state) => [
        styles.container,
        (props.testOnly_hovered ||
          (Platform.OS === 'web' &&
            (state as typeof state & { hovered?: boolean }).hovered)) &&
          styles.hoveredStyle,
        state.pressed && styles.pressedStyle,
        typeof style === 'function' ? style(state) : style,
      ]}
      role="button"
    >
      {leftIcon}

      {children && (
        <View style={styles.textContainer}>
          {typeof children === 'string' ? (
            <Typography
              variant="button"
              style={[styles.typography, props.textStyle]}
              selectable={false}
              numberOfLines={1}
              ellipsizeMode="clip"
            >
              {children}
            </Typography>
          ) : (
            children
          )}
        </View>
      )}

      {rightIcon}
    </Pressable>
  );
}

const BORDER_WIDTH = 1;
function buildStyles(
  theme: Theme,
  scheme: ButtonSchemes | 'disabled',
  rounded: boolean | undefined,
  hasBorder: boolean | undefined,
  iconOnly: boolean,
  leftIconStyle: {
    size?: number;
    color?: ColorValue;
  },
  rightIconStyle: {
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
    (leftIconStyle.size ?? componentStyle.icon.size) +
    2 * padding +
    (useBorder ? 2 * BORDER_WIDTH : 0);

  return StyleSheet.create({
    container: {
      padding,

      flexDirection: 'row',
      alignItems: 'center',
      justifyContent: 'space-between',
      gap: theme.spacings.x2,

      borderWidth: useBorder ? BORDER_WIDTH : 0,
      borderRadius: rounded ? theme.rounded.full : theme.rounded.base,
      borderColor: componentScheme.borderColor,

      backgroundColor: componentScheme.backgroundColor,
      ...(iconOnly && {
        width: iconOnlySize,
        height: iconOnlySize,

        justifyContent: 'center',
      }),
    },
    textContainer: {
      flex: 1,
    },
    hoveredStyle: componentScheme.hovered,
    pressedStyle: componentScheme.pressed,
    typography: {
      color: componentStyle.scheme[scheme].color,
      userSelect: 'none',
    },
    leftIcon: {
      color: leftIconStyle.color ?? componentScheme.color,
      fontSize: leftIconStyle.size ?? componentStyle.icon.size,
    },
    rightIcon: {
      color: rightIconStyle.color ?? componentScheme.color,
      fontSize: rightIconStyle.size ?? componentStyle.icon.size,
    },
  });
}

export function Demo() {
  return (
    // eslint-disable-next-line react-native/no-inline-styles
    <View style={{ gap: 16, paddingHorizontal: 10 }}>
      <Button>Default</Button>
      <Button leftIcon={<Icon name="home-outline" />} />
      <Button leftIcon={<Icon name="add" />}>Button with Left Icon</Button>
      <Button
        leftIcon={<Icon name="add" />}
        rightIcon={<Icon name="chevron-forward" />}
      >
        Button with Left and Right Icon
      </Button>

      <Button scheme="primary">Primary</Button>
      <Button scheme="secondary">Secondary</Button>
      <Button scheme="tertiary">Tertiary</Button>

      <Button
        scheme="success"
        leftIcon={<Icon name="checkmark-circle" />}
      >
        Success
      </Button>
      <Button
        scheme="danger"
        leftIcon={<Icon name="warning" />}
      >
        Danger
      </Button>

      <Button disabled>Disabled</Button>
    </View>
  );
}
