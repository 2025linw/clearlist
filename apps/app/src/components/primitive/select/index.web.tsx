import { Host, Picker } from '@expo/ui';
import { useState } from 'react';

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

  const isControlled = value !== undefined;
  const selectedValue = isControlled ? value : internalValue;

  function handleChange(nextValue: SelectValue | '') {
    if (!isControlled) setInternalValue(nextValue);

    onValueChange?.(nextValue);
  }

  return (
    <Host>
      <Picker
        selectedValue={selectedValue}
        onValueChange={handleChange}
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
    </Host>
  );
}
