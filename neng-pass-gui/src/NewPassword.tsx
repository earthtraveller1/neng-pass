import { createSignal } from "solid-js";
import TextInputField from "./components/PasswordInput";
import Button from "./components/Button";

export default function NewPassword() {
    const [getNewPasswordName, setNewPasswordName] = createSignal("")

    return <div>
        <h1 class="text-4xl py-4 mb-8 select-none">Create a new Password</h1>

        <TextInputField type="text" label="Name" onInput={(event) => {
            setNewPasswordName(event.target.value)
        }} />

        <Button label="create" onClick={() => {
            
        }} />
    </div>
}
