import { useContext, createContext, ReactNode } from 'react';
import { createClient, fetchExchange } from '@urql/core';
import { cacheExchange } from '@urql/exchange-graphcache';
import { authExchange } from '@urql/exchange-auth';
//
import { getStoredItem, STORAGE_KEY } from '../utils/local-storage';

const GRAPHQL_CODE = {
    FORBIDDEN: 'FORBIDDEN',
    UNAUTHORIZED: 'UNAUTHORIZED',
} as const;

const client = createClient({
    url: 'http://localhost:4000/',
    exchanges: [
        cacheExchange(),
        authExchange(async (utils) => {
            return {
                addAuthToOperation(operation) {
                    // Accessing this on every request
                    // This could be improved
                    const token = getStoredItem(STORAGE_KEY.AUTH);

                    if (!token) {
                        return operation;
                    }

                    return utils.appendHeaders(operation, {
                        Authorization: `Bearer ${token}`,
                    });
                },
                didAuthError(error) {
                    return error.graphQLErrors.some((e) => {
                        return (
                            e.extensions?.code === GRAPHQL_CODE.FORBIDDEN ||
                            e.extensions?.code === GRAPHQL_CODE.UNAUTHORIZED
                        );
                    });
                },
                async refreshAuth() {
                    // no-op
                },
            };
        }),
        fetchExchange,
    ],
});

const UrqlClientContext = createContext<ReturnType<typeof createClient> | null>(
    null
);

type UrqlClientProviderProps = {
    children: ReactNode | ReactNode[];
};

export function UrqlClientProvider({ children }: UrqlClientProviderProps) {
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
