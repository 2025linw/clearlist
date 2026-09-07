import { PropsWithChildren, createContext, useContext, useState } from 'react';
import { View } from 'react-native';

import NotificationBar from '@/components/notification-bar';
import Button from '@/components/primitives/button';

import { ErrorContextType } from './types';

// import { ErrorContextType, Notification } from './types';

const ErrorContext = createContext<ErrorContextType>({
  showError(message) {},
  showWarning(message) {},
  showNotification(message) {},
});

export function Provider({ children }: PropsWithChildren) {
  const [message, setMessage] = useState<string | null>(null);

  function showError(message: string) {
    setMessage(message);
  }

  function showWarning(message: string) {
    setMessage(message);
  }

  function showNotification(message: string) {
    setMessage(message);
  }

  return (
    <ErrorContext.Provider value={{ showError, showWarning, showNotification }}>
      {children}

      <NotificationBar
        visible={message !== null}
        message={message!}
        onClose={() => setMessage(null)}
      />
    </ErrorContext.Provider>
  );
}

// function showNotification(notif: Notification) {}

export function useNotificationContext() {
  const ctx = useContext(ErrorContext);
  if (!ctx) {
    throw new Error('useNotification must be used inside ErrorProvider');
  }

  return ctx;
}

export function Demo() {
  const { showNotification, showWarning, showError } = useNotificationContext();

  return (
    /* eslint-disable react-native/no-inline-styles */
    <View style={{ gap: 16 }}>
      <Button onPress={() => showNotification('This is a notification')}>
        Notification
      </Button>
      <Button onPress={() => showWarning('This is a warning')}>Warning</Button>
      <Button onPress={() => showError('This is an error')}>Error</Button>
    </View>
    /* eslint-enable react-native/no-inline-styles */
  );
}
