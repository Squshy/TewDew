import { useNavigate } from 'react-router-dom';
import { useAlert } from '@alertle/react';
//
import type { SlimTewDew } from 'tewgql';
import Routes from '../../routes';
import { useListTewDewsQuery } from '../../urql/queries';
//
import Checkbox from '../../components/Checkbox';

export default function ListTewDews() {
    const navigate = useNavigate();
    const { notifyError } = useAlert();
    const [{ data, fetching, error }] = useListTewDewsQuery();

    if (fetching) {
        return <div>Loading...</div>;
    }

    if (error) {
        const errors = error?.graphQLErrors ?? [
            { message: 'Something went oopsy' },
        ];
        for (const err of errors) {
            notifyError({ message: err.message });
        }

        navigate(Routes.TEW_DEW_HOME);
        return null;
    }

    const tewdews = data?.listTewDews;

    if (!tewdews?.length) {
        return <div>No data!</div>;
    }

    function handleRowClick(tewdew: Pick<SlimTewDew, 'id'>) {
        navigate(Routes.VIEW_TEW_DEW(tewdew.id));
    }

    return (
        <div className="flex flex-col w-full -m-2">
            <h1 className="font-sans text-3xl font-bold text-gray-800 m-2">
                TewDews
            </h1>
            <hr className="w-full m-2" />
            <table className="w-full table-fixed text-left m-2">
                <thead className="w-full uppercase text-xs bg-gray-50">
                    <tr>
                        <th scope="col" className="w-5/12 px-6 py-3">
                            Title
                        </th>
                        <th scope="col" className="w-5/12 px-6 py-3">
                            Description
                        </th>
                        <th scope="col" className="w-2/12 px-6 py-3">
                            Completed
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {tewdews.map((tewdew) => (
                        <tr
                            key={tewdew.id}
                            className="hover:bg-gray-100 border-b cursor-pointer"
                            onClick={() => handleRowClick(tewdew)}
                        >
                            <td className="px-6 py-3">{tewdew.title}</td>
                            <td className="px-6 py-3">{tewdew.description}</td>
                            <td className="px-6 py-3">
                                <Checkbox checked={tewdew.completed} disabled />
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
}
