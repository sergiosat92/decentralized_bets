import { DatePipe, NgClass, NgFor } from '@angular/common';
import { Component, inject } from '@angular/core';
import { LoggerService } from '../logger.service';

@Component({
  selector: 'app-logger',
  imports: [NgFor, NgClass, DatePipe],
  templateUrl: './logger.component.html',
  styleUrl: './logger.component.css'
})
export class LoggerComponent {
  logsService = inject(LoggerService);
  logs = this.logsService.logs;
}
