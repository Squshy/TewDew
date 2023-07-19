export type Route = typeof Routes[keyof typeof Routes];

const Routes = {
    ROOT: '/',
    REGISTER: '/register',
    LOGIN: '/login',
} as const;

export default Routes;
