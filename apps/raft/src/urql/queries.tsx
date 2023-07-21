import { useQuery, UseQueryArgs } from 'urql';
//
import { MeDocument, MeQuery, MeQueryVariables } from 'tewgql';

export function useMeQuery(
    options: Omit<UseQueryArgs<MeQueryVariables>, 'query' | 'variables'> = {}
) {
    return useQuery<MeQuery, MeQueryVariables>({
        query: MeDocument,
        ...options,
    });
}
