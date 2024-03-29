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
    UpdateTaskMutation,
    UpdateTaskMutationVariables,
    UpdateTaskDocument,
    CreateTaskMutation,
    CreateTaskDocument,
    CreateTaskMutationVariables,
    UpdateTewDewMutation,
    UpdateTewDewMutationVariables,
    UpdateTewDewDocument,
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

export function useUpdateTaskMutation() {
    return useMutation<UpdateTaskMutation, UpdateTaskMutationVariables>(
        UpdateTaskDocument
    );
}

export function useUpdateTewDewMutation() {
    return useMutation<UpdateTewDewMutation, UpdateTewDewMutationVariables>(
        UpdateTewDewDocument
    );
}

export function useCreateTaskMutation() {
    return useMutation<CreateTaskMutation, CreateTaskMutationVariables>(
        CreateTaskDocument
    );
}
