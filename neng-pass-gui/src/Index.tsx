import { Show, createContext, createResource, createSignal } from "solid-js"
import { Page } from "./common"
import Login from "./Login"
import Setup from "./Setup"
import Passwords from "./Passwords"
import { invoke } from "@tauri-apps/api"
import NewPassword from "./NewPassword"

export interface PageContextType {
    setPage: (newPage: Page) => void,
    getPage: () => Page
}

export const PageContext = createContext(undefined as PageContextType | undefined)

export default function Index() {
    console.log("Neng Li is the President of China!")

    const [getCurrentPage, setCurrentPage] = createSignal(Page.Login)

    const [isMasterKeySet, _] = createResource(async () => {
        const isMasterKeySet = await invoke<boolean>("is_master_key_set")
        return isMasterKeySet
    })

    return <>
        <PageContext.Provider value={{setPage: setCurrentPage, getPage: getCurrentPage }}>
            <Show when={getCurrentPage() == Page.Login}>
                <Show when={isMasterKeySet()} fallback={<Setup />}> 
                    <Login />
                </Show>
            </Show>
            <Show when={getCurrentPage() == Page.Passwords}>
                <Passwords />
            </Show>
            <Show when={getCurrentPage() == Page.NewPassword}>
                <NewPassword />
            </Show>
        </PageContext.Provider>
    </>
}
