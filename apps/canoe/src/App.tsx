import { useRef, ReactNode } from 'react';
//
import { UrqlClientProvider, useLogin } from './urql';

type WrapperProps = {
    children?: ReactNode | ReactNode[];
};

function Wrapper(props: WrapperProps) {
    return (
        <div className="flex w-full justify-center">
            <div className="flex max-w-4xl w-full justify-center self-center">
                {props.children}
            </div>
        </div>
    );
}

function LoginForm() {
    const usernameRef = useRef<HTMLInputElement>(null);
    const passwordRef = useRef<HTMLInputElement>(null);
    const [state, login] = useLogin();

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const username = usernameRef.current?.value;
        const password = passwordRef.current?.value;
        // TODO: Some fancy stuff
        if (!username || !password) {
            return;
        }
        const result = await login({ password, username });
        console.log({ result });
    };

    return (
        <form onSubmit={handleSubmit} className="flex w-full justify-center">
            <div className="flex flex-col w-full space-y-6 max-w-xl">
                <div>
                    <h2 className="text-3xl text-center font-bold tracking-tight text-gray-800">
                        Login
                    </h2>
                </div>
                <div>
                    <label htmlFor="username" className="sr-only">
                        Username
                    </label>
                    <input
                        type="text"
                        id="username"
                        ref={usernameRef}
                        placeholder="Username"
                        className="relative block w-full rounded-md border-0 p-2.5 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:z-10 focus:ring-2 focus:ring-inset focus:ring-indigo-600 focus:outline-none sm:text-sm sm:leading-6 drop-shadow"
                        required
                    />
                </div>
                <div>
                    <label htmlFor="username" className="sr-only">
                        Password
                    </label>
                    <input
                        type="password"
                        id="password"
                        ref={passwordRef}
                        placeholder="Password"
                        className="relative block w-full rounded-md border-0 p-2.5 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:z-10 focus:ring-2 focus:ring-inset focus:ring-indigo-600 focus:outline-none sm:text-sm sm:leading-6 drop-shadow"
                        required
                    />
                </div>
                <div>
                    <button
                        type="submit"
                        className="w-full bg-indigo-600 p-2 rounded-md text-white text-sm font-semibold hover:bg-indigo-500 transition duration-250 ease-out"
                    >
                        Sign in
                    </button>
                </div>
                {state?.errors && (
                    <div>
                        {state?.errors.map((err) => (
                            <p key={err} className="text-red-500">
                                {err}
                            </p>
                        ))}
                    </div>
                )}
            </div>
        </form>
    );
}

function App() {
    return (
        <UrqlClientProvider>
            <div className="flex w-full h-full">
                <Wrapper>
                    <div className="flex flex-col w-full">
                        <LoginForm />
                    </div>
                </Wrapper>
            </div>
        </UrqlClientProvider>
    );
}

export default App;
