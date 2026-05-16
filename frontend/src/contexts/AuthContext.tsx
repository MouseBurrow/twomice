import { createContext, type ReactNode, useContext, useEffect, useState } from "react";
import { api } from "../api.ts";

import type { AuthState } from "../types.ts";

type AuthContextValue = {
    auth: AuthState;
    refresh: () => Promise<void>;
    logout: () => Promise<void>;
};

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
    const [auth, setAuth] = useState<AuthState>({ status: "unknown" });

    async function refresh() {
        try {
            const info = await api.account();

            if (info.is_admin) {
                setAuth({ status: "admin", info });
            } else {
                setAuth({ status: "user", info });
            }
        } catch {
            setAuth({ status: "guest" });
        }
    }

    async function logout() {
        await api.logout();
        setAuth({ status: "guest" });
    }

    useEffect(() => {
        // eslint-disable-next-line react-hooks/set-state-in-effect
        refresh();
    }, []);

    return (
        <AuthContext.Provider value={{ auth, refresh, logout }}>
            {children}
        </AuthContext.Provider>
    );
}

// eslint-disable-next-line react-refresh/only-export-components
export function useAuth() {
    const ctx = useContext(AuthContext);
    if (!ctx) {
        throw new Error("useAuth must be used inside AuthProvider");
    }
    return ctx;
}
