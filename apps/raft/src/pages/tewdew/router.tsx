import type { RouteObject } from 'react-router-dom';
//
import Routes from '../../routes';
import AuthPageWrapper from '../../components/AuthPageWrapper';
// Tewdew pages
import CreateTewDew from './CreateTewDew';
import ViewTewDew from './ViewTewDew';
import ListTewDews from './ListTewDews';

const tewdewRouter: RouteObject = {
    path: Routes.TEW_DEW_HOME,
    element: <AuthPageWrapper />,
    children: [
        {
            path: Routes.CREATE_TEW_DEW,
            element: <CreateTewDew />,
        },
        {
            path: Routes.LIST_TEW_DEWS,
            element: <ListTewDews />,
        },
        {
            path: Routes.VIEW_TEW_DEW(':id'),
            element: <ViewTewDew />,
        },
    ],
};

export default tewdewRouter;
