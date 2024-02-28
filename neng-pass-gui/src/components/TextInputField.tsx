import { JSX } from "solid-js/jsx-runtime";

interface InputProps {
    label: string,
    type: string,
    onInput: JSX.InputEventHandler<HTMLInputElement, InputEvent>,
    onEnterKey?: () => void,
}

export default function TextInputField(props: InputProps) {
    return <input 
        type={props.type}
        placeholder={props.label}
        class="p-4 bg-neutral-800 mb-4 outline-none focus:border-b-8 border-blue-400 duration-300 hover:bg-neutral-700 focus:bg-slate-800"
        onInput={props.onInput}
        onKeyDown={(event) => {
            if (event.key == "Enter") {
                if (props.onEnterKey != undefined) {
                    props.onEnterKey()
                }
            }
        }}
    />
}
