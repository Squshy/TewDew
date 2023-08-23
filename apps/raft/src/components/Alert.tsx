import { useState, useEffect, useRef } from 'react';
import { useAlertContainer, Alert } from '@alertle/react';

const ANIMATION_DURATION_MS = 250;

type ExpireAlertFn = ReturnType<typeof useAlertContainer>['expireAlert'];
type UpdateAlertFn = ReturnType<typeof useAlertContainer>['updateAlert'];

type NotificationProps = {
    node: Alert;
    expireNode: ExpireAlertFn;
    updateNode: UpdateAlertFn;
};

function NotificationIcon({ node }: { node: Alert }) {
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

function Notification({ node, expireNode, updateNode }: NotificationProps) {
    const [scale, setScale] = useState(false);
    const animationTimeoutRef = useRef<NodeJS.Timeout>();

    useEffect(() => {
        setScale(true);

        // Allow us to set infinite messages until a user clears it manually
        if (!node.expiresInMs || node.expiresInMs === Infinity) {
            return;
        }

        const timeout = setTimeout(() => {
            setScale(false);
        }, Math.max(node.expiresInMs - 500, 500));

        updateNode(node, {
            onDuplicated: () => {
                clearTimeout(timeout);
            },
        });

        const animationTimeout = animationTimeoutRef.current;
        return () => {
            clearTimeout(timeout);
            clearTimeout(animationTimeout);
        };
    }, [node, updateNode]);

    function expireAfterAnimating() {
        setScale(false);
        const timeout = setTimeout(() => {
            expireNode(node);
        }, ANIMATION_DURATION_MS);
        animationTimeoutRef.current = timeout;
    }

    function hehe() {
        updateNode(node, { isDuplicate: false });
    }

    return (
        <div
            className={`transition-all ease-out duration-[${ANIMATION_DURATION_MS}] rounded-md p-3 m-2 bg-slate-100
             border-2 border-slate-200 max-w-2 ${
                 scale ? 'scale-100' : 'scale-0'
             } ${node.isDuplicate ? 'animate-wiggle' : ''}`}
            onMouseOver={hehe}
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

export default function AlertContainer() {
    const { alerts, expireAlert, updateAlert } = useAlertContainer();

    return (
        <div className="absolute right-0 bottom-0 z-50">
            {alerts.map((node) => (
                <Notification
                    key={node.key}
                    node={node}
                    expireNode={expireAlert}
                    updateNode={updateAlert}
                />
            ))}
        </div>
    );
}
