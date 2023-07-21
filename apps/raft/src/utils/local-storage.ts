const KEY_PREFIX = 'tewdew:';

type StorageKey = typeof StorageKeys[keyof typeof StorageKeys];

export const StorageKeys = {
    AUTH: 'token',
} as const;

const formatKey = (key: string) => `${KEY_PREFIX}:${key}`;

export function setStoredItem(key: StorageKey, item: string): void {
    localStorage.setItem(formatKey(key), item);
}

export function clearStoredItem(key: StorageKey): void {
    localStorage.removeItem(formatKey(key));
}

export function getStoredItem(key: StorageKey): string | null {
    return localStorage.getItem(formatKey(key));
}
