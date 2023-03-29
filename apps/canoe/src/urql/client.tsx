import { useContext, createContext, useMemo, ReactNode } from 'react';
import { createClient } from '@urql/core';

const UrqlClientContext = createContext<ReturnType<typeof createClient> | null>(
    null
);

type UrqlClientProviderProps = {
    children: ReactNode | ReactNode[];
};

export function UrqlClientProvider({ children }: UrqlClientProviderProps) {
    const client = useMemo(() => {
        return createClient({ url: 'http://localhost:4000/' });
    }, []);

    return (
        <UrqlClientContext.Provider value={client}>
            {children}
        </UrqlClientContext.Provider>
    );
}

export default function useUrqlClient() {
    const client = useContext(UrqlClientContext);

    if (!client) {
        throw new Error(
            '`useUrqlClient` can only be used within an `<UrqlClientProvider />` provider.'
        );
    }

    return client;
}
