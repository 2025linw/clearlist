import { ReactNode } from 'react';
import { StyleProp, StyleSheet, TextStyle, View, ViewStyle } from 'react-native';

import Typography from '@/components/primitives/typography';

// TODO: Add label orientation (top or left (default))

export type FormFieldProps = {
  label?: string;
  labelStyle?: TextStyle;
  style?: StyleProp<ViewStyle>;
  children: ReactNode;
};

export default function FormField({ children, ...props }: FormFieldProps) {
  return (
    <View style={[styles.field, props.style]}>
      {props.label && (
        <Typography
          variant="h2"
          style={styles.label}
        >
          {props.label}
        </Typography>
      )}

      <View style={styles.inputContainer}>{children}</View>
    </View>
  );
}

const styles = StyleSheet.create({
  field: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  label: {
    width: 90,

    textAlign: 'center',
  },
  inputContainer: {
    flex: 1,
  },
});
