import { useContext, createContext, ReactNode } from 'react';
import { createClient, fetchExchange } from '@urql/core';
import { cacheExchange } from '@urql/exchange-graphcache';
import { authExchange, AuthConfig } from '@urql/exchange-auth';
//

const GRAPHQL_CODES = {
    FORBIDDEN: 'FORBIDDEN',
} as const;

const client = createClient({
    url: 'http://localhost:4000/',
    exchanges: [
        cacheExchange(),
        authExchange(async (utils): Promise<AuthConfig> => {
            const token = localStorage.get('x-token');

            return {
                addAuthToOperation(operation) {
                    if (!token) {
                        return operation;
                    }

                    return utils.appendHeaders(operation, {
                        Authorization: `Bearer ${token}`,
                    });
                },
                didAuthError(error) {
                    return error.graphQLErrors.some(
                        (e) => e.extensions?.code === GRAPHQL_CODES.FORBIDDEN
                    );
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
