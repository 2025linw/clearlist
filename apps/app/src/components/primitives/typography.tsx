import { StyleProp, Text, TextStyle, View } from 'react-native';

import { useTheme } from '@/context/theme';
import { TypographyPalettes, TypographyVariants } from '@/context/theme/types';

type TypographyProps = {
  children: string;
  palette?: TypographyPalettes;
  variant?: TypographyVariants;
  style?: StyleProp<TextStyle>;
};

export default function Typography({
  children,
  palette = 'text',
  variant = 'text',
  ...props
}: TypographyProps) {
  const { components } = useTheme();

  const paletteStyle = components.Typography.palette[palette];
  const variantStyle = components.Typography.variants[variant];

  return (
    <Text style={[paletteStyle, variantStyle, props.style]}>{children}</Text>
  );
}

export function Demo() {
  return (
    /* eslint-disable react-native/no-inline-styles */
    <View style={{ gap: 16 }}>
      <Typography>Default Text</Typography>
      <Typography variant="h1">Variant h1</Typography>
      <Typography variant="h2">Variant h2</Typography>
      <Typography variant="h3">Variant h3</Typography>
      <Typography variant="h4">Variant h4</Typography>
      <Typography variant="text">Variant text</Typography>
      <Typography variant="button">Variant button</Typography>

      <Typography palette="text">Palette text</Typography>
      <Typography palette="primary">Palette primary</Typography>
      <Typography palette="subtle">Palette subtle</Typography>
      <Typography palette="danger">Palette danger</Typography>
      <Typography palette="navigation">Palette navigation</Typography>
    </View>
    /* eslint-enable react-native/no-inline-styles */
  );
}
