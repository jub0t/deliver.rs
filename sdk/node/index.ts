import Cdn from "./cdn.rs"

async function main() {
    const cdn = new Cdn("http://127.0.0.1:3434")

    const latency = await cdn.getLatency();
    console.log(`Response Time: ${latency}ns`)

    const diagnostics = await cdn.getDiagnostics();
    console.log(diagnostics)

    const new_user = await cdn.createUser("admin", "admin");
    console.log(new_user)

    const auth = await cdn.authenticate("admin", "admin");
    console.log(auth)
} main();