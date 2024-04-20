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
    const [getErrorMessage, setErrorMessage] = createSignal("")

    if (pageContext == undefined) {
        throw new Error("Page context was not provided")
    }

    async function saveThePassword() {
        try {
            if (getNewPasswordName() == "") {
                throw "You cannot create a password without a name"
            }

            if (getNewPassword() == "") {
                throw "You did not enter a password"
            }

            await invoke("save_password", { pName: getNewPasswordName(), pPassword: getNewPassword() })
            if (pageContext != undefined) {
                pageContext.setPage(Page.Passwords)
            }
        } catch (error) {
            setErrorMessageHeight("max-h-auto p-2 mb-6")
            setErrorMessage(error as string)
        }
    }

    return <div class="flex flex-col max-w-4xl m-10">
        <h1 class="text-4xl py-4 mb-8 select-none">Create a new Password</h1>

        <ErrorBox heightClass={getErrorMessageHeight()}>
            {getErrorMessage()}
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
