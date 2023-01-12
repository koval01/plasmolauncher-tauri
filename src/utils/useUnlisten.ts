const useUnlisten = () => new Unlisten()

class Unlisten {

    functions: Array<Function> = []

    constructor() {
        this.functions = []
    }

    push(fn: Function) {
        this.functions.push(fn)
    }

    async unlisten () {
        this.functions.forEach(async (fn) => await fn())
    }
}

export default useUnlisten