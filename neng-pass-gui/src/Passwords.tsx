import { For, createResource } from "solid-js"
import { invoke } from "@tauri-apps/api"

export default function Passwords() {
    const [passwords, { mutate : mutatePasswords, refetch : refetchPasswords }] = createResource(async () => {
        return await invoke("get_password_list") as string[]
    })

    return (<>
        <div class="flex flex-row">
            <h1 class="text-4xl text-left ml-8 my-10 font-bold w-fit">Your Passwords</h1>
            <button 
                class={
                    "text-4xl text-right w-fit bg-green-600 px-4 rounded-xl " + 
                    "my-8 ml-auto mr-10 hover:bg-green-700 active:bg-green-800 duration-150"
                }
            >+</button>
        </div>

        <For each={passwords()!}>
            {(password) => {
                return <p class="my-1 pl-8 py-2 border-y-2 border-neutral-500">{password}</p>
            }}
        </For>
    </>)
}
