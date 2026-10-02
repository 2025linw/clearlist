import {
  type DefaultToastOptions,
  type ToastOptions as ToastOptionsLib,
  type ValueOrFunction,
  toast as toastLib,
} from '@backpackapp-io/react-native-toast';

import ToastIcon from './icon';

export { default as ToastRenderer } from './renderer';

export type ToastOptions = Omit<ToastOptionsLib, 'customToast'>;

const toast = (message: string, opts?: ToastOptions) =>
  toastLib(message, {
    icon: <ToastIcon type="default" />,
    duration: 3000,
    ...opts,
  });
toast.success = (message: string, opts?: ToastOptions) =>
  toastLib.success(message, {
    icon: <ToastIcon type="success" />,
    duration: 3000,
    ...opts,
  });
toast.error = (message: string, opts?: ToastOptions) =>
  toastLib.error(message, {
    icon: <ToastIcon type="error" />,
    duration: 6000,
    ...opts,
  });

toast.promise = <T,>(
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

export default toast;
