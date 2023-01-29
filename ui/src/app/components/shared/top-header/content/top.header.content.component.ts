import { TopHeaderService } from '../services/top.header.service';
import {
  AfterViewInit,
  Directive,
  ElementRef,
  Renderer2
} from '@angular/core';

@Directive({
  selector: '[topHeader]',
})
export class TopHeaderContentComponent implements AfterViewInit {
  constructor(private topHeaderService: TopHeaderService,
    private elem: ElementRef, private renderer: Renderer2) {
  }

  ngAfterViewInit(): void {
  
    if(this.elem){
      debugger;
      //this.topHeaderService.setContent(this.topHeaderContent.nativeElement.textContent);
    }
  }
}
