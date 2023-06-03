import React, {
    useState,
    createContext,
    useContext,
    useCallback,
    useEffect,
    useRef,
    useMemo,
    useSyncExternalStore,
} from 'react';

const DEFAULT_MESSAGE_EXPIRY_TIME_MS = 3500; // 3.5 seconds
const ANIMATION_DURATION_MS = 200;

type NotificationFnParams = Partial<
    Pick<NotificationNode, 'title' | 'expiresInMs'>
> & {
    message: string;
};

type NotificationFn = (args: NotificationFnParams) => NotificationNode;

type ExpireNodeFn = (key: string) => void;
type InternalNotificationContext = {
    getNodeMap: () => Map<string, NotificationNode>;
    subscribe: (cb: () => void) => () => boolean;
    expireNode: ExpireNodeFn;
};

type NotificationContext = {
    notify: (
        args: NotificationFnParams & { type: NotificationType }
    ) => NotificationNode;
    notifySuccess: NotificationFn;
    notifyError: NotificationFn;
    notifyWarning: NotificationFn;
    expireNode: ExpireNodeFn;
};

type NotificationType = typeof NotificationType[keyof typeof NotificationType];

const NotificationType = {
    SUCCESS: 'success',
    ERROR: 'error',
    WARNING: 'warning',
} as const;

type NotificationNode = {
    key: string;
    message: string;
    title?: string | undefined;
    type: NotificationType;
    createdAt: number;
    expiresInMs: number;
};

const InternalContext = createContext<InternalNotificationContext | null>(null);
const Context = createContext<NotificationContext | null>(null);

type NotificationProviderProps = {
    children: React.ReactNode | React.ReactNode[];
};

function createNotificationNode({
    type,
    message,
    title,
    expiresInMs = DEFAULT_MESSAGE_EXPIRY_TIME_MS,
}: {
    type: NotificationType;
    message: string;
    title?: string | undefined;
    expiresInMs?: number;
}): NotificationNode {
    const messageKey = message.replace(/\s/g, '').toLocaleLowerCase();
    const titleKey = title?.replace(/\s/g, '').toLocaleLowerCase() || '';
    // Using a key based off of the node's structure will mean duplicate
    // messages will not cause multiple notifications to appear as they will
    // replace the current map entry
    const key = `${type}:${titleKey}:${messageKey}`;

    return {
        key,
        type,
        message,
        expiresInMs,
        title,
        createdAt: Date.now(),
    };
}

type NotificationProps = {
    node: NotificationNode;
    expireNode: ExpireNodeFn;
};

function NotificationIcon({ node }: { node: NotificationNode }) {
    if (node.type === 'success') {
        return (
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                className="w-6 h-6 fill-green-500"
            >
                <path
                    fillRule="evenodd"
                    d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm13.36-1.814a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z"
                    clipRule="evenodd"
                />
            </svg>
        );
    }

    if (node.type === 'error') {
        return (
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                className="w-6 h-6 fill-red-500"
            >
                <path
                    fillRule="evenodd"
                    d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zM12 8.25a.75.75 0 01.75.75v3.75a.75.75 0 01-1.5 0V9a.75.75 0 01.75-.75zm0 8.25a.75.75 0 100-1.5.75.75 0 000 1.5z"
                    clipRule="evenodd"
                />
            </svg>
        );
    }

    return (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            className="w-6 h-6 fill-yellow-500"
        >
            <path
                fillRule="evenodd"
                d="M9.401 3.003c1.155-2 4.043-2 5.197 0l7.355 12.748c1.154 2-.29 4.5-2.599 4.5H4.645c-2.309 0-3.752-2.5-2.598-4.5L9.4 3.003zM12 8.25a.75.75 0 01.75.75v3.75a.75.75 0 01-1.5 0V9a.75.75 0 01.75-.75zm0 8.25a.75.75 0 100-1.5.75.75 0 000 1.5z"
                clipRule="evenodd"
            />
        </svg>
    );
}

