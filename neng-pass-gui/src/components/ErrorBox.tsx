import { JSX } from "solid-js/jsx-runtime"

interface ErrorBoxProps {
    heightClass: string,
    children: JSX.Element
}

export default function ErrorBox(props: ErrorBoxProps) {
    return <div class={`text-xl bg-red-800 ${props.heightClass} duration-500 rounded-xl overflow-hidden text-center select-none`}>
        {props.children}
    </div>
}
