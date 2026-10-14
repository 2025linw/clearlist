import { useEffect, useRef } from 'react';
import {
  TextInput as RNTextInput,
  type TextInputProps as RNTextInputProps,
  StyleSheet,
  View,
} from 'react-native';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';

type TextInputProps = RNTextInputProps & {
  disabled?: boolean;
};

export default function TextInput({
  onChangeText,
  style,
  editable = true,
  disabled,
  ...props
}: TextInputProps) {
  const theme = useTheme();

  const inputRef = useRef<RNTextInput>(null);

  const styles = buildStyles(theme, disabled);
  const { color: placeholderTextColor } = styles.placeholder;

  const isEditable = editable && !disabled;

  useEffect(() => {
    if (disabled) inputRef.current?.blur();
  }, [disabled]);

  return (
    <RNTextInput
      ref={inputRef}
      {...props}
      onChangeText={onChangeText}
      placeholderTextColor={placeholderTextColor}
      style={[styles.container, styles.text, style]}
      editable={isEditable}
      autoFocus={!disabled && props.autoFocus}
      contextMenuHidden={disabled || props.contextMenuHidden}
      caretHidden={disabled || props.caretHidden}
    />
  );
}

function buildStyles(theme: Theme, disabled?: boolean) {
  const componentStyle = theme.components.TextInput;

  return StyleSheet.create({
    container: {
      ...componentStyle.container,
      ...(disabled ? componentStyle.disabled : undefined),
      pointerEvents: disabled ? 'none' : 'auto',
    },
    text: componentStyle.input,
    placeholder: componentStyle.placeholder,
  });
}

export function Demo() {
  return (
    // eslint-disable-next-line react-native/no-inline-styles
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
