// Copy-pastad from https://github.com/urql-graphql/urql/blob/main/packages/react-urql/src/hooks/cache.ts
import { pipe, subscribe } from 'wonka';
import type { Client, OperationResult } from '@urql/core';

type CacheEntry = OperationResult | Promise<unknown> | undefined;

type Cache = {
    get(key: number): CacheEntry;
    set(key: number, value: CacheEntry): void;
    dispose(key: number): void;
};

type ClientWithCache = Client & {
    _react?: Cache;
};

export default function getCacheForClient<T extends ClientWithCache>(
    client: T
): Cache {
    if (!client._react) {
        const reclaim = new Set();
        const map = new Map<number, CacheEntry>();

        if (client.operations$ /* not available in mocks */) {
            pipe(
                client.operations$,
                subscribe((operation) => {
                    if (
                        operation.kind === 'teardown' &&
                        reclaim.has(operation.key)
                    ) {
                        reclaim.delete(operation.key);
                        map.delete(operation.key);
                    }
                })
            );
        }

        client._react = {
            get(key) {
                return map.get(key);
            },
            set(key, value) {
                reclaim.delete(key);
                map.set(key, value);
            },
            dispose(key) {
                reclaim.add(key);
            },
        };
    }

    return client._react as Cache;
}
