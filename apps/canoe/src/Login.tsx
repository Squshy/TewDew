import { useRef, useState } from 'react';
//
import { useLogin, buildFieldErrorMap } from './urql';
// Components
import InputField from './components/InputField';

export default function Login() {
    const usernameRef = useRef<HTMLInputElement>(null);
    const passwordRef = useRef<HTMLInputElement>(null);
    const [errors, setErrors] = useState<Record<string, string>>();
    const [state, login] = useLogin();

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const username = usernameRef.current?.value;
        const password = passwordRef.current?.value;

        // TODO: Some fancy error stuff even though this won't hit due to
        // HTML5 validation
        if (!username || !password) {
            return;
        }

        const result = await login({ password, username });
        const user = result.data?.login.user;
        const userErrors = result.data?.login.userErrors;

        if (userErrors) {
            const errorMap = buildFieldErrorMap(userErrors);
            setErrors(errorMap);
        } else if (user) {
            localStorage.setItem('x-token', user.token);
        }
    };

    return (
        <div className="flex flex-col w-full">
            <form
                onSubmit={handleSubmit}
                className="flex w-full justify-center"
            >
                <div className="flex flex-col w-full space-y-6 max-w-xl">
                    <h2 className="text-3xl text-center font-bold tracking-tight text-gray-800">
                        Login
                    </h2>
                    <InputField
                        label="Username"
                        id="username"
                        placeholder="Username"
                        type="text"
                        ref={usernameRef}
                        error={errors?.username}
                        required
                    />
                    <InputField
                        label="Password"
                        id="password"
                        placeholder="Password"
                        type="password"
                        ref={passwordRef}
                        error={errors?.password}
                        required
                    />
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
        </div>
    );
}
