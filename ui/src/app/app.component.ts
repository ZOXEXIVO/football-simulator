import { Component } from '@angular/core';
import {NavigationEnd, Router} from "@angular/router";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'open-football';

  constructor(private router: Router) {
    router.events.subscribe(event => {
      if (event instanceof NavigationEnd) {
        window.scroll(0, 0);
        if (event.url) {
          this.setPageView(event.url);
        }
      }
    });
  }

  setPageView(url: string) {
    try {
      (<any>window).ga('set', 'page', url);
    }
    catch (ex) {
    }
  }
}
