import { useRouteError } from 'react-router-dom';

export default function ErrorPage() {
    // TODO: Fix any and create custom error type we expect
    const error = useRouteError() as any;
    console.error(error);

    return (
        <div className="w-full h-screen flex flex-col justify-center align-center text-center space-y-2">
            <h1 className="text-3xl font-semibold">Oops!</h1>
            <p className="text-lg">Sorry, an unexpected error has occurred.</p>
            <p className="text-gray-500">
                <i>{error.statusText || error.message}</i>
            </p>
        </div>
    );
}
