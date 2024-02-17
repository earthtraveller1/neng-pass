import { JSX } from "solid-js/jsx-runtime";

interface InputProps {
    label: string,
    onInput: JSX.InputEventHandler<HTMLInputElement, InputEvent>,
}

export default function PasswordInput(props: InputProps) {
    return <input 
        type="password" 
        placeholder={props.label}
        class="p-4 bg-neutral-800 mb-4 outline-none focus:border-b-8 border-blue-400 duration-300 hover:bg-neutral-700 focus:bg-slate-800"
        onInput={props.onInput}
    />
}
