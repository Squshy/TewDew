const KEY_PREFIX = 'tewdew:';

type StorageKey = typeof StorageKey[keyof typeof StorageKey];

export const StorageKey = {
    AUTH: 'token',
} as const;

const formatKey = (key: string) => `${KEY_PREFIX}:${key}`;

export function setStoredItem(key: StorageKey, item: string) {
    localStorage.setItem(formatKey(key), item);
}

export function getStoredItem(key: StorageKey) {
    return localStorage.getItem(formatKey(key));
}
