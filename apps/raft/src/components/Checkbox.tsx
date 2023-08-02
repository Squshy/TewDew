import type { HTMLProps } from 'react';

export default function Checkbox({
    label,
    ...props
}: HTMLProps<HTMLInputElement>) {
    return (
        <div className="flex justify-center items-center">
            <input
                type="checkbox"
                className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 focus:ring-2 cursor-pointer"
                {...props}
            />
            {label && (
                <label
                    htmlFor={props.id}
                    className="ml-2 text-sm font-medium text-gray-900"
                >
                    {label}
                </label>
            )}
        </div>
    );
}
