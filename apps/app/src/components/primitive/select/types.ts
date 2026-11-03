import { type PickerItemValue } from '@expo/ui';

export type SelectValue = PickerItemValue;

type SelectOption = {
  label: string;
  value: SelectValue;
};

type MobileSelectType = 'drawer' | 'screen';

export type SelectProps = {
  data: SelectOption[];
  value?: SelectValue;
  defaultValue?: SelectValue;
  onValueChange?: (value: SelectValue | '') => void;
  placeholder?: string;
  label?: string;
  type?: MobileSelectType;
};
