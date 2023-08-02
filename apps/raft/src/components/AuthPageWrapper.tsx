import {
    FormEvent,
    useEffect,
    useState,
    useRef,
    ReactNode,
    SVGProps,
    Dispatch,
    SetStateAction,
} from 'react';
import { Link, useNavigate, Outlet } from 'react-router-dom';
import useAuthContext from '../contexts/AuthContext';
import Routes from '../routes';
// Icons
import PlusCircleIcon from './icons/outline/PlusCircle';
import ListBulletIcon from './icons/outline/ListBullet';
import HomeIcon from './icons/outline/Home';

function UserMenu() {
    const containerRef = useRef<HTMLDivElement>(null);
    const [isOpen, setIsOpen] = useState(false);
    const { state, clearLocalAuth } = useAuthContext<false>();
    const navigate = useNavigate();

    useEffect(() => {
        const div = containerRef.current;

        if (!div) {
            return;
        }

        function handleOutsideClick(e: MouseEvent) {
            if (isOpen && !div?.contains(e.target as Node | null)) {
                setIsOpen(false);
            }
        }

        document.addEventListener('mousedown', handleOutsideClick);

        return () => {
            document.removeEventListener('mousedown', handleOutsideClick);
        };
    }, [isOpen]);

    function handleSubmit(e: FormEvent<HTMLFormElement>) {
        e.preventDefault();
        clearLocalAuth();
        navigate(Routes.LOGIN);
    }

    if (!state.user) {
        return null;
    }

    return (
        <div className="relative inline-block text-left" ref={containerRef}>
            <div>
                <button
                    type="button"
                    className="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
                    id="menu-button"
                    aria-expanded="true"
                    aria-haspopup="true"
                    onClick={() => setIsOpen(!isOpen)}
                >
                    {state.user.username}
                    <span className="sr-only">Open user menu</span>
                </button>
            </div>

            {isOpen && (
                <div
                    className="absolute right-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                    role="menu"
                    aria-orientation="vertical"
                    aria-labelledby="menu-button"
                    tabIndex={-1}
                >
                    <div className="py-1" role="none">
                        <form onSubmit={handleSubmit} role="none">
                            <button
                                type="submit"
                                className="text-gray-700 block w-full px-4 py-2 text-left text-sm"
                                role="menuitem"
                                tabIndex={-1}
                                id="menu-item-3"
                            >
                                Sign out
                            </button>
                        </form>
                    </div>
                </div>
            )}
        </div>
    );
}

function SideLink({
    linkTo,
    icon,
    title,
    closeSidebar,
}: {
    linkTo: string;
    icon: ((props?: SVGProps<SVGSVGElement>) => JSX.Element) | JSX.Element;
    title: string;
    closeSidebar: () => void;
}) {
    return (
        <li>
            <Link
                to={linkTo}
                className="flex items-center p-2 text-gray-900 rounded-lg hover:bg-gray-100"
                onClick={closeSidebar}
            >
                {typeof icon === 'function' ? icon() : icon}
                <span className="ml-3">{title}</span>
            </Link>
        </li>
    );
}

function Sidebar({
    isOpen,
    setIsOpen,
}: {
    isOpen: boolean;
    setIsOpen: Dispatch<SetStateAction<boolean>>;
}) {
    function closeSidebar() {
        setIsOpen(false);
    }

    return (
        <div
            id="logo-sidebar"
            className={`${
                isOpen ? 'fixed z-40' : 'hidden'
            } sm:flex w-64 h-full transition-transform bg-white border-r border-gray-200 ${
                isOpen ? 'translate-x-0' : 'sm:translate-x-0'
            }`}
            aria-label="Sidebar"
        >
            <div className="w-full h-full p-3 pb-4 overflow-y-auto bg-white">
                <ul className="space-y-2 font-medium">
                    <SideLink
                        linkTo={Routes.HOME}
                        icon={HomeIcon}
                        title="Home"
                        closeSidebar={closeSidebar}
                    />
                    <SideLink
                        linkTo={Routes.CREATE_TEW_DEW}
                        icon={PlusCircleIcon}
                        title="Create Tew Dew"
                        closeSidebar={closeSidebar}
                    />
                    <SideLink
                        linkTo={Routes.LIST_TEW_DEWS}
                        icon={ListBulletIcon}
                        title="List Tew Dews"
                        closeSidebar={closeSidebar}
                    />
                </ul>
            </div>
        </div>
    );
}

function NavWrapper({ children }: { children: ReactNode }) {
    const [isSidebarOpen, setIsSidebarOpen] = useState(false);

    return (
        <div className="flex flex-col w-screen h-screen">
            <nav className="flex w-full bg-white border-b border-gray-200 h-16">
                <div className="w-full px-3 py-3 lg:px-5 lg:pl-3">
                    <div className="flex items-center justify-between">
                        <div className="flex items-center justify-start">
                            <button
                                data-drawer-target="logo-sidebar"
                                data-drawer-toggle="logo-sidebar"
                                aria-controls="logo-sidebar"
                                type="button"
                                className="inline-flex items-center p-2 text-sm text-gray-500 rounded-lg sm:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200"
                                onClick={() => setIsSidebarOpen(!isSidebarOpen)}
                            >
                                <span className="sr-only">Open sidebar</span>
                                <svg
                                    className="w-6 h-6"
                                    aria-hidden="true"
                                    fill="currentColor"
                                    viewBox="0 0 20 20"
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <path
                                        clipRule="evenodd"
                                        fillRule="evenodd"
                                        d="M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10z"
                                    ></path>
                                </svg>
                            </button>
                            <a className="flex ml-2 md:mr-24">
                                <img
                                    src="https://flowbite.com/docs/images/logo.svg"
                                    className="h-8 mr-3"
                                    alt="FlowBite Logo"
                                />
                                <span className="self-center text-xl font-semibold sm:text-2xl whitespace-nowrap">
                                    TewDew
                                </span>
                            </a>
                        </div>
                        <UserMenu />
                    </div>
                </div>
            </nav>
            <div className="flex w-full h-full">
                <Sidebar isOpen={isSidebarOpen} setIsOpen={setIsSidebarOpen} />
                <div className="flex px-4 py-6 w-full h-full bg-gray-100">
                    {children}
                </div>
            </div>
        </div>
    );
}

export default function AuthPageWrapper() {
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
            <div className="flex w-full h-full bg-white p-6 rounded-md">
                <Outlet />
            </div>
        </NavWrapper>
    );
}
