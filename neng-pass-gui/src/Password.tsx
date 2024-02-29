import { useContext } from "solid-js"
import { PasswordContext } from "./Index"

export default function Passwowrd() {
    const passwordContext = useContext(PasswordContext)
    if (passwordContext == undefined) {
        throw new Error("The password context must be provided!")
    }

    const password = passwordContext.getPassword()
    if (password == null) {
        throw new Error("There must be a password stored")
    }

    return <div class="flex flex-col">
        <h1 class="text-4xl py-4 mb-8 select-none">{password}</h1>
    </div>
}
