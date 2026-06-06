import { PropsWithChildren, createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react';

import { authClient } from '@/lib/auth-client';

import { ApiContextType, AuthContextType } from './types';

const AuthContext = createContext<AuthContextType>({
  currentSession: undefined,
  hasSession: false,
});
const ApiContext = createContext<ApiContextType>({
  createAccount: async () => {},
  login: async () => {},
  logout: async () => {},
});

export function Provider({ children }: PropsWithChildren) {
  const [user, setUser] = useState<AuthContextType>({
    currentSession: undefined,
    hasSession: undefined,
  });

  useEffect(() => {
    const getSession = async () => {
      const { data, error } = await authClient.getSession();
      if (error) {
        console.error(error);

        setUser({
          currentSession: undefined,
          hasSession: false,
        });

        throw error;
      }

      if (!data) {
        // if there isn't an existing session or session expired
        setUser({
          currentSession: undefined,
          hasSession: false,
        });

        return;
      }

      setUser({
        currentSession: data.session.token,
        hasSession: true,
      });
    };

    getSession();
  }, []);

  const createAccount = useCallback<ApiContextType['createAccount']>(async (params) => {
    const { data, error } = await authClient.signUp.email({
      email: params.email,
      password: params.password,
      name: params.email.split('@')[0],
    });

    if (error) {
      console.error(error);

      throw error;
    }

    setUser({
      currentSession: data.token!,
      hasSession: true,
    });
  }, []);

  const login = useCallback<ApiContextType['login']>(async (params) => {
    const { data, error } = await authClient.signIn.email({
      email: params.email,
      password: params.password,
    });

    if (error) {
      console.error(error);

      throw error;
    }

    setUser({
      currentSession: data.token!,
      hasSession: true,
    });
  }, []);

  const logout = useCallback<ApiContextType['logout']>(async () => {
    const { error } = await authClient.signOut();

    if (error) {
      console.error(error);

      throw error;
    }

    setUser({
      currentSession: undefined,
      hasSession: false,
    });
  }, []);

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
