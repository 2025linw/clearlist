export type LoginInfo = { email: string; password: string };

export type AuthContextType = {
  loaded: boolean;

  currentSession: string | undefined;
  hasSession: boolean;
};

export type ApiContextType = {
  createAccount: (params: LoginInfo) => Promise<boolean>;
  login: (params: LoginInfo) => Promise<boolean>;
  logout: () => Promise<void>;
};
