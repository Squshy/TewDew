import React from 'react';
import ReactDOM from 'react-dom/client';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import './index.css';
import Routes from './routes/routes';
// Components
import Root from './routes/root';
import ErrorPage from './ErrorPage';
import Login from './Login';
import Register from './Register';
import LandingPage from './LandingPage';

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
        ],
    },
]);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <RouterProvider router={router} />
    </React.StrictMode>
);
