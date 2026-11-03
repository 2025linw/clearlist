import { StyleSheet } from 'react-native';

import { type Theme } from '@contexts/theme/types';

export function buildStyles(theme: Theme) {
  const componentStyles = theme.components.Select;

  return StyleSheet.create({
    container: componentStyles.container,
    option: componentStyles.option,
  });
}
