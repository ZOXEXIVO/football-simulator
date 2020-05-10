import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'left-menu',
  templateUrl: './left.menu.component.html'
})
export class LeftMenuComponent implements OnInit {
  content: string;
  text: Number;

  constructor() {
  }

  ngOnInit() {
  }
}
