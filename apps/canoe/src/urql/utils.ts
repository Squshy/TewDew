import type { FieldError } from 'tewgql';

export function buildFieldErrorMap(
    fieldErrorArr: FieldError[]
): Record<string, string> {
    const obj: Record<string, string> = {};

    for (const error of fieldErrorArr) {
        obj[error.field] = error.message;
    }

    return obj;
}
