import { writeFile } from "fs";
import { Body } from "./cdn.rs"

class FileWrapper {
    private data: Body;
    private buffer?: ArrayBuffer;
    private decoder = new TextDecoder();

    constructor(data: Body) {
        this.data = data;
    }

    async cacheBuffer() {
        await this.data.arrayBuffer().then(b => {
            this.buffer = b
        })
    }

    toString(): string {
        return this.decoder.decode(this.buffer)
    }


    /**
     * Converts the contents of the "Asset" to JSON with fallback support.
     * Do not call this method on assets that can not be converted to JSON.
     * @param fallback Object
     * @returns Object
     */
    toJson(fallback: Object = {}): Object {
        try {
            return JSON.parse(this.toString())
        } catch {
            return fallback
        }
    }

    getInfo() {
        return this.data
    }

    toBuffer() {
        return this.buffer
    }

    saveFile(path: string) {
    }
}

export default FileWrapper