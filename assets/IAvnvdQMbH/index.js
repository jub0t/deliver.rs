class KV {
    data = {};
    constructor() { }

    set(k, v) {
        this.data[k] = v;
    }

    get(k) {
        return this.data[k]
    }
}

const build = new KV()
build.set("john", {
    Username: "John Doe",
    Age: 18
})

const user = build.get("john")
console.log(user)