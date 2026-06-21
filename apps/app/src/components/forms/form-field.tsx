import { ReactNode } from 'react';
import {
  StyleProp,
  StyleSheet,
  TextStyle,
  View,
  ViewStyle,
} from 'react-native';

import Typography from '@/components/primitives/typography';

// TODO: Add label orientation (top or left (default))

export type FormFieldProps = {
  label?: string;
  labelStyle?: StyleProp<TextStyle>;
  style?: StyleProp<ViewStyle>;
  children: ReactNode;
};

export default function FormField({ children, ...props }: FormFieldProps) {
  return (
    <View style={styles.container}>
      {props.label && (
        <Typography
          variant="h2"
          style={[styles.label, props.labelStyle]}
        >
          {props.label}
        </Typography>
      )}

      <View style={[styles.content, props.style]}>{children}</View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  label: {
    width: 90,
  },
  content: { flex: 1 },
});
