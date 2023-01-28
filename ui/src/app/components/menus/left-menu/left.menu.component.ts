import { ChangeDetectionStrategy, ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
import { Subject } from 'rxjs';
import { LeftMenuService, MenuSection } from './services/left.menu.service';

@UntilDestroy()
@Component({
  selector: 'left-menu',
  templateUrl: './left.menu.component.html',
  styleUrls: ['./left.menu.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class LeftMenuComponent implements OnInit {
  menuSections$: Subject<MenuSection[]> = new Subject<MenuSection[]>();

  constructor(
    private leftMenuService: LeftMenuService, 
    private changeDetectorRef: ChangeDetectorRef){      
  }

  ngOnInit () {
    this.leftMenuService.items$.pipe(untilDestroyed(this)).subscribe(menuSections => {
      this.menuSections$.next(menuSections);
      this.changeDetectorRef.markForCheck();
   });
}
}
