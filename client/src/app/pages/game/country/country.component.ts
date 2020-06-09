import { Component, OnInit, OnDestroy } from '@angular/core';
import { HeaderService, HeaderModel } from '../services/header.service';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';

@Component({
  templateUrl: './country.component.html'
})
export class CountryComponent  implements OnInit, OnDestroy {  
  destroy$ = new Subject<void>();

  headerTitle: String = "";
  headerSubTitle: String = "";

  constructor(public headerService: HeaderService) {
  }

  ngOnInit() {
    this.headerService.get()
      .pipe(takeUntil(this.destroy$))
      .subscribe((header: HeaderModel) => {
        setTimeout(() => {
          this.headerTitle = header.title;
          this.headerSubTitle = header.subTitle;
        });
      });
  }

  ngOnDestroy(): void {
    this.destroy$.next();
    this.destroy$.complete();
  }  
}
