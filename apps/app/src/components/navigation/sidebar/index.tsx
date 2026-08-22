import { useRouter } from 'expo-router';
import { useRef } from 'react';
import {
  GestureResponderEvent,
  Pressable,
  StyleProp,
  StyleSheet,
  View,
  ViewStyle,
} from 'react-native';

import Button from '@/components/primitives/button';

type SidebarProps = {
  width?: number;
  onWidthChange?: (width: number) => void;
  style?: StyleProp<ViewStyle>;
};

const MIN_WIDTH = 180;
const MAX_WIDTH = 400;

export default function Sidebar({
  width = 240,
  onWidthChange,
  ...props
}: SidebarProps) {
  const router = useRouter();

  const resizing = useRef(false);
  const startX = useRef(0);
  const startWidth = useRef(width);

  function handlePointerDown() {
    resizing.current = true;
    startX.current = width;
    startWidth.current = width;
  }

  function handlePointerMove(e: GestureResponderEvent) {
    if (!resizing.current) {
      return;
    }

    const delta = e.nativeEvent.pageX - startX.current;

    const nextWidth = Math.min(
      MAX_WIDTH,
      Math.max(MIN_WIDTH, startWidth.current + delta),
    );

    if (onWidthChange) onWidthChange(nextWidth);
  }

  function handlePointerUp() {
    resizing.current = false;
  }

  return (
    <View style={styles.wrapper}>
      <View style={[styles.container, props.style, { width }]}>
        <Button onPress={() => router.navigate('/lists/inbox')}>Inbox</Button>
        <Button onPress={() => router.navigate('/lists/today')}>Today</Button>
        <Button onPress={() => router.navigate('/lists/upcoming')}>
          Upcoming
        </Button>
        <Button onPress={() => router.navigate('/lists/deadline')}>
          Deadline
        </Button>

        <Button onPress={() => router.navigate('/settings')}>Setting</Button>
      </View>

      <Pressable
        style={styles.resizeHandle}
        onPressIn={handlePointerDown}
        onPressMove={handlePointerMove}
        onPressOut={handlePointerUp}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  wrapper: {
    position: 'relative',
    height: '100%',
    zIndex: 1,
  },
  container: {
    flex: 1,
  },
  selected: {
    backgroundColor: 'red',
  },
  resizeHandle: {
    position: 'absolute',
    top: '50%',
    right: -25,
    zIndex: 10,

    width: 25,
    height: 80,

    transform: [{ translateY: -40 }],

    borderWidth: 0,

    backgroundColor: 'red',
  },
});
