import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function Setup() {
    const [value, setValue] = useState('')
    const [err, setErr] = useState('')
    async function register() {
        setErr('')
        try {
            await invoke('register', {value})
        } catch (e: any) {
            console.error(e)
            setErr(e)
        }
    }
    
    return (
        <div>
            <h1>Activation</h1>
            <input type="text" value={value} onChange={e => setValue(e.target.value)} placeholder="Paste your license here..." />
            <button onClick={register}>Activate</button>
            {err && <p style={{color: 'red'}}>{err}</p>}
        </div>
    )
}