import { Component, OnInit } from '@angular/core';

@Component({
  templateUrl: './inbox.component.html'
})
export class InboxComponent implements OnInit {
  canSendMessage: boolean;

  constructor() {
  }

  ngOnInit() {
    // this.signalrService.messageReceived.subscribe((message: ChatMessage) => {

    // });

  }
}
