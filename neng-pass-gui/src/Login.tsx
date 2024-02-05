export default function Login() {
    return <div class="flex flex-col p-10 max-w-4xl m-10">
        <h1 class="text-4xl py-4 mb-8 select-none">Authorization</h1>

        <input 
            type="password" 
            placeholder="Master Key" 
            class="p-4 bg-neutral-800 outline-none focus:border-b-8 border-blue-400 duration-300"
        />

        <button 
            class="text-2xl p-2 rounded-md bg-green-700 text-neutral-100 mt-4 hover:bg-green-800 active:bg-green-900 duration-200"
        >Authorize</button>
    </div>
}
