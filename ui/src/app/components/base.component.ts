import {OnInit} from "@angular/core";
import {ProcessService} from "./shared/process/services/process.service";
import {UntilDestroy, untilDestroyed} from "@ngneat/until-destroy";

@UntilDestroy()
export class BaseComponent implements OnInit {
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
