export type AuthContextType = {
  loaded: boolean;

  currentSession: string | undefined;
  hasSession: boolean;
};

export type ApiContextType = {
  createAccount: (params: { email: string; password: string }) => Promise<void>;
  login: (params: { email: string; password: string }) => Promise<void>;
  logout: () => Promise<void>;
};
