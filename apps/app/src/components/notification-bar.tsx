import { Pressable, StyleSheet } from 'react-native';

import Typography from '@/components/primitives/typography';

type NotificationBarProps = { visible: boolean; message: string; onClose?: () => void };

export default function NotificationBar({ visible, message, onClose }: NotificationBarProps) {
  if (!visible) return;

  return (
    <Pressable
      style={styles.modal}
      onPress={onClose}
    >
      <Typography>{message}</Typography>
    </Pressable>
  );
}

const styles = StyleSheet.create({
  modal: {
    position: 'absolute',
    left: 16,
    right: 16,
    bottom: 48,
    zIndex: 9999,

    padding: 12,
    borderRadius: 12,
  },
});
