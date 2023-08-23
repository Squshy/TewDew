import type { FormEvent } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useAlert } from '@alertle/react';
//
import type { Task } from 'tewgql';
import { useTewDewQuery } from '../../urql/queries';
import {
    useCreateTaskMutation,
    useUpdateTaskMutation,
    useUpdateTewDewMutation,
} from '../../urql/mutations';
import Routes from '../../routes';
// Icons
import CheckCircleOutlineIcon from '../../components/icons/outline/CheckCircle';
import CheckCircleSolidIcon from '../../components/icons/solid/CheckCircle';
import PlusCircleIcon from '../../components/icons/solid/PlusCircle';
// Components
import Checkbox from '../../components/Checkbox';
import InputField from '../../components/InputField';
import CheckCircleIcon from '../../components/icons/outline/CheckCircle';
import { formEntries } from '../../utils/common';

function TaskRow({ task }: { task: Task }) {
    const { notifySuccess, notifyError } = useAlert();
    const [{ fetching }, updateTask] = useUpdateTaskMutation();

    async function handleUpdate(e: FormEvent<HTMLFormElement>) {
        // TODO: Do not allow updating if nothing has changed in the form
        e.preventDefault();

        const { completed, title } = formEntries<{
            completed: boolean;
            title: string;
        }>(e);

        const result = await updateTask({
            taskId: task.id,
            completed,
            titleOpt: title,
        });

        const updatedTask = result.data?.updateTask.task;
        const taskErrors = result.data?.updateTask.taskErrors;
        const errors = result.error?.graphQLErrors;

        if (errors) {
            for (const err of errors) {
                notifyError({ message: err.message });
            }
        } else if (taskErrors) {
            for (const err of taskErrors) {
                notifyError({ title: err.field, message: err.message });
            }
        } else if (updatedTask) {
            notifySuccess({ message: 'Updated task' });
        }
    }

    return (
        <tr className="hover:bg-gray-100 border-b cursor-pointer">
            <td className="px-6 py-3">
                <InputField
                    defaultValue={task.title}
                    id="title"
                    label="Title"
                    name="title"
                    form={`form-${task.id}`}
                    disabled={fetching}
                    hideLabel
                />
            </td>
            <td className="px-6 py-3">
                <Checkbox
                    defaultChecked={task.completed}
                    disabled={fetching}
                    name="completed"
                    id="completed"
                    form={`form-${task.id}`}
                />
            </td>
            <td className="px-6 py-3">
                <form onSubmit={handleUpdate} id={`form-${task.id}`}>
                    <button type="submit" disabled={fetching}>
                        <CheckCircleIcon className="transition duration-50 ease-out text-green-600 cursor-pointer hover:scale-110 hover:text-green-500" />
                    </button>
                </form>
            </td>
        </tr>
    );
}

function AddTaskRow({ tewdewId }: { tewdewId: string }) {
    const { notifyError, notifySuccess } = useAlert();
    const [{ fetching }, addTask] = useCreateTaskMutation();

    async function handleAddTask(e: FormEvent<HTMLFormElement>) {
        e.preventDefault();

        const { completed, title } = formEntries<{
            completed: boolean;
            title: string;
        }>(e);

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
            // eslint-disable-next-line
            // @ts-ignore
            (e.target.title as HTMLInputElement).value = '';
            // eslint-disable-next-line
            // @ts-ignore
            (e.target.completed as HTMLInputElement).checked = false;
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
                    disabled={fetching}
                    form={`form-${tewdewId}`}
                    hideLabel
                />
            </td>
            <td className="px-6 py-3">
                <Checkbox
                    defaultChecked={false}
                    name="completed"
                    id="completed"
                    disabled={fetching}
                    form={`form-${tewdewId}`}
                />
            </td>
            <td className="px-6 py-3">
                <form onSubmit={handleAddTask} id={`form-${tewdewId}`}>
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
                <AddTaskRow tewdewId={tewdewId} />
                {tasks.map((task) => (
                    <TaskRow key={task.id} task={task} />
                ))}
            </tbody>
        </table>
    );
}

export default function ViewTewDew() {
    const params = useParams() as { id: string };
    const navigate = useNavigate();
    const [, updateTewDew] = useUpdateTewDewMutation();

    const { notifyError, notifySuccess } = useAlert();
    const [{ data, error, fetching }] = useTewDewQuery({ variables: params });

    async function handleUpdateTewDew(e: FormEvent<HTMLFormElement>) {
        e.preventDefault();

        const id = data?.retrieveTewDew.id;

        if (!id) {
            return;
        }

        const completed = data.retrieveTewDew.completed;
        const result = await updateTewDew({ id, completed: !completed });

        const tewdew = result.data?.updateTewDew.tewDew;
        const tewdewErrors = result.data?.updateTewDew.tewDewErrors;
        const errors = result.error?.graphQLErrors;

        if (errors) {
            for (const err of errors) {
                notifyError({ message: err.message });
            }
            // eslint-disable-next-line
            // @ts-ignore
            (e.target.completed as HTMLInputElement).checked = false;
        } else if (tewdewErrors) {
            for (const err of tewdewErrors) {
                notifyError({ title: err.field, message: err.message });
            }
            // eslint-disable-next-line
            // @ts-ignore
            (e.target.completed as HTMLInputElement).checked = false;
        } else if (tewdew) {
            notifySuccess({ message: 'Updated' });
        }
    }

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
                    <form onSubmit={handleUpdateTewDew}>
                        {tewdew.completed ? (
                            <button type="submit">
                                <CheckCircleSolidIcon className="text-green-600" />
                            </button>
                        ) : (
                            <button type="submit">
                                <CheckCircleOutlineIcon className="text-gray-300" />
                            </button>
                        )}
                    </form>
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
