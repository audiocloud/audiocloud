export type Request<B> =
    | { method: 'get', headers?: Record<string, any>, path: string }
    | { method: 'post' | 'delete' | 'patch' | 'put', body?: B, headers?: Record<string, any>, path: string }

export interface Requester {
    request<B, T, E>(request: Request<B>): Promise<Result<T, E>>;
}

export type Result<T, E> =
    | { ok: T, error: null, is_ok: true, is_error: false }
    | { ok: null, error: E, is_ok: false, is_error: true }
