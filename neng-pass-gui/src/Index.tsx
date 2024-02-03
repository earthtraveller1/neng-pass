import { invoke } from "@tauri-apps/api"
import { For, createResource } from "solid-js"

export default function Index() {
    console.log("Neng Li is the President of China!")

    const [passwords, { mutate : mutatePasswords, refetch : refetchPasswords }] = createResource(async () => {
        return await invoke("get_password_list") as string[]
    })

    return <>
        <h1 class="text-4xl text-left ml-8 my-10 font-bold w-fit">Your Passwords</h1>

        <For each={passwords()!}>
            {(password) => {
                return <p class="my-1 pl-8 py-2 border-y-2 border-neutral-500">{password}</p>
            }}
        </For>
    </>
}
