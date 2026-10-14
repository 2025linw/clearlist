import {
  Toasts as LibToasts,
  type Toast,
  ToastPosition,
} from '@backpackapp-io/react-native-toast';

import { useTheme } from '@contexts/theme';

export default function Toasts() {
  const theme = useTheme();

  const defaultStyle: Toast['styles'] = {
    pressable: {
      ...theme.shadows.high,
    },
    view: {
      backgroundColor: theme.palette.surface,
    },
    text: {
      color: theme.palette.text,
    },
    indicator: {
      display: 'none',
    },
  };

  return (
    <LibToasts
      defaultPosition={ToastPosition.BOTTOM}
      overrideDarkMode={theme.darkMode}
      defaultStyle={defaultStyle}
    />
  );
}
