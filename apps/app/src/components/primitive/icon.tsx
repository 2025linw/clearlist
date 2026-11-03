import Ionicons from '@react-native-vector-icons/ionicons';
import { type ComponentProps } from 'react';
import { type StyleProp, StyleSheet, type TextStyle } from 'react-native';

import { useTheme } from '@contexts/theme';

type IoniconType = ComponentProps<typeof Ionicons>;
export type IconName = IoniconType['name'];
export type IconColor = IoniconType['color'];

export type IconProps = Omit<IoniconType, 'style'> & {
  style?: StyleProp<Omit<TextStyle, 'color' | 'fontSize'>>;
};

export default function Icon({
  size: sizeProp,
  color: colorProp,
  style,
  role,
  testID,
  ...props
}: IconProps) {
  const { components } = useTheme();

  const componentStyle = components.Icon;

  const size = sizeProp ?? componentStyle.size;
  const color = colorProp ?? componentStyle.color;

  const styles = buildStyles(size);

  return (
    <Ionicons
      {...props}
      size={size}
      color={color}
      testID={testID}
      style={[styles.container, style]}
    />
  );
}

function buildStyles(iconSize?: number) {
  return StyleSheet.create({
    container: {
      width: iconSize,
      height: iconSize,

      alignItems: 'center',
      justifyContent: 'center',
    },
  });
}
