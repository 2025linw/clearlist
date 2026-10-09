import {
  TextInput as RNTextInput,
  type TextInputProps as RNTextInputProps,
  type StyleProp,
  StyleSheet,
  type TextStyle,
  View,
} from 'react-native';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';

type TextInputProps = RNTextInputProps & {
  value?: string;
  onChangeText?: (value: string) => void;
  style?: StyleProp<TextStyle>;
};

export default function TextInput({
  value,
  onChangeText,
  style,
  ...props
}: TextInputProps) {
  const theme = useTheme();

  const styles = buildStyles(theme);
  const { color: placeholderTextColor } = styles.placeholder;

  return (
    <RNTextInput
      {...props}
      value={value}
      onChangeText={onChangeText}
      placeholderTextColor={placeholderTextColor}
      style={[styles.container, styles.text, style]}
    />
  );
}

function buildStyles(theme: Theme) {
  const componentStyle = theme.components.TextInput;

  return StyleSheet.create({
    container: componentStyle.container,
    text: componentStyle.input,
    placeholder: componentStyle.placeholder,
  });
}

export function Demo() {
  return (
    /* eslint-disable react-native/no-inline-styles */
    <View style={{ gap: 16, paddingHorizontal: 10 }}>
      <TextInput />

      <TextInput value="With initial value" />
    </View>
    /* eslint-enable react-native/no-inline-styles */
  );
}
