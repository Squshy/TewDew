// Snagged a lot from https://github.com/urql-graphql/urql/blob/main/packages/react-urql/src/hooks/useQuery.ts
import { useCallback, useRef, useEffect, useState, useMemo } from 'react';
import {
    AnyVariables,
    OperationResult,
    GraphQLRequestParams,
    DocumentInput,
    GraphQLRequest,
    createRequest,
} from '@urql/core';
//
import useUrqlClient from './client';
import getCacheForClient from './cache';
import { MeDocument, MeQuery, MeQueryVariables } from 'tewgql';

type UseQueryArgs<Data, Variables extends AnyVariables = AnyVariables> = {
    pause?: boolean;
} & GraphQLRequestParams<Data, Variables>;

type State<T> = {
    fetching: boolean;
    data: T | undefined;
    errors: string[] | undefined;
};

type QueryFn<
    Data,
    Variables extends AnyVariables = AnyVariables
> = () => Promise<OperationResult<Data, Variables>>;
type UseQueryResponse<T, V extends AnyVariables = AnyVariables> = [
    State<T>,
    QueryFn<T, V>
];

function useRequest<Data, Variables extends AnyVariables = AnyVariables>(
    query: DocumentInput<Data, Variables>,
    variables: Variables
) {
    const prev = useRef<undefined | GraphQLRequest<Data, Variables>>(undefined);

    return useMemo(() => {
        const request = createRequest(query, variables);

        if (prev.current?.key === request.key) {
            return prev.current;
        }

        prev.current = request;
        return request;
    }, [query, variables]);
}

function useQuery<Data, Variables extends AnyVariables = AnyVariables>(
    params: UseQueryArgs<Data, Variables>
): UseQueryResponse<Data, Variables> {
    const isMounted = useRef(true);
    const client = useUrqlClient();
    const cache = getCacheForClient(client);
    const request = useRequest(params.query, params.variables);
    const [state, setState] = useState<State<Data>>(() => {
        const cachedResult = cache.get(request.key) as
            | OperationResult<Data, Variables>
            | undefined;

        return {
            fetching: !!cachedResult,
            data: cachedResult?.data,
            errors: cachedResult?.error?.graphQLErrors.map(
                (err) => err.message
            ),
        };
    });

    const execute = useCallback(async () => {
        setState((prev) => ({ ...prev, fetching: true }));
        const result = await client
            .query(params.query, params.variables)
            .toPromise();

        if (isMounted) {
            setState({
                fetching: false,
                data: result.data,
                errors: result.error?.graphQLErrors.map((err) => err.message),
            });
        }

        cache.set(request.key, result);

        // HACK: Don't care about the other types I don't think?
        return result as OperationResult<Data, Variables>;
    }, [client, params.query, params.variables, setState]);

    useEffect(() => {
        if (params.pause || state.data || state.errors) {
            return;
        }

        execute();

        return () => {
            cache.dispose(request.key);
            isMounted.current = false;
        };
    }, [execute, params.pause, state, cache]);

    return [state, execute];
}

export function useMeQuery(
    options: Omit<UseQueryArgs<MeQueryVariables>, 'query' | 'variables'> = {}
) {
    return useQuery<MeQuery, MeQueryVariables>({
        query: MeDocument,
        ...options,
    });
}
