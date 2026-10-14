import { useState } from 'react';
import { type GestureResponderEvent, Pressable, View } from 'react-native';

import { useTheme } from '@contexts/theme';

import Icon from '@components/primitives/icon';

type CheckboxProps = {
  value?: boolean;
  defaultValue?: boolean;
  onValueChange?: (state: boolean) => void;
  disabled?: boolean;
  testID?: string;
};

export default function Checkbox({
  value,
  defaultValue,
  disabled,
  onValueChange,
  ...props
}: CheckboxProps) {
  const theme = useTheme();

  const [checked, setChecked] = useState(defaultValue);

  const checkedResolved = value ?? checked;

  function onPress(e: GestureResponderEvent) {
    e.stopPropagation();

    if (disabled) return;

    const next = !checked;

    onValueChange?.(next);
    setChecked(next);
  }

  const iconName = checkedResolved
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
    // eslint-disable-next-line react-native/no-inline-styles
    <View style={{ gap: 16, padding: 10 }}>
      <Checkbox />
      <Checkbox value={true} />
      <Checkbox defaultValue={true} />
      <Checkbox disabled />
      <Checkbox
        value={true}
        disabled
      />
    </View>
  );
}
