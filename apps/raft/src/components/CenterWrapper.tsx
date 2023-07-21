import type { ReactNode } from 'react';

type CenterWrapperProps = {
    children?: ReactNode | ReactNode[];
};

export default function CenterWrapper(props: CenterWrapperProps) {
    return (
        <div className="flex w-full justify-center">
            <div className="flex max-w-4xl w-full justify-center self-center">
                {props.children}
            </div>
        </div>
    );
}
