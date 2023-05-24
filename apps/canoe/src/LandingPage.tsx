import useNotification from './contexts/Notifications';

export default function LandingPage() {
    const { showSuccess, showError, showWarning } = useNotification();

    return (
        <div className="w-full h-full bg-red-50 flex space-x-2">
            <div>Hey!</div>
            <button
                onClick={() =>
                    showSuccess({ message: 'nice!', expiresInMs: Infinity })
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
