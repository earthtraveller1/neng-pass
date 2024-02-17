import PasswordInput from "./PasswordInput"
import Button from "./Button"

export default function Setup() {
    return <div class="flex flex-col p-2 max-w-4xl m-10">
        <h1 class="text-4xl pb-4 select-none">Setup</h1>
        <p class="mb-8">Please set your master key to use.</p>

        <PasswordInput label="New Master Key" onInput={() => {}} />
        <PasswordInput label="Confirm Master Key" onInput={() => {}} />

        <Button label="Continue" onClick={undefined} />
    </div>
}
