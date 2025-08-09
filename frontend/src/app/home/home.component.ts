import { Component, inject } from '@angular/core';
import { NavbarComponent } from '../navbar/navbar.component';
import { ApiService } from '../api.service';

@Component({
  selector: 'app-home',
  imports: [NavbarComponent],
  templateUrl: './home.component.html',
  styleUrl: './home.component.css',
})
export class HomeComponent {
  api = inject(ApiService);
  constructor() {
    this.api.get_leagues().subscribe((data) => {
      console.log("leagues", data);
    });
  }
}
