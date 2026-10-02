import { useTheme } from '@contexts/theme';

import Icon, { type IconName } from '@components/primitives/icon';

type ToastType = 'default' | 'success' | 'error';

type ToastIconProps = {
  type: ToastType;
};

const iconNames: Record<ToastType, IconName> = {
  default: 'information-circle-outline',
  success: 'checkmark-circle-outline',
  error: 'alert-circle-outline',
};

export default function ToastIcon({ type }: ToastIconProps) {
  const theme = useTheme();

  const size = theme.components.Typography.variants.text.fontSize + 4;
  const colors = {
    default: theme.palette.text,
    success: theme.palette.success,
    error: theme.palette.error,
  };

  return (
    <Icon
      name={iconNames[type]}
      size={size}
      color={colors[type]}
    />
  );
}
