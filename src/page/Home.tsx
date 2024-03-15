import { invoke } from "@tauri-apps/api/tauri"
import { useEffect, useState } from "react"

interface License {
    value: string
    expire: string
    username: string
    hwid: string
}

export default function HomePage() {
    const [license, setLicense] = useState<License>()
    useEffect(() => {
        async function getLicense() {
            const res: License = await invoke('get_license')
            setLicense(res as License)
        }
        getLicense()
    })

    async function revoke() {
        await invoke('revoke')
    }
    return (
        <div>
            <h1>Welcome  {license?.username}!</h1>
            <p>Your license is <code>{license?.value}</code></p>
            <p>hwid is <code>{license?.hwid}</code></p>
            <p>Expire is <code>{license?.expire}</code></p>
            <button onClick={revoke}>Revoke license</button>
        </div>
    )
}