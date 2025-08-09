import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';

const API_BASE_URL: string = 'http://localhost:8000';

export interface League {
  resource: string;
  id: number;
  season_id: number;
  country_id: number;
  name: string;
  code: string;
  image_path: string;
  type: string;
  updated_at: string; // ISO date string
}

export interface LeaguesResponse {
  leagues: League[];
}

@Injectable({
  providedIn: 'root',
})
export class ApiService {
  http = inject(HttpClient);
  constructor() {}

  get_leagues(){
    return this.http.get<LeaguesResponse>(`${API_BASE_URL}/get_leagues`);
  }
}
