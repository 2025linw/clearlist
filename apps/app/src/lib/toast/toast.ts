import {
  type DefaultToastOptions,
  type ToastOptions as ToastOptionsLib,
  type ValueOrFunction,
  toast as toastLib,
} from '@backpackapp-io/react-native-toast';

import Toast from '@components/toast';

type ToastOptions = Omit<ToastOptionsLib, 'customToast'>;

const toast = (message: string, opts?: ToastOptions) =>
  toastLib(message, { customToast: Toast, ...opts });

toast.error = (message: string, opts?: ToastOptions) =>
  toastLib.error(message, {
    ...opts,
  });
toast.success = (message: string, opts?: ToastOptions) =>
  toastLib.success(message, {
    ...opts,
  });
toast.promise = <T>(
  promise: Promise<T>,
  msgs: {
    loading: Element;
    success: ValueOrFunction<Element, T>;
    error: ValueOrFunction<Element, any>;
  },
  opts?: DefaultToastOptions,
) => toastLib.promise(promise, msgs, { ...opts });
toast.dismiss = toastLib.dismiss;
toast.remove = toastLib.remove;

export { toast };
