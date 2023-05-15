const KEY_PREFIX = 'tewdew:';

export type StorageKey = typeof STORAGE_KEY[keyof typeof STORAGE_KEY];

export const STORAGE_KEY = {
    AUTH: 'token',
} as const;

const formatKey = (key: string) => `${KEY_PREFIX}:${key}`;

export function setStoredItem(key: StorageKey, item: string) {
    localStorage.setItem(formatKey(key), item);
}

export function getStoredItem(key: StorageKey) {
    return localStorage.getItem(formatKey(key));
}
