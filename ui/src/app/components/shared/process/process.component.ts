import { ChangeDetectionStrategy, ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { UntilDestroy } from '@ngneat/until-destroy';
import { ProcessService } from './services/process.service';

@UntilDestroy()
@Component({
  selector: 'process',
  templateUrl: './process.component.html',
  styleUrls: ['./process.component.scss']
})
export class ProcessComponent implements OnInit {
  isProcessing: boolean = false;

  constructor(
    private processService: ProcessService) {
  }

  ngOnInit() {    
  }
  
  process(){
    if(this.isProcessing){
      return;
    }

    this.isProcessing = true;

    this.processService.process().subscribe(data => {
      this.isProcessing = false;
    });
  }
}
