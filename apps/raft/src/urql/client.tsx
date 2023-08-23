import type { ReactNode } from 'react';
import { authExchange } from '@urql/exchange-auth';
import { Client, fetchExchange, Provider } from 'urql';
//
import { getStoredItem, StorageKeys } from '../utils/local-storage';
import cacheExchange from './cache-exchange';

const GRAPHQL_CODE = {
    FORBIDDEN: 'FORBIDDEN',
    UNAUTHORIZED: 'UNAUTHORIZED',
} as const;

const client = new Client({
    url: 'http://localhost:4000/',
    exchanges: [
        cacheExchange,
        // eslint-disable-next-line @typescript-eslint/require-await
        authExchange(async (utils) => {
            return {
                addAuthToOperation(operation) {
                    // Accessing this on every request
                    // This could be improved
                    const token = getStoredItem(StorageKeys.TOKEN);

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

export default function UrqlClientProvider({
    children,
}: {
    children: ReactNode | ReactNode[];
}) {
    return <Provider value={client}>{children}</Provider>;
}
