import { ChangeDetectionStrategy, ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
import {ContentDto, TopHeaderService} from './services/top.header.service';
import {RouterLink} from "@angular/router";

@UntilDestroy()
@Component({
    selector: 'top-header',
    templateUrl: './top.header.component.html',
    styleUrls: ['./top.header.component.scss'],
    standalone: false,
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class TopHeaderComponent implements OnInit {
  content: ContentDto | null = null;

  constructor(
    private topHeaderService: TopHeaderService,
    private changeDetectorRef: ChangeDetectorRef) {
  }

  ngOnInit() {
    this.topHeaderService.content$.pipe(untilDestroyed(this)).subscribe(content => {
      this.content = content;
      this.changeDetectorRef.markForCheck();
    });
  }
}
