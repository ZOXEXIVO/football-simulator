import { Injectable } from "@angular/core";
import { BehaviorSubject, of } from "rxjs";

@Injectable({
    providedIn: 'root',
})
export class TopMenuService {
    public items$: BehaviorSubject<MenuSection[]> = new BehaviorSubject<MenuSection[]>([]);

    setMenu(items: MenuSection[]) {
        this.items$.next(items);
    }
}

export class MenuSection {
    public items: MenuItem[] | undefined
}

export class MenuItem {
    public title: String | undefined;
    public url: String | undefined;
    public icon: String | undefined;
}