import React, { forwardRef, type ForwardedRef } from 'react';

type InputFieldProps = React.HTMLProps<HTMLInputElement> & {
    label: string;
    id: string; // id is not required on base element, make it required
    error?: string | undefined;
};

const InputField = forwardRef(function (
    { label, id, type, error, ...props }: InputFieldProps,
    ref: ForwardedRef<HTMLInputElement>
) {
    return (
        <div>
            <label htmlFor={id} className="sr-only">
                {label}
            </label>
            <input
                type={type}
                id={id}
                ref={ref}
                className={`relative block w-full rounded-md border-0 p-2.5
                    text-gray-900 ring-1 ring-inset ring-${
                        error ? 'red' : 'gray'
                    }-300 placeholder:text-gray-400 \
                    focus:z-10 focus:ring-2 focus:ring-inset focus:ring-${
                        error ? 'red' : 'indigo'
                    }-600 focus:outline-none \
                    sm:text-sm sm:leading-6 drop-shadow`}
                {...props}
            />
            <span className="text-red-500 text-sm">{error}</span>
        </div>
    );
});

export default InputField;
