import type { ReactNode } from 'react';
import { Outlet } from 'react-router-dom';
//
import { UrqlClientProvider } from '../urql';

type WrapperProps = {
    children?: ReactNode | ReactNode[];
};

export function Wrapper(props: WrapperProps) {
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
        <UrqlClientProvider>
            <div className="flex w-full h-full">
                <Wrapper>
                    <Outlet />
                </Wrapper>
            </div>
        </UrqlClientProvider>
    );
}
