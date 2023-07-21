const KEY_PREFIX = 'tewdew:';

type StorageKey = typeof StorageKeys[keyof typeof StorageKeys];

export const StorageKeys = {
    TOKEN: 'token',
    USER: 'user',
} as const;

const formatKey = (key: string) => `${KEY_PREFIX}:${key}`;

export function setStoredItem(key: StorageKey, item: string): void {
    localStorage.setItem(formatKey(key), item);
}

export function setStoredObject<T extends Record<PropertyKey, unknown>>(
    key: StorageKey,
    item: T
): void {
    setStoredItem(key, JSON.stringify(item));
}

export function clearStoredItem(key: StorageKey): void {
    localStorage.removeItem(formatKey(key));
}

export function getStoredItem(key: StorageKey): string | null {
    return localStorage.getItem(formatKey(key));
}

export function getStoredObject<T>(key: StorageKey): T | null {
    const item = getStoredItem(key);

    try {
        return item ? (JSON.parse(item) as T) : null;
    } catch (e) {
        console.error(e);
        return null;
    }
}
