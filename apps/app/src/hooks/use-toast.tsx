import { toast } from '@backpackapp-io/react-native-toast';
import { type ValueOrFunction } from '@backpackapp-io/react-native-toast/lib/typescript/core/types';

import { View } from 'react-native';

import { useTheme } from '@contexts/theme';

import Button from '@components/primitives/button';
import { ToastIcon } from '@components/toast';

export function useToast() {
  const theme = useTheme();

  const message = (message: string) =>
    toast(message, {
      duration: 2500,
      icon: <ToastIcon type="default" />,
      styles: {
        view: {
          borderWidth: theme.spacings.thin,
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
          borderWidth: theme.spacings.thin,
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
          borderWidth: theme.spacings.thin,
          borderColor: theme.palette.warning,
        },
      },
    });

  const error = (message: string) =>
    toast.error(message, {
      duration: 10000,
      icon: <ToastIcon type="error" />,
      styles: {
        view: {
          borderWidth: theme.spacings.thin,
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

export function Demo() {
  const toast = useToast();

  return (
    // eslint-disable-next-line react-native/no-inline-styles
    <View style={{ gap: 16, paddingHorizontal: 10 }}>
      <Button onPress={() => toast.message('This is a default toast')}>
        Default
      </Button>
      <Button onPress={() => toast.success('This is a success toast')}>
        Success
      </Button>
      <Button onPress={() => toast.warn('This is an warn toast')}>Warn</Button>
      <Button onPress={() => toast.error('This is an error toast')}>
        Error
      </Button>
    </View>
  );
}
