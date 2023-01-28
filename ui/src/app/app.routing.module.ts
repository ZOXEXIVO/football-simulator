import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { countryRoutes } from './components/countries/country.routes';

const routes: Routes = [
  ...countryRoutes,
  {path: '**', redirectTo: '/countries', pathMatch: 'full'}
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
