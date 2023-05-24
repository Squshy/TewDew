import React, {
    useState,
    createContext,
    useContext,
    useCallback,
    useEffect,
    useRef,
    useMemo,
} from 'react';

const DEFAULT_MESSAGE_EXPIRY_TIME_MS = 3500; // 3.5 seconds

type NotificationFnParams = {
    message: string;
    title?: string;
    expiresInMs?: number;
};

type NotificationFn = (args: NotificationFnParams) => void;

type NotificationContext = {
    showSuccess: NotificationFn;
    showError: NotificationFn;
    showWarning: NotificationFn;
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
    title: string;
    type: NotificationType;
    createdAt: number;
    expiresInMs: number;
};

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
    title?: string;
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
        createdAt: Date.now(),
        title: title ?? `${type.charAt(0).toUpperCase()}${type.slice(1)}`,
    };
}

type NotificationProps = {
    node: NotificationNode;
    expireNode: (node: NotificationNode) => void;
};

function notificationColorFromType(type: NotificationType): string {
    switch (type) {
        case 'error':
            return 'bg-red-500';
        case 'success':
            return 'bg-green-500';
        case 'warning':
            return 'bg-yellow-500';
    }
}

const ANIMATION_DURATION_MS = 500;
function Notification({ node, expireNode }: NotificationProps) {
    const [scale, setScale] = useState(false);
    const timeoutsRef = useRef<NodeJS.Timeout[]>([]);

    const expireAfterAnimating = useMemo(
        () =>
            function () {
                const animationTimeout = setTimeout(() => {
                    setScale(false);

                    const expireTimeout = setTimeout(() => {
                        expireNode(node);
                    }, ANIMATION_DURATION_MS);

                    timeoutsRef.current.push(expireTimeout);
                }, node.expiresInMs);

                timeoutsRef.current.push(animationTimeout);
            },
        [expireNode, node]
    );

    useEffect(() => {
        setScale(true);

        // Allow us to set infinite messages until a user clears it manually
        if (node.expiresInMs === Infinity) {
            return;
        }

        expireAfterAnimating();
        const timeouts = timeoutsRef.current;

        return () => {
            for (const timeout in timeouts) {
                clearTimeout(timeout);
            }
        };
    }, [node.expiresInMs, expireAfterAnimating]);

    const bgColor = notificationColorFromType(node.type);

    return (
        <div
            className={`transition-all ease-out duration-[${ANIMATION_DURATION_MS}] rounded-md p-2 m-2 ${bgColor} max-w-2 ${
                scale ? 'scale-100' : 'scale-0'
            }`}
        >
            <p>Key: {node.key}</p>
            <p>Title: {node.title}</p>
            <p>Message: {node.message}</p>
            <p>Type: {node.type}</p>
            <p>Expres In: {node.expiresInMs}</p>
            <p>Created At: {node.createdAt}</p>
            <button onClick={expireAfterAnimating}>Expire</button>
        </div>
    );
}

export function NotificationProvider(props: NotificationProviderProps) {
    const [nodeMap, setNodeMap] = useState<Map<string, NotificationNode>>(
        new Map()
    );

    const expireNode = useCallback((node: NotificationNode): void => {
        setNodeMap((prev) => {
            prev.delete(node.key);
            return new Map(prev);
        });
    }, []);

    const addNode = useCallback((node: NotificationNode): void => {
        setNodeMap((prev) => {
            prev.set(node.key, node);
            return new Map(prev);
        });
    }, []);

    const showSuccess = useCallback((params: NotificationFnParams): void => {
        const node = createNotificationNode({
            type: NotificationType.SUCCESS,
            ...params,
        });

        addNode(node);
    }, []);

    const showError = useCallback((params: NotificationFnParams): void => {
        const node = createNotificationNode({
            type: NotificationType.ERROR,
            ...params,
        });

        addNode(node);
    }, []);

    const showWarning = useCallback((params: NotificationFnParams): void => {
        const node = createNotificationNode({
            type: NotificationType.WARNING,
            ...params,
        });

        addNode(node);
    }, []);

    const nodeList = Array.from(nodeMap).sort(
        ([, a], [, b]) => b.createdAt - a.createdAt
    );

    return (
        <Context.Provider value={{ showSuccess, showWarning, showError }}>
            <div className="absolute right-0 z-50">
                {nodeList.map(([key, node]) => (
                    <Notification
                        key={key}
                        node={node}
                        expireNode={expireNode}
                    />
                ))}
            </div>
            {props.children}
        </Context.Provider>
    );
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
