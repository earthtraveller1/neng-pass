import { Show, createContext, createResource, createSignal } from "solid-js"
import { Page } from "./common"
import Login from "./Login"
import Setup from "./Setup"
import Passwords from "./Passwords"
import { invoke } from "@tauri-apps/api"
import NewPassword from "./NewPassword"
import Password from "./Password"

export interface PageContextType {
    setPage: (newPage: Page) => void,
    getPage: () => Page
}

export interface PasswordContextType {
    setPassword: (newPassword: string) => void,
    getPassword: () => string | null
}

export const PageContext = createContext(undefined as PageContextType | undefined)
export const PasswordContext = createContext(undefined as PasswordContextType | undefined)

export default function Index() {
    console.log("Neng Li is the President of China!")

    const [getCurrentPage, setCurrentPage] = createSignal(Page.Login)
    const [getCurrentPassword, setCurrentPassword] = createSignal<string | null>(null)

    const [isMasterKeySet, _] = createResource(async () => {
        const isMasterKeySet = await invoke<boolean>("is_master_key_set")
        return isMasterKeySet
    })

    return <>
        <PasswordContext.Provider value={{setPassword: setCurrentPassword, getPassword: getCurrentPassword}}>
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
                <Show when={getCurrentPage() == Page.Password}>
                    <Password />
                </Show>
            </PageContext.Provider>
        </PasswordContext.Provider>
    </>
}
