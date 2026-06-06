import {
  TextInput as RNTextInput,
  TextInputProps as RNTextInputProps,
  StyleProp,
  TextStyle,
} from 'react-native';

import { useTheme } from '@/context/theme';

type TextInputProps = RNTextInputProps & {
  style?: StyleProp<Pick<TextStyle, 'color'>>;
};

export default function TextInput({ style, ...props }: TextInputProps) {
  const { components } = useTheme();

  const typography = components.Typography;

  return (
    <RNTextInput
      placeholderTextColor={typography.palette.subtle.color}
      style={[typography.palette['text'], typography.variants['text'], style]}
      {...props}
    />
  );
}
