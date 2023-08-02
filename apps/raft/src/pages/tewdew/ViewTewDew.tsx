import type { FormEvent } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useAlert } from '@alertle/react';
//
import type { Task } from 'tewgql';
import { useTewDewQuery } from '../../urql/queries';
import {
    useCreateTaskMutation,
    useUpdateTaskMutation,
} from '../../urql/mutations';
import Routes from '../../routes';
// Icons
import CheckCircleOutlineIcon from '../../components/icons/outline/CheckCircle';
import CheckCircleSolidIcon from '../../components/icons/solid/CheckCircle';
import PlusCircleIcon from '../../components/icons/solid/PlusCircle';
// Components
import Checkbox from '../../components/Checkbox';
import InputField from '../../components/InputField';
import { formEntries } from '../../utils/common';

function TaskRow({ task }: { task: Task }) {
    const { notifySuccess } = useAlert();
    const [{ fetching }, updateTask] = useUpdateTaskMutation();

    async function handleUpdate() {
        const hehe = await updateTask({ taskId: task.id, completed });
        notifySuccess({ message: 'Updated task' });
    }

    return (
        <tr
            className="hover:bg-gray-100 border-b cursor-pointer"
            onClick={() => console.log('ha!')}
        >
            <td className="px-6 py-3">
                <InputField
                    defaultValue={task.title}
                    id="title"
                    label="Title"
                    name="title"
                />
            </td>
            <td className="px-6 py-3">
                <Checkbox
                    defaultChecked={task.completed}
                    disabled={fetching}
                    name="completed"
                    id="completed"
                />
            </td>
        </tr>
    );
}

function AddTaskRow({ tewdewId }: { tewdewId: string }) {
    const { notifyError, notifySuccess } = useAlert();
    const [{ fetching }, addTask] = useCreateTaskMutation();

    async function handleAddTask(e: FormEvent<HTMLFormElement>) {
        e.preventDefault();
        const completed: boolean = (
            e.currentTarget.completed as HTMLInputElement
        ).checked;
        const title: string = (
            e.currentTarget.title as unknown as HTMLInputElement
        ).value;

        const result = await addTask({ tewdewId, completed, title });

        const task = result.data?.createTask.task;
        const taskErrors = result.data?.createTask.taskErrors;
        const errors = result.error?.graphQLErrors;

        if (errors) {
            for (const err of errors) {
                notifyError({ message: err.message });
            }
        } else if (taskErrors) {
            for (const err of taskErrors) {
                notifyError({ title: err.field, message: err.message });
            }
        } else if (task) {
            notifySuccess({ message: 'Created one!' });
        }
    }

    return (
        <tr className="hover:bg-gray-100 border-b">
            <td className="px-6 py-3">
                <InputField
                    id="title"
                    label="Title"
                    name="title"
                    placeholder="Title"
                    form="form"
                    disabled={fetching}
                    hideLabel
                />
            </td>
            <td className="px-6 py-3">
                <Checkbox
                    name="completed"
                    id="completed"
                    form="form"
                    disabled={fetching}
                />
            </td>
            <td className="px-6 py-3">
                <form onSubmit={handleAddTask} id="form">
                    <button type="submit" disabled={fetching}>
                        <PlusCircleIcon className="transition duration-50 ease-out text-green-600 cursor-pointer hover:scale-110 hover:text-green-500" />
                    </button>
                </form>
            </td>
        </tr>
    );
}

function TasksTable({ tasks, tewdewId }: { tasks: Task[]; tewdewId: string }) {
    return (
        <table className="w-full table-fixed text-left m-2 bg-white">
            <thead className="w-full uppercase text-xs bg-gray-50">
                <tr>
                    <th scope="col" className="w-9/12 px-6 py-3">
                        Title
                    </th>
                    <th scope="col" className="w-2/12 px-6 py-3">
                        Completed
                    </th>
                    <th scope="col" className="w-1/12 px-6 py-3" />
                </tr>
            </thead>
            <tbody>
                {tasks.map((task) => (
                    <TaskRow key={task.id} task={task} />
                ))}
                <AddTaskRow tewdewId={tewdewId} />
            </tbody>
        </table>
    );
}

export default function ViewTewDew() {
    const params = useParams() as { id: string };
    const navigate = useNavigate();

    const { notifyError } = useAlert();
    const [{ data, error, fetching }] = useTewDewQuery({ variables: params });

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

    if (!data) {
        return <div>No tewdew found</div>;
    }

    const tewdew = data.retrieveTewDew;
    return (
        <div className="flex flex-col w-full -m-2">
            <div className="flex justify-between items-center m-2">
                <h1 className="flex font-sans text-3xl font-bold text-gray-800">
                    {tewdew.title}
                </h1>
                <div className="flex">
                    {tewdew.completed ? (
                        <CheckCircleSolidIcon className="text-green-600" />
                    ) : (
                        <CheckCircleOutlineIcon
                            className="text-gray-300"
                            disabled
                        />
                    )}
                </div>
            </div>
            <hr className="w-full m-2" />
            <div className="flex flex-col w-full h-full p-3 space-y-4 m-2">
                {tewdew.description && (
                    <div className="flex w-full">
                        <p className="text-gray-700">{tewdew.description}</p>
                    </div>
                )}
                <div className="w-full flex flex-col -m-2">
                    <h3 className="flex text-xl font-bold text-gray-800 m-2">
                        Tasks
                    </h3>
                    <hr className="w-full m-2" />
                    <TasksTable tasks={tewdew.tasks} tewdewId={tewdew.id} />
                </div>
            </div>
        </div>
    );
}
