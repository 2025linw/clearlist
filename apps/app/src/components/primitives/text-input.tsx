import {
  TextInput as RNTextInput,
  type TextInputProps as RNTextInputProps,
  type StyleProp,
  StyleSheet,
  type TextStyle,
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
  const variantStyle = theme.components.Typography.variants.text;

  return (
    <RNTextInput
      {...props}
      value={value}
      onChangeText={onChangeText}
      placeholderTextColor={styles.placeholder.color}
      style={[styles.typography, variantStyle, style]}
    />
  );
}

function buildStyles(theme: Theme) {
  const componentStyle = theme.components.TextInput;

  return StyleSheet.create({
    typography: {
      ...componentStyle.input,
    },
    placeholder: {
      ...componentStyle.placeholder,
    },
  });
}
