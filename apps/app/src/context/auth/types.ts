export type AuthContextType = {
  currentSession: string | undefined;
  hasSession: boolean | undefined;
};

export type ApiContextType = {
  createAccount: (params: { email: string; password: string }) => Promise<void>;
  login: (params: { email: string; password: string }) => Promise<void>;
  logout: () => Promise<void>;
};
