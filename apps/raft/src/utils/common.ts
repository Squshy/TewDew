/**
 * Takes a form event and creates an object containing key value pairs for
 * the inputs named as children of the form. Inputs _must_ be named for
 * them to be present.
 */
export function formEntries<T extends Record<string, unknown>>(
    e: React.FormEvent<HTMLFormElement>
): T {
    const ok = (e.target || e.currentTarget) as HTMLFormElement;
    const haha = Object.fromEntries(new FormData(ok)) as T;

    for (const [key, value] of Object.entries(haha)) {
        // eslint-disable-next-line
        // @ts-ignore
        if ((e.target[key] as HTMLInputElement).type === 'checkbox') {
            // eslint-disable-next-line
            // @ts-ignore
            haha[key] = value === 'on';
        }
    }

    return haha;
}
