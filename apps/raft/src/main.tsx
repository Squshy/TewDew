import React from 'react';
import ReactDOM from 'react-dom/client';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import './index.css';
import Routes from './routes';
// Components
import Root from './Root';
import ErrorPage from './ErrorPage';
import Login from './pages/Login';
import Register from './pages/Register';
import LandingPage from './pages/LandingPage';
import Home from './pages/Home';

const router = createBrowserRouter([
    {
        path: Routes.ROOT,
        element: <Root />,
        errorElement: <ErrorPage />,
        children: [
            {
                index: true,
                element: <LandingPage />,
            },
            {
                path: Routes.LOGIN,
                element: <Login />,
            },
            {
                path: Routes.REGISTER,
                element: <Register />,
            },
            {
                path: Routes.HOME,
                element: <Home />,
                children: [
                    {
                        index: true,
                        element: <div>hey beans</div>,
                    },
                ],
            },
        ],
    },
]);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <RouterProvider router={router} />
    </React.StrictMode>
);
