import { getTimeZones } from '@vvo/tzdb';

import { useCalendars } from 'expo-localization';

import Select from '@components/primitive/select';

type TimezoneSelectProps = {
  value: string | undefined;
  onValueChange: (value: string | undefined) => void;
};

export default function TimezoneSelect({
  value,
  onValueChange,
}: TimezoneSelectProps) {
  const calendars = useCalendars();

  return (
    <Select
      data={[
        {
          value: 'default',
          label: 'Use device defaults - ' + calendars[0].timeZone,
        },
        ...getTimeZones().map((value) => ({
          value: value.name,
          label: value.name.replaceAll('_', ' '),
        })),
      ]}
      value={value}
      onValueChange={(value) => onValueChange(value as string)}
      label="Timezone"
      placeholder="Choose a timezone"
      type="screen"
    />
  );
}
