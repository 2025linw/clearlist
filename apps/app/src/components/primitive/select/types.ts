import { type PickerItemValue } from '@expo/ui';

export type SelectValue = PickerItemValue;

type SelectOption = {
  label: string;
  value: SelectValue;
};

export type SelectProps = {
  data: SelectOption[];
  value?: SelectValue;
  defaultValue?: SelectValue;
  onValueChange?: (value: SelectValue | '') => void;
  placeholder?: string;
};
