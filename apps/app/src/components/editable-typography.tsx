import { useState } from 'react';
import {
  Pressable,
  type StyleProp,
  StyleSheet,
  type TextStyle,
  View,
} from 'react-native';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';

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
  testOnly_editing?: boolean;
};

export default function EditableTypography({
  value,
  onChangeText,
  onSave,
  placeholder,
  ...props
}: EditableTypographyProps) {
  const theme = useTheme();

  const [text, setText] = useState(value ?? '');
  const [editing, setEditing] = useState(props.testOnly_editing ?? false);

  const styles = buildStyles(theme, editing);

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
          setEditing(props.testOnly_editing ?? false);

          onSave?.(text);
        }}
        autoFocus={props.testOnly_editing ? false : true}
        multiline={props.multiline}
        style={[styles.container, styles.text, props.style]}
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
      style={styles.container}
    >
      <Typography
        palette={text ? 'text' : 'subtle'}
        style={[styles.text, props.style]}
        testID={props.testID}
      >
        {text || placeholder || ''}
      </Typography>
    </Pressable>
  );
}

function buildStyles(theme: Theme, editing: boolean) {
  const componentStyle = theme.components.EditableTypography;

  const { borderColor, ...container } = componentStyle.container;

  return StyleSheet.create({
    container: {
      ...container,
      borderColor: editing ? borderColor : 'transparent',
    },
    text: componentStyle.input,
  });
}

export function Demo() {
  return (
    /* eslint-disable react-native/no-inline-styles */
    <View style={{ gap: 16, paddingHorizontal: 10 }}>
      <EditableTypography />
      <EditableTypography value="With initial value" />

      <EditableTypography testOnly_editing />
      <EditableTypography
        value="With initial value"
        testOnly_editing
      />
    </View>
    /* eslint-enable react-native/no-inline-styles */
  );
}
