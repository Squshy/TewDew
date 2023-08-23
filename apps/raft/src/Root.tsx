import { Outlet } from 'react-router-dom';
import { AlertProvider } from '@alertle/react';
//
import UrqlClientProvider from './urql/client';
import AlertContainer from './components/Alert';
import { AuthProvider } from './contexts/AuthContext';

export default function Root() {
    return (
        <AlertProvider
            alertContainer={<AlertContainer />}
            defaultExpiresInMs={3500}
        >
            <AuthProvider>
                <UrqlClientProvider>
                    <div className="flex w-full h-full">
                        <Outlet />
                    </div>
                </UrqlClientProvider>
            </AuthProvider>
        </AlertProvider>
    );
}
