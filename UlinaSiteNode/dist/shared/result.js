"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Err = exports.ErrVoid = exports.OkVoid = exports.Ok = void 0;
const result = (ok, value, message) => {
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
function Ok(value) {
    return result(true, value);
}
exports.Ok = Ok;
function OkVoid() {
    return result(true);
}
exports.OkVoid = OkVoid;
function ErrVoid(error) {
    return result(false, undefined, error);
}
exports.ErrVoid = ErrVoid;
function Err(error) {
    return result(false, undefined, error);
}
exports.Err = Err;
//# sourceMappingURL=result.js.map