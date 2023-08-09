import { cacheExchange, Cache, FieldInfo } from '@urql/exchange-graphcache';
// import gql from 'graphql-tag';
import type {
    CreateTewDewMutation,
    CreateTewDewMutationVariables,
} from 'tewgql';

function invalidateAllTewDews(cache: Cache) {
    const fieldInfos = cache
        .inspectFields('Query')
        .filter((info: FieldInfo) => info.fieldName === 'listTewDews');

    for (const info of fieldInfos) {
        cache.invalidate('Query', info.fieldKey);
    }
}

function createTewDew(
    result: CreateTewDewMutation,
    _args: CreateTewDewMutationVariables,
    cache: Cache
) {
    // Do not invalidate the cache if the creation was unsuccessful
    if (result.createTewDew.tewDewErrors) {
        return;
    }

    // When we create a new tewdew we will want to invalidate all of our
    // currently stored tewdews
    invalidateAllTewDews(cache);
}

export default cacheExchange({
    updates: {
        Mutation: {
            createTewDew: createTewDew,
            login: (_, __, cache) => {
                invalidateAllTewDews(cache);
            },
        },
    },
});
