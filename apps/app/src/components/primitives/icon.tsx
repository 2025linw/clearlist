import Ionicons from '@react-native-vector-icons/ionicons';
import { type ComponentProps } from 'react';
import { type StyleProp, StyleSheet, View, type ViewStyle } from 'react-native';

import { useTheme } from '@contexts/theme';

type IoniconType = ComponentProps<typeof Ionicons>;
export type IconName = IoniconType['name'];
export type IconColor = IoniconType['color'];

export type IconProps = IoniconType & {
  containerStyle?: StyleProp<ViewStyle>;
};

export default function Icon({
  size,
  color,
  style,
  containerStyle,
  role,
  testID,
  ...props
}: IconProps) {
  const { components } = useTheme();

  const componentStyle = components.Icon;

  const flattenedStyle = StyleSheet.flatten(style);
  const resolvedSize = size ?? flattenedStyle?.fontSize ?? componentStyle.size;
  const resolvedColor = color ?? flattenedStyle?.color ?? componentStyle.color;

  const styles = buildStyles(resolvedSize);

  return (
    <View
      style={[styles.container, containerStyle]}
      role={role}
      testID={testID}
    >
      <Ionicons
        {...props}
        size={resolvedSize}
        color={resolvedColor}
        style={[style, { fontSize: resolvedSize, color: resolvedColor }]}
      />
    </View>
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
