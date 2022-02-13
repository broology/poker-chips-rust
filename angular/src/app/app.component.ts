import { Component } from '@angular/core';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css'],
})
export class AppComponent {
  private _chips = 499999;

  get chips() {
    return this._chips;
  }

  increment(): void {
    this._chips += 1;
  }

  decrement(): void {
    this._chips -= 1;
  }

  update(event: any): void {
    this._chips = event.target.value;
  }
}
