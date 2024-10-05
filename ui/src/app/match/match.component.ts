import {UntilDestroy} from "@ngneat/until-destroy";
import {Component} from "@angular/core";

@Component({
  selector: 'base-component',
  templateUrl: './match.component.html'
})
@UntilDestroy()
export class MatchComponent{

}
