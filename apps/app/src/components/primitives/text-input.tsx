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
  disabled?: boolean;
};

export default function TextInput({
  onChangeText,
  style,
  editable,
  disabled,
  ...props
}: TextInputProps) {
  const theme = useTheme();

  const styles = buildStyles(theme, disabled);
  const { color: placeholderTextColor } = styles.placeholder;

  const isEditable = editable && !disabled;

  return (
    <RNTextInput
      {...props}
      onChangeText={onChangeText}
      placeholderTextColor={placeholderTextColor}
      style={[styles.container, styles.text, style]}
      editable={isEditable}
    />
  );
}

function buildStyles(theme: Theme, disabled?: boolean) {
  const componentStyle = theme.components.TextInput;

  return StyleSheet.create({
    container: {
      ...componentStyle.container,
      ...(disabled ? componentStyle.disabled : undefined),
    },
    text: componentStyle.input,
    placeholder: componentStyle.placeholder,
  });
}

export function Demo() {
  return (
    // eslint-disable-next-line
    <View style={{ gap: 16, paddingHorizontal: 10 }}>
      <TextInput editable={false} />
      <TextInput
        value="With initial value"
        editable={false}
      />
      <TextInput
        defaultValue="With initial default value"
        editable={false}
      />

      <TextInput
        value="Disabled"
        disabled
      />
    </View>
  );
}
