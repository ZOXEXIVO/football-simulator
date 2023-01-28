import { Injectable } from "@angular/core";
import { Observable } from "rxjs";
import { CountryApiClient, CountryDto } from "src/client/country.api.client";

@Injectable({
    providedIn: 'root',
})
export class CountryService {
    constructor(private client: CountryApiClient) { }

    getList(): Observable<CountryDto[]> {
        return this.client.getList();
    }
}
