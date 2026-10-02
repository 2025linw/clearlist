import { useState } from 'react';
import { Gesture, GestureDetector } from 'react-native-gesture-handler';
import Animated from 'react-native-reanimated';
import { scheduleOnRN } from 'react-native-worklets';

import Icon from '@components/primitives/icon';

type CheckboxProps = {
  checked?: boolean;
  disabled?: boolean;
  onCheck?: () => void;
  onUncheck?: () => void;
};

export default function Checkbox({
  checked = false,
  disabled = false,
  onCheck,
  onUncheck,
}: CheckboxProps) {
  const [localState, setLocalState] = useState(checked);

  const tapGesture = Gesture.Tap()
    .enabled(!disabled)
    .onEnd((_e, success) => {
      if (!success) return;

      const next = !localState;
      scheduleOnRN(setLocalState, next);

      const handler = checked ? onUncheck : onCheck;
      if (handler) {
        scheduleOnRN(handler);
      }
    });

  return (
    <GestureDetector gesture={tapGesture}>
      <Animated.View>
        <Icon
          name={localState ? 'checkbox' : 'square-outline'}
          size={20}
        />
      </Animated.View>
    </GestureDetector>
  );
}
