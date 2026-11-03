import { Picker } from '@expo/ui/community/picker';
import { useState } from 'react';

import { useTheme } from '@contexts/theme';

import { buildStyles } from './style';
import { type SelectProps, type SelectValue } from './types';

export default function Select({
  data,
  value,
  defaultValue,
  onValueChange,
  placeholder,
}: SelectProps) {
  const theme = useTheme();

  const [internalValue, setInternalValue] = useState<SelectValue | ''>(
    defaultValue ?? '',
  );

  const styles = buildStyles(theme);
  const isControlled = value !== undefined;
  const selectedValue = isControlled ? value : internalValue;

  function handleChange(nextValue: SelectValue | '') {
    if (!isControlled) setInternalValue(nextValue);

    onValueChange?.(nextValue);
  }

  return (
    <Picker
      selectedValue={selectedValue}
      onValueChange={handleChange}
      style={styles.container}
    >
      <Picker.Item
        label={placeholder ?? '-'}
        value=""
        style={styles.option}
      />

      {data.map(({ label, value }) => (
        <Picker.Item
          key={value}
          label={label ?? value}
          value={value}
          style={styles.option}
        />
      ))}
    </Picker>
  );
}
