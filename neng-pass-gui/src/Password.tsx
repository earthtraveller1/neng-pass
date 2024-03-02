import { createSignal, useContext } from "solid-js"
import { PageContext, PasswordContext } from "./Index"
import Button, { ButtonStyle } from "./components/Button"
import { Page } from "./common"
import { invoke } from "@tauri-apps/api"

export default function Password() {
    const passwordContext = useContext(PasswordContext)
    if (passwordContext == undefined) {
        throw new Error("The password context must be provided!")
    }

    const pageContext = useContext(PageContext)
    if (pageContext == undefined) {
        throw new Error("The page context was not set! What the actual fuck?")
    }

    const passwordName = passwordContext.getPassword()
    if (passwordName == null) {
        throw new Error("There must be a password stored")
    }

    const [getFirstButtonLabel, setFirstButtonLabel] = createSignal("Copy to Clipboard")

    return <div class="flex flex-col p-4">
        <h1 class="text-4xl py-4 mb-8 select-none text-center">{passwordName}</h1>
        <Button label={getFirstButtonLabel()} style={ButtonStyle.Green} onClick={async () => {
            try {
                const password = await invoke<string>("get_password", {pName: passwordName})
                await navigator.clipboard.writeText(password)
                setFirstButtonLabel("Copied to Clipboard!")
            } catch (error) {
                setFirstButtonLabel("Sorry, something went wrong")
            }
        }}/>
        <Button label="Delete" style={ButtonStyle.Red} onClick={async () => {
            await invoke("delete_password", {pName: passwordName})
            pageContext.setPage(Page.Passwords)
        }}/>
        <Button label="Back" style={ButtonStyle.Gray} onClick={() => {
            pageContext.setPage(Page.Passwords)
        }}/>
    </div>
}
