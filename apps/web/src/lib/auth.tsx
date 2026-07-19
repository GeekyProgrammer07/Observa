import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";
import { auth as authApi } from "./api";
import { createLogger } from "./logger";

const log = createLogger("auth");

interface AuthState {
  token: string | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  signin: (username: string, password: string) => Promise<void>;
  signup: (data: {
    firstname: string;
    lastname: string;
    username: string;
    password: string;
  }) => Promise<void>;
  signout: () => void;
}

const AuthContext = createContext<AuthState | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [token, setToken] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const stored = localStorage.getItem("access_token");
    setToken(stored);
    setIsLoading(false);
  }, []);

  const signin = useCallback(async (username: string, password: string) => {
    try {
      const res = await authApi.signin({ username, password });
      localStorage.setItem("access_token", res.access_token);
      setToken(res.access_token);
      log.info("user signed in");
    } catch (err) {
      log.warn("signin failed", err);
      throw err;
    }
  }, []);

  const signup = useCallback(
    async (data: {
      firstname: string;
      lastname: string;
      username: string;
      password: string;
    }) => {
      await authApi.signup(data);
    },
    []
  );

  const signout = useCallback(() => {
    localStorage.removeItem("access_token");
    setToken(null);
    log.info("user signed out");
  }, []);

  return (
    <AuthContext.Provider
      value={{
        token,
        isAuthenticated: !!token,
        isLoading,
        signin,
        signup,
        signout,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth(): AuthState {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}
