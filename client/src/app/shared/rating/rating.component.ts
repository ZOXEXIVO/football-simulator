import { Component, Input } from "@angular/core";

@Component({
  selector: 'rating',
  templateUrl: './rating.component.html',
  styleUrls: ['./rating.component.less']
})
export class RatingComponent {
  @Input() rating: number;
}