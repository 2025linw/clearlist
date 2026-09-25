import {
  ToastOptions as ToastOptionsLib,
  toast as toastLib,
} from '@backpackapp-io/react-native-toast';

export { default as ToastRenderer } from './renderer';

type ToastOptions = Omit<ToastOptionsLib, 'customToast'>;

const toast = (message: string, opts: ToastOptions) =>
  toastLib(message, { ...opts });

toast.error = (message: string, opts: ToastOptions) =>
  toastLib.error(message, {
    ...opts,
  });
toast.success = (message: string, opts: ToastOptions) =>
  toastLib.success(message, {
    ...opts,
  });
toast.promise = toastLib.promise;
toast.dismiss = toastLib.dismiss;
toast.remove = toastLib.remove;

export default toast;
