import { useRef } from 'react';
import useNotification from './contexts/Notifications';

export default function LandingPage() {
    const { showSuccess, showError, showWarning } = useNotification();
    const ref = useRef<HTMLInputElement>(null);

    return (
        <div className="w-full h-full bg-red-50 flex space-x-2">
            <input type="text" ref={ref} />
            <div>Hey!</div>
            <button
                onClick={() =>
                    showSuccess({
                        message: ref.current?.value || 'nice!',
                        expiresInMs: Infinity,
                    })
                }
            >
                Success
            </button>
            <button
                onClick={() =>
                    showError({ message: 'nice!', title: `${Math.random()}` })
                }
            >
                Error
            </button>
            <button onClick={() => showWarning({ message: 'nice!' })}>
                Warning
            </button>
        </div>
    );
}
