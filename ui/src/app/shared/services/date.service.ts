import { HttpClient } from "@angular/common/http";
import { Injectable } from "@angular/core";

@Injectable({
    providedIn: 'root',
 })
 export class DateService {
    constructor(private http: HttpClient) {
    }
 
    get_current() {
        return this.http.get<DateDto>('/api/date');
    } 
 }

 export interface DateDto {
    date: string,
    time: string
 }