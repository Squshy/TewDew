import {
    createContext,
    useContext,
    useReducer,
    ReactNode,
    useCallback,
} from 'react';
import type { AuthUser } from 'tewgql';
import {
    StorageKeys,
    clearStoredItem,
    setStoredItem,
} from '../utils/local-storage';

type AuthState = {
    user?: AuthUser;
};

type AuthContext = {
    state: AuthState;
    storeLocalAuth: (user: AuthUser) => void;
    clearLocalAuth: () => void;
};

const Context = createContext<AuthContext | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
    const [state, setState] = useReducer(
        (prev: AuthState, cur: Partial<AuthState>) => ({ ...prev, ...cur }),
        {}
    );

    const storeLocalAuth = useCallback((user: AuthUser) => {
        setState({ user });
        setStoredItem(StorageKeys.AUTH, user.token);
    }, []);

    const clearLocalAuth = useCallback(() => {
        setState({ user: undefined });
        clearStoredItem(StorageKeys.AUTH);
    }, []);

    return (
        <Context.Provider value={{ state, storeLocalAuth, clearLocalAuth }}>
            {children}
        </Context.Provider>
    );
}

export default function useAuthContext() {
    const context = useContext(Context);

    if (!context) {
        throw new Error(
            '`useAuthContext` can only be used within a `<AuthProvider />` provider.'
        );
    }

    return context;
}
