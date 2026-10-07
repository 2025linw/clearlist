import { toast } from '@backpackapp-io/react-native-toast';
import { type ValueOrFunction } from '@backpackapp-io/react-native-toast/lib/typescript/core/types';

import { StyleSheet } from 'react-native';

import { useTheme } from '@contexts/theme';

import ToastIcon from '@components/toast-icon';

export function useToast() {
  const theme = useTheme();

  const message = (message: string) =>
    toast(message, {
      duration: 2500,
      icon: <ToastIcon type="default" />,
      styles: {
        view: {
          borderWidth: StyleSheet.hairlineWidth,
          borderColor: theme.palette.border,
        },
      },
    });

  const success = (message: string) =>
    toast.success(message, {
      duration: 2500,
      icon: <ToastIcon type="success" />,
      styles: {
        view: {
          borderWidth: StyleSheet.hairlineWidth,
          borderColor: theme.palette.success,
        },
      },
    });

  const warn = (message: string) =>
    toast.error(message, {
      duration: 5000,
      icon: <ToastIcon type="warn" />,
      styles: {
        view: {
          borderWidth: StyleSheet.hairlineWidth,
          borderColor: theme.palette.warning,
        },
      },
    });

  const error = (message: string) =>
    toast.error(message, {
      duration: Infinity,
      icon: <ToastIcon type="error" />,
      styles: {
        view: {
          borderWidth: StyleSheet.hairlineWidth,
          borderColor: theme.palette.error,
        },
      },
    });

  const promise = <T,>(
    promise: Promise<T>,
    msgs: {
      loading: Element;
      success: ValueOrFunction<Element, T>;
      error: ValueOrFunction<Element, any>;
    },
  ) => toast.promise(promise, msgs);

  const dismiss = toast.dismiss;
  const remove = toast.remove;

  return { message, success, warn, error, promise, dismiss, remove };
}
