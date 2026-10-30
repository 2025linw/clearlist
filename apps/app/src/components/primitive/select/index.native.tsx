import { BottomSheet, Picker } from '@expo/ui';
import { useState } from 'react';

import Button from '@components/primitive/button';

import { type SelectProps, type SelectValue } from './types';

export default function Select({
  data,
  value,
  defaultValue,
  onValueChange,
  placeholder,
}: SelectProps) {
  const [internalValue, setInternalValue] = useState<SelectValue | ''>(
    defaultValue ?? '',
  );
  const [isPresented, setIsPresented] = useState(false);

  const isControlled = value !== undefined;
  const selectedValue = isControlled ? value : internalValue;

  function handleChange(nextValue: SelectValue | '') {
    if (!isControlled) setInternalValue(nextValue);

    onValueChange?.(nextValue);
  }

  return (
    <>
      <Button onPress={() => setIsPresented(true)}>Open</Button>

      <BottomSheet
        isPresented={isPresented}
        onDismiss={() => setIsPresented(false)}
        // contentPadding={50}
      >
        <Picker
          selectedValue={selectedValue}
          onValueChange={handleChange}
          appearance="wheel"
        >
          <Picker.Item
            label={placeholder ?? '-'}
            value=""
          />

          {data.map(({ label, value }) => (
            <Picker.Item
              key={value}
              label={label ?? value}
              value={value}
            />
          ))}
        </Picker>
      </BottomSheet>
    </>
  );
}
