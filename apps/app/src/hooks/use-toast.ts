import { type ValueOrFunction } from '@backpackapp-io/react-native-toast/lib/typescript/core/types';

import { StyleSheet } from 'react-native';

import { useTheme } from '@contexts/theme';
import toast from '@lib/toast';

export function useToast() {
  const theme = useTheme();

  const message = (message: string) =>
    toast(message, {
      styles: {
        view: {
          borderWidth: StyleSheet.hairlineWidth,
          borderColor: theme.palette.border,
        },
      },
    });

  const success = (message: string) =>
    toast.success(message, {
      styles: {
        view: {
          borderWidth: StyleSheet.hairlineWidth,
          borderColor: theme.palette.success,
        },
      },
    });

  const error = (message: string) =>
    toast.error(message, {
      styles: {
        view: {
          borderWidth: StyleSheet.hairlineWidth,
          borderColor: theme.palette.error,
        },
      },
    });

  const promise = <T>(
    promise: Promise<T>,
    msgs: {
      loading: Element;
      success: ValueOrFunction<Element, T>;
      error: ValueOrFunction<Element, any>;
    },
  ) => toast.promise(promise, msgs);

  const dismiss = toast.dismiss;
  const remove = toast.remove;

  return { message, success, error, promise, dismiss, remove };
}
