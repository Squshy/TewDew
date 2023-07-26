import { AnyVariables, useQuery, UseQueryArgs } from 'urql';
//
import {
    ListTewDewsDocument,
    ListTewDewsQuery,
    ListTewDewsQueryVariables,
    MeDocument,
    MeQuery,
    MeQueryVariables,
} from 'tewgql';

type QueryVars<T extends AnyVariables> = Omit<
    UseQueryArgs<T>,
    'query' | 'variables'
>;

export function useMeQuery(options: QueryVars<MeQueryVariables> = {}) {
    return useQuery<MeQuery, MeQueryVariables>({
        query: MeDocument,
        ...options,
    });
}

export function useListTewDewsQuery(
    options: QueryVars<ListTewDewsQueryVariables> = {}
) {
    return useQuery<ListTewDewsQuery, ListTewDewsQueryVariables>({
        query: ListTewDewsDocument,
        ...options,
    });
}
