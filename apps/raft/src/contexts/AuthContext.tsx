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
    getStoredObject,
    setStoredItem,
    setStoredObject,
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
        { user: getStoredObject<AuthUser>(StorageKeys.USER) ?? undefined }
    );

    const storeLocalAuth = useCallback((user: AuthUser) => {
        setState({ user });
        setStoredItem(StorageKeys.TOKEN, user.token);
        setStoredObject(StorageKeys.USER, user);
    }, []);

    const clearLocalAuth = useCallback(() => {
        setState({ user: undefined });
        clearStoredItem(StorageKeys.TOKEN);
        clearStoredItem(StorageKeys.USER);
    }, []);

    return (
        <Context.Provider value={{ state, storeLocalAuth, clearLocalAuth }}>
            {children}
        </Context.Provider>
    );
}

/**
 * Pass in a generic of `false` to allow the state to be undefined. Otherwise,
 * assume the state is present since this will primarily be used within authenticated
 * routes which are protected by a wrapper which navigates away if there is no state.
 */
export default function useAuthContext<
    AssertExists extends boolean = true
>(): AssertExists extends true
    ? Omit<AuthContext, 'state'> & { state: Required<AuthState> }
    : AuthContext {
    const context = useContext(Context);

    if (!context) {
        throw new Error(
            '`useAuthContext` can only be used within a `<AuthProvider />` provider.'
        );
    }

    return context as AssertExists extends true
        ? Omit<AuthContext, 'state'> & { state: Required<AuthState> }
        : AuthContext;
}
