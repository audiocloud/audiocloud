import Axios, { AxiosInstance, AxiosRequestConfig } from "axios";

export type Request<B> =
  | { method: "get"; headers?: Record<string, any>; path: string }
  | {
      method: "post" | "delete" | "patch" | "put";
      body?: B;
      headers?: Record<string, any>;
      path: string;
    };

export interface Requester {
  request<B, T, E>(request: Request<B>): Promise<Result<T, E>>;
}

export type Result<T, E> =
  | { ok: T; error: null; is_ok: true; is_error: false }
  | { ok: null; error: E; is_ok: false; is_error: true };

export class AxiosRequester implements Requester {
  axios: AxiosInstance;

  constructor(defaults?: AxiosRequestConfig) {
    this.axios = Axios.create(defaults);
  }

  async request<B, T, E>(request: Request<B>): Promise<Result<T, E>> {
    const axios_request: AxiosRequestConfig = {
      url: request.path,
      headers: request.headers || {},
      method: request.method,
    };

    if ("body" in request) {
      axios_request.data = request.body;
      axios_request.headers!["content-type"] = "application/json";
    }

    const res = await this.axios
      .request(axios_request)
      .catch(({ response }) => response);
    if (res.status >= 200 && res.status < 300) {
      return Promise.resolve({
        ok: res.data,
        is_error: false,
        is_ok: true,
      } as Result<T, E>);
    } else {
      return Promise.reject({
        error: res.data,
        is_ok: false,
        is_error: true,
      } as Result<T, E>);
    }
  }
}
