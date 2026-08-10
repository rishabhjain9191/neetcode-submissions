class MinStack {
    constructor() {
        this.container = new Array();
        this.prefixContainer = new Array();
        this.prefixContainer.push(Infinity);
    }

    /**
     * @param {number} val
     * @return {void}
     */
    push(val) {
        this.container.push(val);
        this.prefixContainer.push(
            Math.min(
                this.prefixContainer.at(this.prefixContainer.length -1),
         val       
            )
        );
    }

    /**
     * @return {void}
     */
    pop() {
        this.container.pop();
        this.prefixContainer.pop();
    }

    /**
     * @return {number}
     */
    top() {
        return this.container.at(-1);
    }

    /**
     * @return {number}
     */
    getMin() {
        return this.prefixContainer.at(-1);
    }
}
