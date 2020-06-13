import { NgModule } from '@angular/core';
import { SpinnerComponent } from './spinners/default/spinner.component';
import { LineSpinnerComponent } from './spinners/line/line.spinner.component';
import { RatingComponent } from './rating/rating.component';
import { CommonModule } from '@angular/common';

@NgModule({
  declarations: [
    SpinnerComponent,
    RatingComponent,
    LineSpinnerComponent
  ],
  imports: [
    CommonModule
  ],
  exports: [
    SpinnerComponent,
    RatingComponent,
    LineSpinnerComponent
  ]
})
export class SharedModule { }
