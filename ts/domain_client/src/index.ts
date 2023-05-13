import Axios, { AxiosInstance, AxiosRequestHeaders } from "axios";

import { LoginUserResponse } from "./types";

export { createWebSocketClient } from "./ws_socket";

export class DomainClientApi {
  private axios: AxiosInstance;
  private token: string | null = null;

  constructor(baseURL: string) {
    this.axios = Axios.create({
      baseURL,
      transformRequest: this.transformRequest,
    });
  }

  private transformRequest = (data: any, headers: AxiosRequestHeaders) => {
    if (!headers.hasAuthorization() && this.token) {
      headers.setAuthorization(`Bearer ${this.token}`);
    }
  };

  async userLogin(id: string, password: string): Promise<LoginUserResponse> {
    const { data } = await this.axios.post<LoginUserResponse>(
      `/users/login/${id}`,
      { password }
    );

    if (data.token) {
      this.token = data.token;
    }

    return data;
  }
}
