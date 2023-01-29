import { TopHeaderService } from '../services/top.header.service';
import {
  AfterViewInit,
  Directive,
  ElementRef,
  Renderer2
} from '@angular/core';

@Directive({
  selector: '[topHeader]'
})
export class TopHeaderContentComponent implements AfterViewInit {
  constructor(private topHeaderService: TopHeaderService,
    private elem: ElementRef) {
  }

  ngAfterViewInit(): void {
    this.topHeaderService.setContent(this.elem.nativeElement.innerHTML);
    this.elem.nativeElement.innerHTML = '';
  }
}
