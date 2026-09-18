import {
  PropsWithChildren,
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from 'react';

import { API_URL } from '@/constants';

import { useNotificationContext } from '@/context/error';

import { apiFetch } from '@/lib/api-client';
import { authClient } from '@/lib/auth-client';

import { ApiContextType, AuthContextType, LoginInfo } from './types';

const AuthContext = createContext<AuthContextType>({
  loaded: false,
  currentSession: undefined,
  hasSession: false,
});
const ApiContext = createContext<ApiContextType>({
  createAccount: async (_: LoginInfo) => false,
  login: async (_: LoginInfo) => false,
  logout: async () => {},
});

export function Provider({ children }: PropsWithChildren) {
  const { showError } = useNotificationContext();

  const [user, setUser] = useState<AuthContextType>({
    loaded: false,
    currentSession: undefined,
    hasSession: false,
  });

  useEffect(() => {
    const getSession = async () => {
      const { data, error } = await authClient.getSession();
      if (error) {
        showError('Unable to connect to authentication service');

        setUser({
          loaded: true,
          currentSession: undefined,
          hasSession: false,
        });

        return;
      }

      if (!data) {
        showError('Your session has expired');

        setUser({
          loaded: true,
          currentSession: undefined,
          hasSession: false,
        });

        return;
      }

      try {
        await apiFetch(API_URL + '/api/me');
      } catch {
        showError('Unable to get user information');
      }

      setUser({
        loaded: true,
        currentSession: data.session.token,
        hasSession: true,
      });
    };

    getSession();
  }, [showError]);

  const createAccount = useCallback<ApiContextType['createAccount']>(
    async (params) => {
      const { data, error } = await authClient.signUp.email({
        email: params.email,
        password: params.password,
        name: params.email.split('@')[0],
      });
      if (error) {
        showError('Unable to create new account');

        return false;
      }

      const res = await apiFetch(API_URL + '/api/me');
      if (res.status !== 200) {
        throw false;
      }

      setUser({
        loaded: true,
        currentSession: data.token!,
        hasSession: true,
      });
      return true;
    },
    [showError],
  );

  const login = useCallback<ApiContextType['login']>(
    async (params) => {
      const { data, error } = await authClient.signIn.email({
        email: params.email,
        password: params.password,
      });
      if (error) {
        showError('Unable to login to account');

        return false;
      }

      const res = await apiFetch(API_URL + '/api/me');
      if (res.status !== 200) {
        throw false;
      }

      setUser({
        loaded: true,
        currentSession: data.token!,
        hasSession: true,
      });
      return true;
    },
    [showError],
  );

  const logout = useCallback<ApiContextType['logout']>(async () => {
    const { error } = await authClient.signOut();
    if (error) {
      showError('Unable to logout of account');

      return;
    }

    setUser({
      loaded: true,
      currentSession: undefined,
      hasSession: false,
    });
  }, [showError]);

  const api = useMemo(
    () => ({
      createAccount,
      login,
      logout,
    }),
    [createAccount, login, logout],
  );

  return (
    <AuthContext value={user}>
      <ApiContext value={api}>{children}</ApiContext>
    </AuthContext>
  );
}

export function useSession() {
  const ctx = useContext(AuthContext);
  if (!ctx) {
    throw new Error('useTheme must be used inside AuthProvider');
  }

  return ctx;
}

export function useSessionApi() {
  const ctx = useContext(ApiContext);
  if (!ctx) {
    throw new Error('useTheme must be used inside AuthProvider');
  }

  return ctx;
}
