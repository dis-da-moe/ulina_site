System.register([], function (exports_1, context_1) {
    "use strict";
    var result;
    var __moduleName = context_1 && context_1.id;
    function Ok(value) {
        return result(true, value);
    }
    exports_1("Ok", Ok);
    function OkVoid() {
        return result(true);
    }
    exports_1("OkVoid", OkVoid);
    function ErrVoid(error) {
        return result(false, undefined, error);
    }
    exports_1("ErrVoid", ErrVoid);
    function Err(error) {
        return result(false, undefined, error);
    }
    exports_1("Err", Err);
    return {
        setters: [],
        execute: function () {
            result = (ok, value, message) => {
                const result = {
                    ok,
                    err: !ok,
                    value: value,
                    message: message,
                    mapOk(value) {
                        if (this.ok) {
                            return Ok(value);
                        }
                        else {
                            return Err(this.message);
                        }
                    },
                    chain(action) {
                        if (this.ok) {
                            return action(this.value);
                        }
                        else {
                            return Err(this.message);
                        }
                    },
                    asyncChain(action) {
                        if (this.ok) {
                            return action(this.value);
                        }
                        else {
                            return Promise.resolve(Err(this.message));
                        }
                    }
                };
                return result;
            };
        }
    };
});
//# sourceMappingURL=result.js.map