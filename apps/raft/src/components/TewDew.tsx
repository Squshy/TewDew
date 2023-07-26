import { useContext, createContext, ReactNode } from 'react';
import { SlimTewDew as TewDew } from 'tewgql';

const Context = createContext<TewDew | null>(null);

function useTewDew() {
    const context = useContext(Context);

    if (!context) {
        throw new Error(
            '`useTewDew` can only be used within a `<TewDew />` provider.'
        );
    }

    return context;
}

function Title() {
    const { title } = useTewDew();

    return <div>{title}</div>;
}

function Description() {
    const { description } = useTewDew();

    if (!description) {
        return <div>No description provided</div>;
    }

    return <div>{description}</div>;
}

function Completed() {
    const { completed } = useTewDew();

    return <div>{completed.toString()}</div>;
}

const TewDewProvider = ({
    tewdew,
    children,
}: {
    tewdew: TewDew;
    children: ReactNode;
}) => {
    return <Context.Provider value={tewdew}>{children}</Context.Provider>;
};

const TewDew = Object.assign(TewDewProvider, {
    Title,
    Description,
    Completed,
});

export default TewDew;
