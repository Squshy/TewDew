import { useNavigate } from 'react-router-dom';
import { useAlert } from '@alertle/react';
import type { SlimTewDew, TewDewErrorsFragment } from 'tewgql';
//
import { useCreateTewDewMutation } from '../../urql/mutations';
import { formEntries } from '../../utils/common';
import Routes from '../../routes';
// Components
import InputField from '../../components/InputField';

type CreateTewDewFields = Required<Pick<SlimTewDew, 'description' | 'title'>>;

export default function CreateTewDew() {
    const { notifyError, notifySuccess } = useAlert();
    const navigate = useNavigate();
    const [{ fetching }, createTewDew] = useCreateTewDewMutation();

    async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault();
        const data = formEntries<CreateTewDewFields>(e);

        const result = await createTewDew(data);
        const tewdew = result.data?.createTewDew.tewDew;
        const errors =
            result.data?.createTewDew.tewDewErrors ??
            result.error?.graphQLErrors;

        if (errors) {
            for (const err of errors) {
                const title = (
                    err as TewDewErrorsFragment
                ).field?.toLocaleUpperCase();
                notifyError({ message: err.message, title });
            }
        } else if (tewdew) {
            notifySuccess({ message: 'Created tew dew' });
            navigate(Routes.VIEW_TEW_DEW(tewdew.id));
        }
    }

    if (fetching) {
        // TODO: Better loading views
        return <div>Loading...</div>;
    }

    return (
        <div className="flex flex-col w-full -m-2">
            <h1 className="font-sans text-3xl font-bold text-gray-800 m-2">
                Create a Tew Dew
            </h1>
            <hr className="w-full m-2" />
            <form onSubmit={handleSubmit}>
                <div className="flex flex-col w-full h-full p-3 space-y-4">
                    <InputField
                        name="title"
                        label="Title"
                        placeholder="Title"
                        type="text"
                        id="title"
                        required
                    />
                    <InputField
                        name="description"
                        label="Description"
                        placeholder="Description"
                        type="text"
                        id="description"
                    />
                    <button
                        type="submit"
                        className="w-full bg-indigo-600 p-2 rounded-md text-white text-sm font-semibold hover:bg-indigo-500 transition duration-250 ease-out"
                    >
                        Create tew dew
                    </button>
                </div>
            </form>
        </div>
    );
}
