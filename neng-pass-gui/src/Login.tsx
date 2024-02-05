import { createSignal, useContext } from "solid-js"
import { PageContext } from "./Index"
import { Page } from "./common"
import { invoke } from "@tauri-apps/api"

export default function Login() {
    const pageContext = useContext(PageContext)
    if (pageContext == undefined) {
        throw new Error("pageContext can't be undefined!")
    }

    const [getMessageHeight, setMessageHeight] = createSignal("max-h-0")
    const [getEnteredPassword, setEnteredPassword] = createSignal("")

    return <div class="flex flex-col p-10 max-w-4xl m-10">
        <h1 class="text-4xl py-4 mb-8 select-none">Authorization</h1>

        <div class={`text-xl bg-red-800 ${getMessageHeight()} duration-500 rounded-xl overflow-hidden text-center select-none`}>
            Incorrect Master Key
        </div>

        <input 
            type="password" 
            placeholder="Master Key" 
            class="p-4 bg-neutral-800 outline-none focus:border-b-8 border-blue-400 duration-300 hover:bg-neutral-700 focus:bg-slate-800"
            onInput={(event) => {
                setEnteredPassword(event.target.value)
            }}
        />

        <button 
            class="text-2xl p-2 rounded-md bg-green-700 text-neutral-100 mt-4 hover:bg-green-800 active:bg-green-900 duration-200"
            onClick={async () => {
                const master_key_correct = await invoke<boolean>("is_master_key_correct", { pMasterKey: getEnteredPassword() })

                if (master_key_correct) {
                    pageContext.setPage(Page.Passwords)
                } else {
                    setMessageHeight("max-h-auto p-2 mb-6")
                }
            }}
        >Authorize</button>
    </div>
}
