import { Component, OnInit } from '@angular/core';
import { Router } from '../../node_modules/@angular/router';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html'
})
export class AppComponent implements OnInit {
  isAuthenticated: boolean;

  constructor(private router: Router) {
  }

  ngOnInit() {
  }
}
