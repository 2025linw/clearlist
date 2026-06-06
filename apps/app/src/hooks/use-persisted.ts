import { useEffect, useState } from 'react';

import { ColorTheme, ThemeMode } from '@/context/theme/types';

import * as Storage from '@/lib/storage';

type StorageSchema = {
  systemTheme: 'system' | ThemeMode;
  colorTheme: ColorTheme;
};
const storageDefaults: StorageSchema = {
  systemTheme: 'system',
  colorTheme: 'default',
};

export default function usePersisted<K extends keyof StorageSchema>(key: K) {
  const [value, setValue] = useState<StorageSchema[K]>(storageDefaults[key]);
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    Storage.getItem(key).then((stored) => {
      if (stored !== null) {
        try {
          const parsed = JSON.parse(stored);

          setValue(parsed);
        } catch {
          console.error('Invalid format found in storage');

          setValue(storageDefaults[key]);
        }
      }

      setLoaded(true);
    });
  }, [key]);

  function setPersisted(val: StorageSchema[K]) {
    setValue(val);

    Storage.setItem(key, JSON.stringify(val)).catch((e) => {
      console.error('Failed to persist', e);
    });
  }

  return { value, setValue: setPersisted, loaded } as const;
}
