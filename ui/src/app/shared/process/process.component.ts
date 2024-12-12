import { Component, OnInit } from '@angular/core';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
import { DateDto, DateService } from 'src/app/shared/services/date.service';
import { ProcessService } from './services/process.service';

@UntilDestroy()
@Component({
    selector: 'process',
    templateUrl: './process.component.html',
    standalone: false,
    styleUrls: ['./process.component.scss']
})
export class ProcessComponent implements OnInit {
  date: DateDto | undefined;
  isProcessingInAction: boolean = false;

  constructor(
    private processService: ProcessService,
    private dateService: DateService) {
  }

  ngOnInit() {    
    this.updateDate();
  }
  
  process(){
    if(this.isProcessingInAction){
      return;
    }

    this.isProcessingInAction = true;

    this.processService.process().subscribe(data => {
      this.isProcessingInAction = false;
      this.updateDate();
    });
  }

  isProcessing(): boolean {
    return this.isProcessingInAction;
  }

  updateDate() {
    this.dateService.get_current().pipe(untilDestroyed(this)).subscribe(dateObj => {
      this.date = dateObj;
    });
  }
}
