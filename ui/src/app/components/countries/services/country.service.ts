import { HttpClient } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { Observable } from "rxjs";

@Injectable({
    providedIn: 'root',
})
export class CountryService {
    constructor(private http: HttpClient) {         
    }

    getList(): Observable<CountryListDto[]> {
        return this.http.get<CountryListDto[]>('/api/countries');
    }

    get(slug: String): Observable<CountryDto> {
        return this.http.get<CountryDto>('/api/countries/' + slug);
    }    
}

// List

export interface CountryListDto {    
    name: String;
    countries: CountryListItemDto[];
 }

export interface CountryListItemDto {
    slug: String,
    code: String,
    name: String,
    leagues: LeagueDto[]
 }

 export interface LeagueDto {
    slug: String;
    name: String,
 }

 // Get

 export interface CountryDto {    
    slug: String,
    name: String,
    code: String,
    continent_name: String,
    leagues: LeagueDto[]
 }