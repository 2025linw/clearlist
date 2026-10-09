import { useState } from 'react';
import {
  Pressable,
  type StyleProp,
  StyleSheet,
  type TextStyle,
} from 'react-native';

import TextInput from '@components/primitives/text-input';
import Typography from '@components/primitives/typography';

type EditableTypographyProps = {
  value?: string;
  onChangeText?: (value?: string) => void;
  onSave?: (value?: string) => void;
  placeholder?: string;
  disabled?: boolean;
  multiline?: boolean;
  style?: StyleProp<TextStyle>;
  testID?: string;
};

export default function EditableTypography({
  value,
  onChangeText,
  onSave,
  placeholder,
  ...props
}: EditableTypographyProps) {
  const [text, setText] = useState(value || '');
  const [editing, setEditing] = useState(false);

  if (editing) {
    return (
      <TextInput
        value={text}
        onChangeText={(text: string) => {
          setText(text);

          onChangeText?.(text);
        }}
        placeholder={placeholder}
        onBlur={() => {
          setEditing(false);

          onSave?.(text);
        }}
        autoFocus
        multiline={props.multiline}
        style={[styles.container, props.style]}
        testID={props.testID}
      />
    );
  }

  return (
    <Pressable
      onPress={() => {
        setEditing(true);
      }}
      disabled={props.disabled}
    >
      <Typography
        palette={text ? 'text' : 'subtle'}
        style={props.style}
        testID={props.testID}
      >
        {text || placeholder || ''}
      </Typography>
    </Pressable>
  );
}

const styles = StyleSheet.create({
  container: {
    padding: 0, // This is to make sure that `multiline` doesn't add paddingTop
  },
});
