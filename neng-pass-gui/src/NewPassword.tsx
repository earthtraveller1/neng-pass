import { createSignal, useContext } from "solid-js";
import TextInputField from "./components/TextInputField";
import Button, { ButtonStyle } from "./components/Button";
import { invoke } from "@tauri-apps/api";
import { PageContext } from "./Index";
import { Page } from "./common";
import ErrorBox from "./components/ErrorBox";

export default function NewPassword() {
    const [getNewPasswordName, setNewPasswordName] = createSignal("")
    const [getNewPassword, setNewPassword] = createSignal("")

    const pageContext = useContext(PageContext)
    const [getErrorMessageHeight, setErrorMessageHeight] = createSignal("max-h-0")

    if (pageContext == undefined) {
        throw new Error("Page context was not provided")
    }

    async function saveThePassword() {
        try {
            await invoke("save_password", { pName: getNewPasswordName(), pPassword: getNewPassword() })
            if (pageContext != undefined) {
                pageContext.setPage(Page.Passwords)
            }
        } catch (error) {
            setErrorMessageHeight("max-h-auto p-2 mb-6")
        }
    }

    return <div class="flex flex-col max-w-4xl m-10">
        <h1 class="text-4xl py-4 mb-8 select-none">Create a new Password</h1>

        <ErrorBox heightClass={getErrorMessageHeight()}>
            Password is too long (max length is 16 characters)
        </ErrorBox>

        <TextInputField type="text" label="Name" onInput={(event) => {
            setNewPasswordName(event.target.value)
        }} onEnterKey={async () => {
            await saveThePassword()
        }}/>

        <TextInputField type="password" label="Password" onInput={(event) => {
            setNewPassword(event.target.value)
        }} onEnterKey={async () => {
            await saveThePassword()
        }}/>

        <Button label="Create" style={ButtonStyle.Green} onClick={async () => {
            await saveThePassword()
        }} />
    </div>
}
