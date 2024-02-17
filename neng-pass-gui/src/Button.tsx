import { JSX } from "solid-js/jsx-runtime"

interface ButtonProps {
    label: string,
    onClick: JSX.EventHandlerUnion<HTMLButtonElement, MouseEvent>
}

export default function Button(props: ButtonProps) {
    return <button 
        class="text-2xl p-2 rounded-md bg-green-700 text-neutral-100 mt-4 hover:bg-green-800 active:bg-green-900 duration-200"
        onClick={props.onClick}
    >{props.label}</button>
}
