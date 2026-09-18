import {
  TextInput as RNTextInput,
  TextInputProps as RNTextInputProps,
  StyleProp,
  StyleSheet,
  TextStyle,
} from 'react-native';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';

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
      value={value}
      onChangeText={onChangeText}
      placeholderTextColor={styles.placeholder.color}
      style={[styles.typography, variantStyle, style]}
      {...props}
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
