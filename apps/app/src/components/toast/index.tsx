import { View } from 'react-native';

import Icon from '@components/primitives/icon';
import Typography from '@components/primitives/typography';

type ToastProps = {};

export default function Toast(props: ToastProps) {
  return (
    <View style={{ flexDirection: 'row', alignItems: 'center' }}>
      <Icon name="add" />

      <Typography>Test</Typography>
    </View>
  );
}
