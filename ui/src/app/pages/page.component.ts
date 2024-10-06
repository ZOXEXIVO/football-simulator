import {Component, OnInit} from "@angular/core";
import {UntilDestroy, untilDestroyed} from "@ngneat/until-destroy";
import {ProcessService} from "../shared/process/services/process.service";

@Component({
  selector: 'base-component',
  templateUrl: './page.component.html'
})
@UntilDestroy()
export class PageComponent implements OnInit {
  constructor(private progressService: ProcessService) {
    progressService.OnProcess.pipe(untilDestroyed(this)).subscribe((data: any) => {
      this.onDataRefresh()
    });
  }

  ngOnInit(): void {
    this.onDataRefresh();
  }

  onDataRefresh() {

  }
}
