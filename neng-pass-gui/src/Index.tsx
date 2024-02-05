import { invoke } from "@tauri-apps/api"
import { For, Show, createContext, createResource, createSignal } from "solid-js"
import { Page } from "./common"
import Login from "./Login"

export interface PageContextType {
    setPage: (newPage: Page) => void,
    getPage: () => Page
}

export const PageContext = createContext(undefined as PageContextType | undefined)

export default function Index() {
    console.log("Neng Li is the President of China!")

    const [passwords, { mutate : mutatePasswords, refetch : refetchPasswords }] = createResource(async () => {
        return await invoke("get_password_list") as string[]
    })

    const [getCurrentPage, setCurrentPage] = createSignal(Page.Login)

    return <>
        <Show when={getCurrentPage() == Page.Login}>
            <PageContext.Provider value={{setPage: setCurrentPage, getPage: getCurrentPage }}>
                <Login />
            </PageContext.Provider>
        </Show>
        <Show when={getCurrentPage() == Page.Passwords}>
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
        </Show>
    </>
}
