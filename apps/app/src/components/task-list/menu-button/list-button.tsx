import { StyleSheet } from 'react-native';
import { Gesture, GestureDetector } from 'react-native-gesture-handler';
import Animated from 'react-native-reanimated';
import { scheduleOnRN } from 'react-native-worklets';

import { useTheme } from '@contexts/theme';
import { type Theme } from '@contexts/theme/types';

import Icon from '@components/primitives/icon';

import { BUTTON_SIZE, ICON_SIZE } from './constants';

type ListButtonProps = {
  onAddTask?: () => void;
};

export default function ListButton({ onAddTask }: ListButtonProps) {
  const theme = useTheme();
  const styles = buildStyles(theme);

  const tapGesture = Gesture.Tap().onEnd((_e, success) => {
    if (onAddTask && success) scheduleOnRN(onAddTask);
  });

  return (
    <GestureDetector gesture={tapGesture}>
      <Animated.View style={styles.button}>
        <Icon
          name="add"
          size={ICON_SIZE}
          color={styles.icon.color}
        />
      </Animated.View>
    </GestureDetector>
  );
}

function buildStyles(theme: Theme) {
  return StyleSheet.create({
    button: {
      width: BUTTON_SIZE,
      aspectRatio: 1,

      borderRadius: theme.rounded.full,

      alignItems: 'center',
      justifyContent: 'center',

      backgroundColor: theme.palette.primary,
      color: '#fff',
    },
    icon: {
      color: '#fff',
    },
  });
}
