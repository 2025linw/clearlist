import {
  TextInput as RNTextInput,
  TextInputProps as RNTextInputProps,
  StyleProp,
  StyleSheet,
  TextStyle,
  View,
  ViewStyle,
} from 'react-native';

import { useTheme } from '@/context/theme';
import { Theme } from '@/context/theme/types';

type TextInputProps = RNTextInputProps & {
  style?: StyleProp<TextStyle>;
  containerStyle?: StyleProp<ViewStyle>;
};

export default function TextInput({ style, ...props }: TextInputProps) {
  const theme = useTheme();

  const styles = buildStyle(theme);

  return (
    <View style={styles.container}>
      <RNTextInput
        placeholderTextColor={styles.placeholder.color}
        style={[styles.typography, style]}
        {...props}
      />
    </View>
  );
}

type TextInputStyle = {
  container: ViewStyle;
  typography: TextStyle;
  placeholder: TextStyle;
};

function buildStyle(theme: Theme): TextInputStyle {
  const componentStyle = theme.components.TextInput;

  return StyleSheet.create({
    container: {
      borderRadius: theme.rounded.base,
      paddingVertical: theme.spacings.base,
      paddingHorizontal: theme.spacings.lg,

      backgroundColor: theme.palette.surface,
      borderColor: theme.palette.border,
    },
    typography: {
      ...componentStyle.input,
    },
    placeholder: {
      ...componentStyle.placeholder,
    },
  });
}
