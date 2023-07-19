import type { ReactNode } from 'react';
import { Outlet } from 'react-router-dom';
import { AlertProvider } from '@alertle/react';
//
import { UrqlClientProvider } from '../urql';
//
import AlertContainer from '../components/Alert';

type WrapperProps = {
    children?: ReactNode | ReactNode[];
};

function Wrapper(props: WrapperProps) {
    return (
        <div className="flex w-full justify-center">
            <div className="flex max-w-4xl w-full justify-center self-center">
                {props.children}
            </div>
        </div>
    );
}

export default function Root() {
    return (
        <AlertProvider alertContainer={<AlertContainer />}>
            <UrqlClientProvider>
                <div className="flex w-full h-full">
                    <Wrapper>
                        <Outlet />
                    </Wrapper>
                </div>
            </UrqlClientProvider>
        </AlertProvider>
    );
}
