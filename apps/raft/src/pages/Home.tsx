import { useEffect } from 'react';
import { useNavigate, Outlet } from 'react-router-dom';
import useAuthContext from '../contexts/AuthContext';
import Routes from '../routes';

export default function Home() {
    const { state } = useAuthContext();
    const navigate = useNavigate();

    // TODO: Handle cases with failed auth due to expired JWT (in urql maybe?)
    useEffect(() => {
        if (!state.user) {
            navigate(Routes.LOGIN);
        }
    }, [state, navigate]);

    return <Outlet />;
}
