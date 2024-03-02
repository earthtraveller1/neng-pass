import { JSX } from "solid-js/jsx-runtime"

export enum ButtonStyle {
    Red, Green, Gray
}

interface ButtonProps {
    label: string,
    onClick: JSX.EventHandlerUnion<HTMLButtonElement, MouseEvent> | undefined
    style?: ButtonStyle
}

export default function Button(props: ButtonProps) {
    let normal_color = "bg-zinc-700"
    let hover_color = "hover:bg-zinc-800"
    let active_color = "active:bg-zinc-900"

    switch (props.style) {
        case ButtonStyle.Red:
            normal_color = "bg-red-700"
            hover_color = "hover:bg-red-800"
            active_color = "active:bg-red-900"
            break
        case ButtonStyle.Gray:
            normal_color = "bg-zinc-700"
            hover_color = "hover:bg-zinc-800"
            active_color = "active:bg-zinc-900"
            break
        case ButtonStyle.Green:
            normal_color = "bg-green-700"
            hover_color = "hover:bg-green-800"
            active_color = "active:bg-green-900"
            break
    }

    return <button 
        class={`text-2xl p-2 rounded-md ${normal_color} text-neutral-100 mt-4 ${hover_color} ${active_color} duration-200`}
        onClick={props.onClick}
    >{props.label}</button>
}
