import { useRef } from 'react';
// import { useNavigate } from 'react-router-dom';
//
import { useAlert } from '@alertle/react';
import { useRegisterMutation } from './urql';
import { setStoredItem, StorageKey } from './utils/local-storage';
// Components
import InputField from './components/InputField';

export default function Register() {
    const usernameRef = useRef<HTMLInputElement>(null);
    const passwordRef = useRef<HTMLInputElement>(null);
    const [, register] = useRegisterMutation();
    const { notifyError } = useAlert();

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const username = usernameRef.current?.value;
        const password = passwordRef.current?.value;

        if (!username || !password) {
            return;
        }

        const result = await register({ username, password });
        const user = result.data?.register.user;
        const userErrors = result.data?.register.userErrors;

        if (userErrors) {
            for (const err of userErrors) {
                notifyError({ message: err.message });
            }
        } else if (user) {
            setStoredItem(StorageKey.AUTH, user.token);
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
                        Register
                    </h2>
                    <InputField
                        label="Username"
                        id="username"
                        placeholder="Username"
                        type="text"
                        ref={usernameRef}
                        required
                    />
                    <InputField
                        label="Password"
                        id="password"
                        placeholder="Password"
                        type="password"
                        ref={passwordRef}
                        required
                    />
                    <div>
                        <button
                            type="submit"
                            className="w-full bg-indigo-600 p-2 rounded-md text-white text-sm font-semibold hover:bg-indigo-500 transition duration-250 ease-out"
                        >
                            Register
                        </button>
                    </div>
                </div>
            </form>
        </div>
    );
}