function Notification({ node, expireNode }: NotificationProps) {
    const [scale, setScale] = useState(false);
    const timeoutsRef = useRef<NodeJS.Timeout[]>([]);

    const expireAfterAnimating = useMemo(
        () =>
            function () {
                // Animate
                setScale(false);
                const expireTimeout = setTimeout(() => {
                    expireNode(node.key);
                }, ANIMATION_DURATION_MS);

                timeoutsRef.current.push(expireTimeout);
            },
        [expireNode, node]
    );

    useEffect(() => {
        setScale(true);

        // Allow us to set infinite messages until a user clears it manually
        if (node.expiresInMs === Infinity) {
            return;
        }

        const timeout = setTimeout(() => {
            expireAfterAnimating();
        }, node.expiresInMs);

        timeoutsRef.current.push(timeout);
        const timeouts = timeoutsRef.current;

        return () => {
            for (const timeout in timeouts) {
                clearTimeout(timeout);
            }
        };
    }, [node.expiresInMs, expireAfterAnimating]);

    return (
        <div
            className={`transition-all ease-out duration-[${ANIMATION_DURATION_MS}] rounded-md p-2 m-2 bg-slate-100 border-2 border-slate-200 max-w-2 ${
                scale ? 'scale-100' : 'scale-0'
            }`}
        >
            <div className="w-64 flex flex-col space-y-2">
                <div className="w-full flex space-x-2 items-start">
                    <NotificationIcon node={node} />
                    {node.title ? (
                        <p className="flex-1 text-md align-middle font-semibold text-slate-700">
                            {node.title}
                        </p>
                    ) : (
                        <p className="w-full text-sm text-slate-500 break-words whitespace-normal">
                            {node.message}
                        </p>
                    )}
                    <button
                        onClick={expireAfterAnimating}
                        className="w-4 transition-all duration-200 ease-out text-slate-900 hover:text-slate-700 hover:scale-125"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            strokeWidth={2}
                            stroke="currentColor"
                            className="w-full h-full"
                        >
                            <path
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                d="M6 18L18 6M6 6l12 12"
                            />
                        </svg>
                    </button>
                </div>
                {node.title && (
                    <p className="w-full text-sm text-slate-500 break-words whitespace-normal">
                        {node.message}
                    </p>
                )}
            </div>
        </div>
    );
}

// Used the fast context to have the notifications render only within their own lil happy space
// @see https://github.com/jherr/fast-react-context/blob/main/fast-context-generic/src/createFastContext.tsx
export function NotificationProvider(props: NotificationProviderProps) {
    const store = useRef(new Map<string, NotificationNode>());
    const subscribers = useRef(new Set<() => void>());

    const getNodeMap = useCallback(() => store.current, []);

    const addNode = useCallback((node: NotificationNode) => {
        // Do not notify subscribers or update to new map reference
        // if we currently have a similar node.
        if (store.current.has(node.key)) {
            return node;
        }

        store.current.set(node.key, node);
        store.current = new Map(store.current);
        for (const cb of subscribers.current) {
            cb();
        }

        return node;
    }, []);

    const expireNode = useCallback((key: string) => {
        store.current.delete(key);
        store.current = new Map(store.current);
        for (const cb of subscribers.current) {
            cb();
        }
    }, []);

    const subscribe = useCallback((cb: () => void) => {
        subscribers.current.add(cb);
        return () => subscribers.current.delete(cb);
    }, []);

    // TODO: Add generic dull notification type
    const notify = useCallback(
        (params: NotificationFnParams & { type: NotificationType }) => {
            const node = createNotificationNode(params);
            return addNode(node);
        },
        [addNode]
    );

    const notifySuccess = useCallback(
        (params: Omit<NotificationFnParams, 'type'>): NotificationNode => {
            return notify({ ...params, type: NotificationType.SUCCESS });
        },
        [notify]
    );

    const notifyError = useCallback(
        (params: Omit<NotificationFnParams, 'type'>): NotificationNode => {
            return notify({ ...params, type: NotificationType.ERROR });
        },
        [notify]
    );

    const notifyWarning = useCallback(
        (params: Omit<NotificationFnParams, 'type'>): NotificationNode => {
            return notify({ ...params, type: NotificationType.WARNING });
        },
        [notify]
    );

    return (
        <Context.Provider
            value={{
                notify,
                notifySuccess,
                notifyWarning,
                notifyError,
                expireNode,
            }}
        >
            <InternalContext.Provider
                value={{ getNodeMap, subscribe, expireNode }}
            >
                <Hehe />
            </InternalContext.Provider>
            {props.children}
        </Context.Provider>
    );
}

function Hehe() {
    const { state, expireNode } = useInternalNotification();

    return (
        <div className="absolute right-0 z-50">
            {Array.from(state.values())
                .sort(
                    (a, b) =>
                        b.createdAt +
                        b.expiresInMs -
                        (a.createdAt + a.expiresInMs)
                )
                .map((node) => (
                    <Notification
                        key={node.key}
                        node={node}
                        expireNode={expireNode}
                    />
                ))}
        </div>
    );
}

function useInternalNotification() {
    const store = useContext(InternalContext);

    if (!store) {
        throw new Error('Store not found');
    }

    const state = useSyncExternalStore(store.subscribe, store.getNodeMap);

    return { state, expireNode: store.expireNode };
}

export default function useNotification() {
    const context = useContext(Context);

    if (!context) {
        throw new Error(
            '`useNotification` can only be used within a `<NotificationProvider />` provider.'
        );
    }

    return context;
}
