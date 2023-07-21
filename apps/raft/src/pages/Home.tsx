import { FormEvent, useEffect, useState, useRef } from 'react';
import { useNavigate, Outlet } from 'react-router-dom';
//
import useAuthContext from '../contexts/AuthContext';
import Routes from '../routes';

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

    return <Outlet />;
}

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
            if (!div?.contains(e.target as Node | null)) {
                setIsOpen(false);
            }
        }

        document.addEventListener('mousedown', handleOutsideClick);

        return () => {
            document.removeEventListener('mousedown', handleOutsideClick);
        };
    }, []);

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

function Navbar() {
    const [isSidebarOpen, setIsSidebarOpen] = useState(false);
    return (
        <>
            <nav className="fixed top-0 z-50 w-full bg-white border-b border-gray-200">
                <div className="px-3 py-3 lg:px-5 lg:pl-3">
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
                                        clip-rule="evenodd"
                                        fill-rule="evenodd"
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
            <Sidebar isOpen={isSidebarOpen} />
        </>
    );
}

function Sidebar({ isOpen }: { isOpen: boolean }) {
    return (
        <aside
            id="logo-sidebar"
            className={`fixed top-0 left-0 z-40 w-64 h-screen pt-20 transition-transform -translate-x-full bg-white border-r border-gray-200 ${
                isOpen ? 'translate-x-0' : 'sm:translate-x-0'
            }`}
            aria-label="Sidebar"
        >
            <div className="h-full px-3 pb-4 overflow-y-auto bg-white">
                <ul className="space-y-2 font-medium">
                    <li>
                        <a
                            href="#"
                            className="flex items-center p-2 text-gray-900 rounded-lg hover:bg-gray-100"
                        >
                            <svg
                                className="w-5 h-5 text-gray-500 transition duration-75 group-hover:text-gray-900"
                                aria-hidden="true"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="currentColor"
                                viewBox="0 0 22 21"
                            >
                                <path d="M16.975 11H10V4.025a1 1 0 0 0-1.066-.998 8.5 8.5 0 1 0 9.039 9.039.999.999 0 0 0-1-1.066h.002Z" />
                                <path d="M12.5 0c-.157 0-.311.01-.565.027A1 1 0 0 0 11 1.02V10h8.975a1 1 0 0 0 1-.935c.013-.188.028-.374.028-.565A8.51 8.51 0 0 0 12.5 0Z" />
                            </svg>
                            <span className="ml-3">Dashboard</span>
                        </a>
                    </li>
                </ul>
            </div>
        </aside>
    );
}

export default function Home() {
    const { state } = useAuthContext();

    return (
        <div className="w-full">
            <Navbar />
            <div className="p-4 sm:ml-64 mt-14 w-full h-full">
                <div className="w-full bg-red-500 h-12">
                    Hey {state.user.username}
                </div>
            </div>
        </div>
    );
}
