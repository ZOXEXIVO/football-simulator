import { Injectable } from "@angular/core";
import { Title } from "@angular/platform-browser";

const DEFAULT_TITLE: string = "Open Football";

@Injectable({
    providedIn: 'root',
 })
 export class TitleService {
    constructor(private title: Title) {
    }
 
    setTitle(title: String) {
        this.title.setTitle(title + ' - ' + DEFAULT_TITLE);
    } 
 }