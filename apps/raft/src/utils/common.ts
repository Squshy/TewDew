/**
 * Takes a form event and creates an object containing key value pairs for
 * the inputs named as children of the form. Inputs _must_ be named for
 * them to be present.
 */
export function formEntries<T extends Record<string, unknown>>(
    e: React.FormEvent<HTMLFormElement>
): T {
    return Object.fromEntries(new FormData(e.currentTarget)) as T;
}
