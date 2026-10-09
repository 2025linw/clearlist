import { useState } from 'react';
import { type GestureResponderEvent, Pressable, View } from 'react-native';

import { useTheme } from '@contexts/theme';

import Icon from '@components/primitives/icon';

type CheckboxProps = {
  checked?: boolean;
  disabled?: boolean;
  onChange?: (state: boolean) => void;
  testID?: string;
};

export default function Checkbox({
  checked,
  disabled,
  onChange,
  ...props
}: CheckboxProps) {
  const theme = useTheme();

  const [localState, setLocalState] = useState(checked);

  function onPress(e: GestureResponderEvent) {
    e.stopPropagation();

    if (disabled) return;

    const next = !localState;

    onChange?.(next);
    setLocalState(next);
  }

  const iconName = localState
    ? 'checkbox'
    : disabled
      ? 'square'
      : 'square-outline';

  return (
    <Pressable
      role="checkbox"
      disabled={disabled}
      onPress={onPress}
      testID={props.testID}
    >
      <Icon
        name={iconName}
        size={20}
        color={disabled ? theme.palette.subtle : theme.palette.text}
        testID={props.testID ? `${props.testID}-icon` : undefined}
      />
    </Pressable>
  );
}

export function Demo() {
  return (
    // eslint-disable-next-line
    <View style={{ gap: 16, padding: 10 }}>
      <Checkbox />
      <Checkbox checked={true} />
      <Checkbox disabled />
      <Checkbox
        checked={true}
        disabled
      />
    </View>
  );
}
