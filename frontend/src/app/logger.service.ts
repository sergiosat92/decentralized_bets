import { Injectable, signal } from '@angular/core';

interface Log {
  topic: string;
  message: string;
  timestamp: Date;
}

@Injectable({
  providedIn: 'root',
})
export class LoggerService {
  logs = signal<Log[]>([]);
  constructor() {}

  printLog(topic: string, message: string) {
    this.logs.update((logs) => [
      ...logs,
      { topic, message, timestamp: new Date() },
    ]);
  }
}
