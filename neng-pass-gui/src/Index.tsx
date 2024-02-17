import { Show, createContext, createSignal } from "solid-js"
import { Page } from "./common"
import Login from "./Login"
import Passwords from "./Passwords"

export interface PageContextType {
    setPage: (newPage: Page) => void,
    getPage: () => Page
}

export const PageContext = createContext(undefined as PageContextType | undefined)

export default function Index() {
    console.log("Neng Li is the President of China!")

    const [getCurrentPage, setCurrentPage] = createSignal(Page.Login)

    return <>
        <Show when={getCurrentPage() == Page.Login}>
            <PageContext.Provider value={{setPage: setCurrentPage, getPage: getCurrentPage }}>
                <Login />
            </PageContext.Provider>
        </Show>
        <Show when={getCurrentPage() == Page.Passwords}>
            <Passwords />
        </Show>
    </>
}
