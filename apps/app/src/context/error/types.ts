export type ErrorContextType = {
  showError: (message: string) => void;
  showWarning: (message: string) => void;
  showNotification: (message: string) => void;
};

type NotificationType = 'success' | 'info' | 'warning' | 'error';

export type Notification = {
  type: NotificationType;
  message: string;
};
