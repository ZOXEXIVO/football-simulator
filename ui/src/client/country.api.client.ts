import { Injectable } from "@angular/core";
import { HttpClient } from '@angular/common/http';
import { map, Observable } from "rxjs";

@Injectable({
    providedIn: 'root',
})
export class CountryApiClient {
    constructor(private http: HttpClient) { }

    getList(): Observable<CountryDto[]> {
        return this.http.get<CountryDto[]>('/api/countries');
    }
}

export interface CountryDto {    
    name: String;
    countries: CountryItemDto[];
 }

export interface CountryItemDto {
    slug: String;
    code: String,
    name: String,
    leagues: LeagueDto[]
 }

 export interface LeagueDto {
    slug: String;
    name: String,
 }