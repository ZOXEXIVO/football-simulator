import { HttpClient } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { Observable } from "rxjs";

@Injectable({
    providedIn: 'root',
 })
 export class ProcessService {
    constructor(private http: HttpClient) {
    }
 
    process(): Observable<Object>{
        return this.http.post('/api/game/process', {});        
    } 
 }