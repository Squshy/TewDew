import { cacheExchange, Cache } from '@urql/exchange-graphcache';
import gql from 'graphql-tag';
import type {
    CreateTewDewMutation,
    CreateTewDewMutationVariables,
    CreateTaskMutation,
    CreateTaskMutationVariables,
    UpdateTewDewMutation,
    UpdateTewDewMutationVariables,
} from 'tewgql';

function invalidateCache(cache: Cache) {
    cache.invalidate('Query');
    cache.invalidate('Mutation');
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

    const frag = gql`
        fragment _ on SlimTewDew {
            id
            title
            description
            completed
            userId
            createdAt
            updatedAt
        }
    `;

    cache.writeFragment(frag, result.createTewDew.tewDew!);
    cache.invalidate('query', 'listTewDews');
}

function createTask(
    result: CreateTaskMutation,
    args: CreateTaskMutationVariables,
    cache: Cache
) {
    if (result.createTask.taskErrors) {
        return;
    }

    const frag = gql`
        fragment _ on Task {
            id
            title
            completed
            tewDewId
            userId
            createdAt
            updatedAt
        }
    `;

    cache.writeFragment(frag, result.createTask.task!);
    cache.invalidate({ __typename: 'TewDew', id: args.tewdewId });
}

function updateTewDew(
    result: UpdateTewDewMutation,
    _args: UpdateTewDewMutationVariables,
    cache: Cache
) {
    if (result.updateTewDew.tewDewErrors) {
        return;
    }

    const frag = gql`
        fragment _ on TewDew {
            id
            title
            description
            completed
            userId
            createdAt
            updatedAt
        }
    `;

    cache.writeFragment(frag, result.updateTewDew.tewDew!);
    cache.invalidate({ __typename: 'TewDew', id: _args.id });
}

export default cacheExchange({
    updates: {
        Mutation: {
            createTewDew: createTewDew,
            createTask: createTask,
            updateTewDew: updateTewDew,
            login: (_, __, cache) => {
                invalidateCache(cache);
            },
        },
    },
});
