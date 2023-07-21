import { useCallback, useRef, useEffect, useState } from 'react';
import { useAlert } from '@alertle/react';
import { useMutation } from 'urql';
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
