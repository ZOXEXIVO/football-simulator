import { Injectable } from "@angular/core";
import { BehaviorSubject, of } from "rxjs";

@Injectable({
    providedIn: 'root',
})
export class TopHeaderService {
    public content$: BehaviorSubject<ContentDto> = new BehaviorSubject<ContentDto>(      {
      mainContent: '', subContent: '', subContentLink: ''}
    );

    setContent(mainContent: String, subContent: String, subContentLink: String) {
        this.content$.next(new ContentDto(mainContent, subContent, subContentLink));
    }
}

export class ContentDto {
  constructor(mainContent: String, subContent: String, subContentLink: String) {
    this.mainContent = mainContent;

    this.subContent = subContent;
    this.subContentLink = subContentLink;
  }

  public mainContent: String;

  public subContent: String;
  public subContentLink: String;
}
