import { useTheme } from '@contexts/theme';

import Icon, { type IconName } from '@components/primitives/icon';

type ToastType = 'default' | 'success' | 'warn' | 'error';

type ToastIconProps = {
  type: ToastType;
};

const iconNames: Record<ToastType, IconName> = {
  default: 'information-circle-outline',
  success: 'checkmark-circle-outline',
  warn: 'alert-circle-outline',
  error: 'close-circle-outline',
};

export default function ToastIcon({ type }: ToastIconProps) {
  const theme = useTheme();

  const componentStyle = theme.components.Toast;

  return (
    <Icon
      name={iconNames[type]}
      size={componentStyle.size}
      color={componentStyle.palette[type].color}
    />
  );
}
