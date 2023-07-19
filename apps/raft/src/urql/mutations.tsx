import { useCallback, useRef, useEffect, useState } from 'react';
import { TypedDocumentNode, AnyVariables, OperationResult } from '@urql/core';
import { useAlert } from '@alertle/react';
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
    const { notifyError, expireNode } = useAlert();
    const isMounted = useRef(true);
    const client = useUrqlClient();
    const [state, setState] = useState<State<Data>>(initialState);
    // eslint-disable-next-line
    const nodesRef = useRef<Map<string, any>>(new Map());

    useEffect(() => {
        isMounted.current = true;
        const nodes = nodesRef.current;
        return () => {
            // Clean up non dismissed nodes started from this mutation
            for (const node of Array.from(nodes.values())) {
                expireNode(node);
            }
            isMounted.current = false;
        };
    }, [expireNode]);

    const execute = useCallback(
        async (variables: Variables) => {
            setState((prev) => ({ ...prev, fetching: true }));
            const result = await client.mutation(query, variables).toPromise();

            if (isMounted.current) {
                // Extract the error mesasges from the fatal GraphQL errors
                const errors = result.error?.graphQLErrors.map(
                    (err) => err.message
                );

                setState({
                    fetching: false,
                    data: result.data,
                    errors,
                });

                if (errors) {
                    for (const err of errors) {
                        const node = notifyError({
                            message: err,
                            onExpire: ({ key }) => {
                                nodesRef.current.delete(key);
                            },
                        });
                        nodesRef.current.set(node.key, node);
                    }
                }
            }

            return result;
        },
        [client, query, setState, notifyError]
    );

    return [state, execute];
}

export function useLoginMutation() {
    return useMutation<LoginMutation, LoginMutationVariables>(LoginDocument);
}

export function useRegisterMutation() {
    return useMutation<RegisterMutation, RegisterMutationVariables>(
        RegisterDocument
    );
}

export function useCreateTewDewMutation() {
    return useMutation<CreateTewDewMutation, CreateTewDewMutationVariables>(
        CreateTewDewDocument
    );
}
