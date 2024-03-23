import Cdn from "./cdn.rs"

async function main() {
    const cdn = new Cdn("http://127.0.0.1:3434")
    await cdn.authenticate("admin", "admin");
    const start = new Date().getTime();

    cdn.getAsset("04ac3e7e-19ec-4f63-ba39-fa1a1e2af148", "manifest.json").then(manifest => {
        const json = manifest.toJson();
        console.log(json)
        console.log(`Elapsed: ${new Date().getTime() - start}ms`)
    })
} main();
