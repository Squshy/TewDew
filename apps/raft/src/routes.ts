export type Route = typeof Routes[keyof typeof Routes];

const Routes = {
    ROOT: '/',
    REGISTER: '/register',
    LOGIN: '/login',
    HOME: '/home',
    TEW_DEW_HOME: '/tewdews/',
    LIST_TEW_DEWS: '/tewdews/list',
    CREATE_TEW_DEW: '/tewdews/create',
    VIEW_TEW_DEW: (id: string) => `/tewdews/${id}`,
} as const;

export default Routes;
