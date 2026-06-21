import { useState } from 'react';
import {
  Pressable,
  StyleProp,
  StyleSheet,
  TextStyle,
  ViewStyle,
} from 'react-native';

import TextInput from '@/components/primitives/text-input';
import Typography from '@/components/primitives/typography';

type Props = {
  value?: string;
  onSave?: (value: string) => void;
  placeholder?: string;
  disabled?: boolean;
  multiline?: boolean;
  style?: StyleProp<TextStyle>;
  containerStyle?: StyleProp<ViewStyle>;
};

export default function EditableTypography({
  value,
  onSave,
  placeholder,
  ...props
}: Props) {
  const [text, setText] = useState(value || '');
  const [editing, setEditing] = useState(false);

  if (editing) {
    return (
      <TextInput
        value={text}
        onChangeText={setText}
        placeholder={placeholder}
        onBlur={() => {
          setEditing(false);

          onSave?.(text);
        }}
        autoFocus
        multiline={props.multiline}
        style={[styles.container, props.style]}
      />
    );
  }

  return (
    <Pressable
      onPress={() => {
        setEditing(true);
      }}
      style={props.containerStyle}
      disabled={props.disabled}
    >
      <Typography
        palette={text ? 'text' : 'subtle'}
        style={props.style}
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
