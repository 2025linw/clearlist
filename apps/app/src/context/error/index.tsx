import { PropsWithChildren, createContext, useContext, useState } from 'react';

import NotificationBar from '@/components/notification-bar';

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

  function showWarning(message: string) {}

  function showNotification(message: string) {}

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
