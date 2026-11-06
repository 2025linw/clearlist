import { useRouter } from 'expo-router';
import {
  type PropsWithChildren,
  createContext,
  useCallback,
  useContext,
  useEffect,
  useEffectEvent,
  useMemo,
  useState,
} from 'react';

import { authClient } from '@lib/auth-client';
import { api as apiClient } from '@services/api/client';

import { useToast } from '@hooks/use-toast';

import {
  type ApiContextType,
  type AuthContextType,
  type LoginInfo,
} from './types';

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

type AuthProviderProps = PropsWithChildren;

export default function AuthProvider({ children }: AuthProviderProps) {
  const router = useRouter();
  const toast = useToast();

  const [user, setUser] = useState<AuthContextType>({
    loaded: false,
    currentSession: undefined,
    hasSession: false,
  });

  const createAccount = useCallback<ApiContextType['createAccount']>(
    async (params) => {
      const { data, error } = await authClient.signUp.email({
        email: params.email,
        password: params.password,
        name: params.email.split('@')[0],
      });
      if (error) {
        toast.error('Unable to create new account');

        return false;
      }

      const res = await apiClient.get<Response>('/api/me');
      if (res.status !== 200) {
        toast.error('Unable to access application account');

        throw false;
      }

      setUser({
        loaded: true,
        currentSession: data.token!,
        hasSession: true,
      });
      return true;
    },
    [toast],
  );
  const login = useCallback<ApiContextType['login']>(
    async (params) => {
      const { data, error } = await authClient.signIn.email({
        email: params.email,
        password: params.password,
      });
      if (error) {
        toast.error('Unable to login to account');

        return false;
      }

      const res = await apiClient.get<Response>('/api/me');
      if (res.status !== 200) {
        toast.error('Unable to access application account');

        throw false;
      }

      setUser({
        loaded: true,
        currentSession: data.token!,
        hasSession: true,
      });
      return true;
    },
    [toast],
  );
  const logout = useCallback<ApiContextType['logout']>(async () => {
    const { error } = await authClient.signOut();
    if (error) {
      toast.error('Unable to sign out. Please try again.');

      return;
    }

    setUser({
      loaded: true,
      currentSession: undefined,
      hasSession: false,
    });
  }, [toast]);

  const notifySessionError = useEffectEvent((message: string) => {
    toast.error(message);
  });

  const api = useMemo(
    () => ({
      createAccount,
      login,
      logout,
    }),
    [createAccount, login, logout],
  );

  useEffect(() => {
    const getSession = async () => {
      const { data, error } = await authClient.getSession();
      if (error) {
        notifySessionError('Unable to connect to authentication service');

        setUser({
          loaded: true,
          currentSession: undefined,
          hasSession: false,
        });

        return;
      }

      if (!data) {
        if (user.hasSession) notifySessionError('Your session has expired');

        setUser({
          loaded: true,
          currentSession: undefined,
          hasSession: false,
        });

        return;
      }

      try {
        await apiClient.get<Response>('/api/me');
      } catch {
        notifySessionError('Unable to get user information');
      }

      setUser({
        loaded: true,
        currentSession: data.session.token,
        hasSession: true,
      });
    };

    getSession();
  }, [router, user.hasSession]);

  return (
    <AuthContext value={user}>
      <ApiContext value={api}>{children}</ApiContext>
    </AuthContext>
  );
}

export function useSession() {
  const ctx = useContext(AuthContext);
  if (!ctx) {
    throw new Error('useSession must be used inside Auth Provider');
  }

  return ctx;
}

export function useSessionApi() {
  const ctx = useContext(ApiContext);
  if (!ctx) {
    throw new Error('useSessionApi must be used inside Auth Provider');
  }

  return ctx;
}
