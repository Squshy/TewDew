import { useCallback, useRef, useEffect, useState } from 'react';
import { TypedDocumentNode, AnyVariables, OperationResult } from '@urql/core';
//
import {
    LoginMutation,
    LoginDocument,
    LoginMutationVariables,
    RegisterMutation,
    RegisterMutationVariables,
    RegisterDocument,
    CreateTewDewMutation,
    CreateTewDewMutationVariables,
    CreateTewDewDocument,
} from 'tewgql';
//
import useUrqlClient from './client';

type State<T> = {
    fetching: boolean;
    data: T | undefined;
    errors: string[] | undefined;
};

type MutationFn<Data, Variables extends AnyVariables = AnyVariables> = (
    vars: Variables
) => Promise<OperationResult<Data, Variables>>;

const initialState = {
    fetching: false,
    data: undefined,
    errors: undefined,
} as const;

function useMutation<Data, Variables extends AnyVariables = AnyVariables>(
    query: TypedDocumentNode<Data, Variables>
): [State<Data>, MutationFn<Data, Variables>] {
    const isMounted = useRef(true);
    const client = useUrqlClient();
    const [state, setState] = useState<State<Data>>(initialState);

    useEffect(() => {
        isMounted.current = true;
        return () => {
            isMounted.current = false;
        };
    }, []);

    const execute = useCallback(
        async (variables: Variables) => {
            setState((prev) => ({ ...prev, fetching: true }));
            const result = await client.mutation(query, variables).toPromise();

            if (isMounted.current) {
                setState({
                    fetching: false,
                    data: result.data,
                    // Extract the error mesasges from the fatal GraphQL errors
                    errors: result.error?.graphQLErrors.map(
                        (err) => err.message
                    ),
                });
            }

            return result;
        },
        [client, query, setState]
    );

    return [state, execute];
}

export function useLogin() {
    return useMutation<LoginMutation, LoginMutationVariables>(LoginDocument);
}

export function useRegister() {
    return useMutation<RegisterMutation, RegisterMutationVariables>(
        RegisterDocument
    );
}

export function useCreateTewDew() {
    return useMutation<CreateTewDewMutation, CreateTewDewMutationVariables>(
        CreateTewDewDocument
    );
}
