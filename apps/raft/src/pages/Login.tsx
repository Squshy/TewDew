import { useRef } from 'react';
import { useAlert } from '@alertle/react';
import { Link, useNavigate } from 'react-router-dom';
//
import { useLoginMutation } from '../urql';
import { formEntries } from '../utils/common';
import Routes from '../routes';
import useAuthContext from '../contexts/AuthContext';
// Components
import InputField from '../components/InputField';
import CenterWrapper from '../components/CenterWrapper';

export default function Login() {
    const formRef = useRef(null);
    const { notifyError } = useAlert();
    const [, login] = useLoginMutation();
    const { storeLocalAuth } = useAuthContext();
    const navigate = useNavigate();

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const { username, password } = formEntries<{
            username: string;
            password: string;
        }>(e);

        // TODO: Some fancy error stuff even though this won't hit due to
        // HTML5 validation
        if (!username || !password) {
            return;
        }

        const result = await login({ password, username });
        const user = result.data?.login.user;
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
        <CenterWrapper>
            <div className="flex flex-col w-full">
                <form
                    ref={formRef}
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
                                Sign in
                            </button>
                        </div>
                        <p className="text-xs">
                            Don&apos;t have an account? Sign up{' '}
                            <Link
                                to={Routes.REGISTER}
                                className="text-indigo-600"
                            >
                                here
                            </Link>
                        </p>
                    </div>
                </form>
            </div>
        </CenterWrapper>
    );
}
