import Ionicons from '@react-native-vector-icons/ionicons';
import { type ComponentProps } from 'react';
import { type StyleProp, StyleSheet, View, type ViewStyle } from 'react-native';

import { useTheme } from '@contexts/theme';

type IoniconType = ComponentProps<typeof Ionicons>;
export type IconName = IoniconType['name'];
export type IconColor = IoniconType['color'];

export type IconProps = IoniconType & {
  style?: StyleProp<ViewStyle>;
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
    <View
      style={[styles.container, style]}
      role={role}
      testID={testID}
    >
      <Ionicons
        {...props}
        size={size}
        color={color}
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
