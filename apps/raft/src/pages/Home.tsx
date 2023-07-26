import { useEffect } from 'react';
import { useNavigate, Outlet } from 'react-router-dom';
//
import useAuthContext from '../contexts/AuthContext';
import Routes from '../routes';
// Components
import NavWrapper from '../components/NavWrapper';

export function HomeWrapper() {
    const { state } = useAuthContext<false>();
    const navigate = useNavigate();

    // TODO: Handle cases with failed auth due to expired JWT (in urql maybe?)
    useEffect(() => {
        if (!state.user) {
            navigate(Routes.LOGIN, { replace: true });
        }
    }, [state, navigate]);

    if (!state.user) {
        return null;
    }

    return (
        <NavWrapper>
            <Outlet />
        </NavWrapper>
    );
}

export default function Home() {
    return (
        <div className="flex justify-center w-full bg-white p-6 rounded-md">
            <p>
                This is a simple to-do tracking application with a robust
                back-end powered by Rust and GraphQL using the Actix web
                framework and async-graphql.
            </p>
        </div>
    );
}
