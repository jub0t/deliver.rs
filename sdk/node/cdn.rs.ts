import { request } from 'undici'
import { JsonHeader } from "./misc"

export default class RustNetwork {
    private url: String;

    constructor(url: String) {
        this.url = url;
    }


    // Misc
    async getDiagnostics() {
        const {
            body
        } = await request(`${this.url}/diagnostics`)
        const data = await body.json()
        return data;
    }

    // Authentication
    async authenticate(username: String, password: String) {
        const {
            body
        } = await request(`${this.url}/authenticate`, {
            method: "POST",
            headers: {
                ...JsonHeader
            },
            body: JSON.stringify({
                username,
                password
            })
        })

        const data = await body.json()
        return data;
    }
}