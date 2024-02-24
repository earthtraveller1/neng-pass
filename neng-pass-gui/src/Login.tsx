import { createSignal, useContext } from "solid-js"
import { PageContext } from "./Index"
import { Page } from "./common"
import { invoke } from "@tauri-apps/api"
import TextInputField from "./components/PasswordInput"
import Button from "./components/Button"
import ErrorBox from "./components/ErrorBox"

export default function Login() {
    const pageContext = useContext(PageContext)
    if (pageContext == undefined) {
        throw new Error("pageContext can't be undefined!")
    }

    const [getMessageHeight, setMessageHeight] = createSignal("max-h-0")
    const [getEnteredPassword, setEnteredPassword] = createSignal("")

    return <div class="flex flex-col p-10 max-w-4xl m-10">
        <h1 class="text-4xl py-4 mb-8 select-none">Authorization</h1>

        <ErrorBox heightClass={getMessageHeight()}>
            Incorrect Master Key
        </ErrorBox>

        <TextInputField type="password" label="Master Key" onInput={event => {
            setEnteredPassword(event.target.value)
            setMessageHeight("max-h-0")
        }}/>

        <Button label="Authorize" onClick={async () => {
            const masterKey = getEnteredPassword()
            const masterKeyCorrect = await invoke<boolean>("is_master_key_correct", { pMasterKey: masterKey })

            if (masterKeyCorrect) {
                pageContext.setPage(Page.Passwords)
                await invoke("set_master_key", { pMasterKey: masterKey})
            } else {
                setMessageHeight("max-h-auto p-2 mb-6")
            }
        }} />
    </div>
}
