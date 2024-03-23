import Cdn from "./cdn.rs"

async function main() {
    const cdn = new Cdn("http://127.0.0.1:3434")

    // const latency = await cdn.getLatency();
    // console.log(`Response Time: ${latency}ns`)

    // const diagnostics = await cdn.getDiagnostics();
    // console.log(diagnostics)

    // const new_user = await cdn.createUser("admin", "admin");
    // console.log(new_user)

    const auth = await cdn.authenticate("admin", "admin");
    console.log(auth)

    cdn.getAsset("04ac3e7e-19ec-4f63-ba39-fa1a1e2af148", "manifest.json").then(manifest => {
        const json = manifest.toJson({});
        console.log(json)
    })
} main();