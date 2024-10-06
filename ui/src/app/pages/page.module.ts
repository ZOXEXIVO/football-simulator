import {NgModule} from '@angular/core';

import {PageComponent} from "./page.component";
import {CountryModule} from "./countries/country.module";
import {PlayerModule} from "./players/player.module";
import {LeagueModule} from "./leagues/league.module";
import {TeamModule} from "./teams/team.module";
import {RouterModule} from "@angular/router";
import {pageRoutes} from "./page.routing.module";
import {CommonModule} from "@angular/common";
import {SharedModule} from "../shared/shared.module";

@NgModule({
    declarations: [
        PageComponent,
    ],
    imports: [
        SharedModule,
        CommonModule,
        PlayerModule,
        CountryModule,
        LeagueModule,
        TeamModule,
        RouterModule.forChild(pageRoutes)
    ],
    bootstrap: [PageComponent]
})
export class PageModule {
}
