import {
  type Toast,
  ToastPosition,
  Toasts,
} from '@backpackapp-io/react-native-toast';

import { useTheme } from '@contexts/theme';

export default function ToastRenderer() {
  const theme = useTheme();

  const defaultStyle: Toast['styles'] = {
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
    <Toasts
      defaultPosition={ToastPosition.BOTTOM}
      overrideDarkMode={theme.darkMode}
      defaultStyle={defaultStyle}
    />
  );
}
