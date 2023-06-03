import { useNavigate } from 'react-router-dom';
import Routes from './routes/routes';

export default function LandingPage() {
    const navigate = useNavigate();

    return (
        <div className="w-full h-full flex flex-col space-y-2">
            <div>
                <p className="text-xl">Welcome</p>
            </div>
            <div className="flex space-x-2">
                <div className="flex flex-col w-full">
                    <p>Don&apos;t have an account?</p>
                    <button
                        className="w-full h-full p-2 rounded-md bg-blue-500 text-white"
                        onClick={() => navigate(Routes.REGISTER)}
                    >
                        Register
                    </button>
                </div>
                <div className="flex flex-col w-full">
                    <p>Already have an account?</p>
                    <button
                        className="w-full h-full p-2 rounded-md bg-blue-500 text-white"
                        onClick={() => navigate(Routes.LOGIN)}
                    >
                        Login
                    </button>
                </div>
            </div>
        </div>
    );
}
