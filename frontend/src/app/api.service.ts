import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';

const API_TOKEN: string = 'g7E3SZYM5wsQFc3W9yvkIz1KTK8bdCLsNo9ZrNxt9Bh0cv3uMJ9sg2BA6eRQ';
const API_BASE_URL: string = 'https://cricket.sportmonks.com/api/v2.0';
const API_AUTH_HEADER: string = `?api_token=${API_TOKEN}`;

@Injectable({
  providedIn: 'root',
})
export class ApiService {
  http = inject(HttpClient);
  headers = {
    'Content-Type': 'application/json',
    Authorization: `Bearer ${API_TOKEN}`,
    'Access-Control-Allow-Origin': '*',
  };
  constructor() {}

  get_leagues() {
    return this.http.get(`${API_BASE_URL}/leagues${API_AUTH_HEADER}&include=england`, {
      headers: this.headers,
    });
  }
}
