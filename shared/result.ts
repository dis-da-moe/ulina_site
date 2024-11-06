export interface Result<ValueType> {
    readonly ok: boolean
    readonly err: boolean
    readonly value?: ValueType
    readonly message?: string
    readonly mapOk: (<ReturnType> (value: ReturnType) => Result<ReturnType>)
    chain <ReturnType> (this: Result<ValueType>, action: (value: ValueType) => Result<ReturnType>) : Result<ReturnType>
    asyncChain <ReturnType> (this: Result<ValueType>, action: (value: ValueType) => Promise<Result<ReturnType>>) : Promise<Result<ReturnType>>
}

const result = <ValueType>(ok: boolean, value?: ValueType, message?: string): Result<ValueType> => {
    const result =  {
        ok,
        err: !ok,
        value: value,
        message: message,
        mapOk<ValueType, ReturnType>(value: ReturnType): Result<ReturnType> {
            if (this.ok){
                return Ok(value);
            }
            else{
                return Err(this.message);
            }
        },
        chain<ReturnType>(this: Result<ValueType>, action: (value: ValueType) => Result<ReturnType>): Result<ReturnType> {
            if (this.ok){
                return action(this.value);
            }
            else{
                return Err(this.message);
            }
        },
        asyncChain<ReturnType>(this: Result<ValueType>, action: (value: ValueType) => Promise<Result<ReturnType>>): Promise<Result<ReturnType>> {
            if (this.ok){
                return action(this.value);
            }
            else{
                return Promise.resolve(Err(this.message));
            }
        }
    };
    return result;
}

export function Ok<ValueType>(value: ValueType): Result<ValueType> {
    return result<ValueType>(true, value);
}

export function OkVoid(): Result<void>{
    return result<void>(true);
}

export function ErrVoid(error: string): Result<void>{
    return result<void>(false, undefined, error);
}

export function Err<ValueType>(error: string): Result<ValueType> {
    return result<ValueType>(false, undefined, error);
}