import { createSignal, useContext } from "solid-js";
import TextInputField from "./components/TextInputField";
import Button from "./components/Button";
import { invoke } from "@tauri-apps/api";
import { PageContext } from "./Index";
import { Page } from "./common";

export default function NewPassword() {
    const [getNewPasswordName, setNewPasswordName] = createSignal("")
    const pageContext = useContext(PageContext)

    if (pageContext == undefined) {
        throw new Error("Page context was not provided")
    }

    return <div class="flex flex-col max-w-4xl m-10">
        <h1 class="text-4xl py-4 mb-8 select-none">Create a new Password</h1>

        <TextInputField type="text" label="Name" onInput={(event) => {
            setNewPasswordName(event.target.value)
        }} />

        <Button label="Create" onClick={async () => {
            await invoke("create_password", { pName: getNewPasswordName() })
            pageContext.setPage(Page.Passwords)
        }} />
    </div>
}
