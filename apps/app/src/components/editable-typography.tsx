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
  defaultValue?: string;
  onChangeText?: (value?: string) => void;
  onSave?: (value?: string) => void;
  placeholder?: string;
  editable?: boolean;
  disabled?: boolean;
  multiline?: boolean;
  style?: StyleProp<TextStyle>;
  testID?: string;
  testOnly_editing?: boolean;
};

export default function EditableTypography({
  value,
  defaultValue,
  onChangeText,
  onSave,
  placeholder,
  disabled,
  editable = true,
  ...props
}: EditableTypographyProps) {
  const theme = useTheme();

  const [text, setText] = useState(defaultValue);
  const [editing, setEditing] = useState(props.testOnly_editing ?? false);

  const styles = buildStyles(theme, editing, disabled);

  const displayedText = value ?? text;
  const isEditable = editable && !disabled;

  if (editing) {
    return (
      <TextInput
        value={displayedText ?? ''}
        placeholder={placeholder}
        editable={isEditable}
        disabled={disabled}
        onChangeText={(text: string) => {
          setText(text);

          onChangeText?.(text);
        }}
        onBlur={() => {
          setEditing(props.testOnly_editing ?? false);

          onSave?.(value ?? text);
        }}
        autoFocus={props.testOnly_editing ? false : true}
        multiline={props.multiline}
        style={[styles.input, props.style]}
        testID={props.testID}
      />
    );
  }

  return (
    <Pressable
      onPress={() => {
        setEditing(true);
      }}
      disabled={!isEditable}
    >
      <Typography
        palette={displayedText ? 'text' : 'subtle'}
        style={[styles.input, props.style]}
        testID={props.testID}
      >
        {displayedText || placeholder || ''}
      </Typography>
    </Pressable>
  );
}

function buildStyles(theme: Theme, editing: boolean, disabled?: boolean) {
  const componentStyle = theme.components.EditableTypography;

  const state = disabled ? 'disabled' : editing ? 'editing' : undefined;

  return StyleSheet.create({
    input: {
      ...componentStyle.container,
      ...componentStyle.text,
      ...(state ? componentStyle.state[state] : {}),
    },
  });
}

export function Demo() {
  return (
    /* eslint-disable react-native/no-inline-styles */
    <View style={{ gap: 16, paddingHorizontal: 10 }}>
      <EditableTypography editable={false} />
      <EditableTypography
        value="Has value - display mode"
        editable={false}
      />
      <EditableTypography
        defaultValue="Has default value - display mode"
        editable={false}
      />
      <EditableTypography placeholder="Has placeholder - display mode" />
      <EditableTypography
        value="Disabled - display mode"
        disabled={true}
        editable={false}
      />

      <EditableTypography
        testOnly_editing
        editable={false}
      />
      <EditableTypography
        value="Has value - editing mode"
        testOnly_editing
        editable={false}
      />
      <EditableTypography
        defaultValue="Has default value - editing mode"
        testOnly_editing
        editable={false}
      />
      <EditableTypography
        placeholder="Has placeholder - editing mode"
        testOnly_editing
        editable={false}
      />
      <EditableTypography
        value="Disabled - editing mode"
        disabled={true}
        editable={false}
        testOnly_editing
      />
    </View>
    /* eslint-enable react-native/no-inline-styles */
  );
}
