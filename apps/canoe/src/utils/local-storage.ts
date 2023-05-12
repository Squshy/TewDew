const KEY_PREFIX = 'tewdew:';

const formatKey = (key: string) => `${KEY_PREFIX}:${key}`;

export function setStoredItem(token: string, item: string) {
    localStorage.setItem(formatKey(token), item);
}

export function getStoredItem(key: string) {
    return localStorage.getItem(formatKey(key));
}
