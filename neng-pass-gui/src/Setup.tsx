import PasswordInput from "./components/PasswordInput"
import Button from "./components/Button"
import { createSignal, useContext } from "solid-js"
import ErrorBox from "./components/ErrorBox"
import { invoke } from "@tauri-apps/api"
import { PageContext } from "./Index"
import { Page } from "./common"

export default function Setup() {
    const [getNewMasterKey, setNewMasterKey] = createSignal("")
    const [getConfirmMasterKey, setConfirmMasterKey] = createSignal("")
    const [getErrorBoxHeightClass, setErrorBoxHeightClass] = createSignal("max-h-0")

    const pageContext = useContext(PageContext)

    return <div class="flex flex-col p-2 max-w-4xl m-10">
        <h1 class="text-4xl pb-4 select-none">Setup</h1>
        <p class="mb-8">Please set your master key to use.</p>

        <ErrorBox heightClass={getErrorBoxHeightClass()}>The keys that you entered do not match</ErrorBox>

        <PasswordInput label="New Master Key" onInput={(event) => {
            setErrorBoxHeightClass("max-h-0")
            setNewMasterKey(event.target.value.trim())
        }} />
        <PasswordInput label="Confirm Master Key" onInput={(event) => {
            setErrorBoxHeightClass("max-h-0")
            setConfirmMasterKey(event.target.value.trim())
        }} />

        <Button label="Continue" onClick={async () => {
            if (getNewMasterKey() != getConfirmMasterKey()) {
                setErrorBoxHeightClass("max-h-auto p-2 mb-6")
            } else {
                await invoke("set_new_master_key", { pNewMasterKey: getNewMasterKey() })
                await invoke("set_master_key", { pMasterKey: getNewMasterKey() })
                if (pageContext != null) {
                    pageContext.setPage(Page.Passwords)
                }
            }
        }} />
    </div>
}
