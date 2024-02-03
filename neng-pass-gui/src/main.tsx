import * as solidjsweb from "solid-js/web"

import Index from "./Index"

function main() {
    const appRoot = document.getElementById("app")
    if (appRoot != null) {
        solidjsweb.render(() => <Index />, appRoot)
    }
}

main()
