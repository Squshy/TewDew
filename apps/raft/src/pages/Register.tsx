import { useRef } from 'react';
import { useNavigate } from 'react-router-dom';
//
import { useAlert } from '@alertle/react';
import { useRegisterMutation } from '../urql';
import Routes from '../routes';
// Components
import InputField from '../components/InputField';
import useAuthContext from '../contexts/AuthContext';
import { formEntries } from '../utils/common';

export default function Register() {
    const formRef = useRef(null);
    const [, register] = useRegisterMutation();
    const { notifyError } = useAlert();
    const { storeLocalAuth } = useAuthContext();
    const navigate = useNavigate();

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        const { username, password } = formEntries<{
            username: string;
            password: string;
        }>(e);

        if (!username || !password) {
            return;
        }

        const result = await register({ username, password });
        const user = result.data?.register.user;
        const errors = result.error?.graphQLErrors;

        if (errors) {
            for (const err of errors) {
                notifyError({ message: err.message });
            }
        } else if (user) {
            storeLocalAuth(user);
            navigate(Routes.HOME);
        }
    };

    return (
        <div className="flex flex-col w-full">
            <form
                ref={formRef}
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
                        name="username"
                        required
                    />
                    <InputField
                        label="Password"
                        id="password"
                        placeholder="Password"
                        type="password"
                        name="password"
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
