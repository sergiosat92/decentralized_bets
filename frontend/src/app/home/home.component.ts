import { Component, inject } from '@angular/core';
import { NavbarComponent } from '../navbar/navbar.component';
import { ApiService, League, LeaguesResponse } from '../api.service';
import { LoggerComponent } from '../logger/logger.component';
import { NgFor } from '@angular/common';
import { LoggerService } from '../logger.service';

@Component({
  selector: 'app-home',
  imports: [NavbarComponent, LoggerComponent, NgFor],
  templateUrl: './home.component.html',
  styleUrl: './home.component.css',
})
export class HomeComponent {
  api = inject(ApiService);
  logger = inject(LoggerService);
  leagues: League[] = [];

  constructor() {
    this.api.get_leagues().subscribe((data: LeaguesResponse) => {
      this.leagues = data.leagues;
      this.logger.printLog('success', 'Leagues fetched successfully ✅');
    });
  }
}
