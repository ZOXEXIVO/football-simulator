import { Injectable } from "@angular/core";
import { BehaviorSubject, of } from "rxjs";

@Injectable({
    providedIn: 'root',
})
export class TopHeaderService {
    public content$: BehaviorSubject<ContentDto> = new BehaviorSubject<ContentDto>(      {
      mainContent: '', subContent: '', subContentLink: '', showIcon: false}
    );

    setContent(mainContent: String, subContent: String, subContentLink: String, showIcon: boolean = true) {
        this.content$.next(new ContentDto(mainContent, subContent, subContentLink, showIcon));
    }
}

export class ContentDto {
  constructor(mainContent: String, subContent: String, subContentLink: String, showIcon: boolean = true) {
    this.mainContent = mainContent;

    this.subContent = subContent;
    this.subContentLink = subContentLink;
    this.showIcon = showIcon;
  }

  public mainContent: String;

  public subContent: String;
  public subContentLink: String;

  public showIcon: boolean;
}
