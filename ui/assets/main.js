"use strict";
(self["webpackChunkfootball_simulator"] = self["webpackChunkfootball_simulator"] || []).push([["main"],{

/***/ 5041:
/*!**********************************!*\
  !*** ./src/app/app.component.ts ***!
  \**********************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "AppComponent": () => (/* binding */ AppComponent)
/* harmony export */ });
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _components_shared_left_menu_left_menu_component__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./components/shared/left-menu/left.menu.component */ 5910);
/* harmony import */ var _components_shared_top_header_top_header_component__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./components/shared/top-header/top.header.component */ 2400);
/* harmony import */ var _angular_router__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! @angular/router */ 124);




class AppComponent {
  constructor() {
    this.title = 'football-simulator';
  }
}
AppComponent.ɵfac = function AppComponent_Factory(t) {
  return new (t || AppComponent)();
};
AppComponent.ɵcmp = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵdefineComponent"]({
  type: AppComponent,
  selectors: [["app-root"]],
  decls: 44,
  vars: 0,
  consts: [[1, "main_container"], [1, "main_content"], [1, "col-md-3", "left_col"], [1, "header_row"], [1, "header_top"], [1, "col-md-6", 2, "padding-left", "40px"], [1, "col-md-6"], [1, "nav", "navbar-right"], [1, "item-msg", 2, "cursor", "pointer"], ["id", "process-game-btn", 2, "text-transform", "uppercase", "text-align", "center"], [2, "margin-top", "5px", "width", "100px", "padding-top", "20px"], ["id", "process-game-btn-spinner", 1, "spinner", 2, "display", "none"], [1, "item-date"], ["id", "date-up", 1, "date-title"], ["id", "date-down", 1, "date-body"], [1, "btn-group", "btn-drop", "btn-globe"], ["data-toggle", "dropdown", "href", "/#", "aria-expanded", "false", 1, "btn", "dropdown-toggle"], [1, "fas", "fa-globe-americas"], [1, "dropdown-menu"], ["href", "/#"], ["href", "/#", 1, "link-search"], [1, "fas", "fa-search"], [2, "margin", "0", "padding", "0"], [1, "right_col"], [2, "clear", "both"]],
  template: function AppComponent_Template(rf, ctx) {
    if (rf & 1) {
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementStart"](0, "div", 0)(1, "div")(2, "div", 1)(3, "div", 2);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelement"](4, "left-menu");
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementEnd"]();
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementStart"](5, "div", 3)(6, "div", 4)(7, "div", 5);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelement"](8, "top-header");
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementEnd"]();
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementStart"](9, "div", 6)(10, "ul", 7)(11, "li", 8)(12, "div", 9);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵtext"](13, "Process");
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementEnd"]();
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementStart"](14, "div", 10);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelement"](15, "div", 11);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementEnd"]()();
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementStart"](16, "li", 12);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelement"](17, "div", 13)(18, "div", 14);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementEnd"]();
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementStart"](19, "li")(20, "div", 15)(21, "a", 16);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelement"](22, "i", 17);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementEnd"]();
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementStart"](23, "ul", 18)(24, "li")(25, "a", 19);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵtext"](26, "Item 1");
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementEnd"]()();
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementStart"](27, "li")(28, "a", 19);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵtext"](29, "Item 2");
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementEnd"]()();
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementStart"](30, "li")(31, "a", 19);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵtext"](32, "Item 3");
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementEnd"]()();
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementStart"](33, "li")(34, "a", 19);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵtext"](35, "Item 4");
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementEnd"]()()()()();
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementStart"](36, "li")(37, "a", 20);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelement"](38, "i", 21);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementEnd"]()()()()()();
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelement"](39, "hr", 22);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementStart"](40, "div", 23)(41, "div", 1);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelement"](42, "router-outlet")(43, "div", 24);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementEnd"]()()()()();
    }
  },
  dependencies: [_components_shared_left_menu_left_menu_component__WEBPACK_IMPORTED_MODULE_0__.LeftMenuComponent, _components_shared_top_header_top_header_component__WEBPACK_IMPORTED_MODULE_1__.TopHeaderComponent, _angular_router__WEBPACK_IMPORTED_MODULE_3__.RouterOutlet],
  styles: ["\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IiIsInNvdXJjZVJvb3QiOiIifQ== */"]
});

/***/ }),

/***/ 6747:
/*!*******************************!*\
  !*** ./src/app/app.module.ts ***!
  \*******************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "AppModule": () => (/* binding */ AppModule)
/* harmony export */ });
/* harmony import */ var _angular_platform_browser__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__(/*! @angular/platform-browser */ 4497);
/* harmony import */ var _app_routing_module__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./app.routing.module */ 3670);
/* harmony import */ var _app_component__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./app.component */ 5041);
/* harmony import */ var _components_countries_country_module__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./components/countries/country.module */ 2618);
/* harmony import */ var _components_shared_shared_module__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./components/shared/shared.module */ 4065);
/* harmony import */ var _angular_common__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! @angular/common */ 4666);
/* harmony import */ var _angular_common_http__WEBPACK_IMPORTED_MODULE_9__ = __webpack_require__(/*! @angular/common/http */ 8987);
/* harmony import */ var _components_leagues_league_module__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./components/leagues/league.module */ 481);
/* harmony import */ var _components_teams_team_module__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./components/teams/team.module */ 1809);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! @angular/core */ 2560);










class AppModule {}
AppModule.ɵfac = function AppModule_Factory(t) {
  return new (t || AppModule)();
};
AppModule.ɵmod = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_6__["ɵɵdefineNgModule"]({
  type: AppModule,
  bootstrap: [_app_component__WEBPACK_IMPORTED_MODULE_1__.AppComponent]
});
AppModule.ɵinj = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_6__["ɵɵdefineInjector"]({
  providers: [{
    provide: _angular_common__WEBPACK_IMPORTED_MODULE_7__.APP_BASE_HREF,
    useValue: '/'
  }],
  imports: [_components_shared_shared_module__WEBPACK_IMPORTED_MODULE_3__.SharedModule, _angular_platform_browser__WEBPACK_IMPORTED_MODULE_8__.BrowserModule, _app_routing_module__WEBPACK_IMPORTED_MODULE_0__.AppRoutingModule, _components_countries_country_module__WEBPACK_IMPORTED_MODULE_2__.CountryModule, _components_leagues_league_module__WEBPACK_IMPORTED_MODULE_4__.LeagueModule, _components_teams_team_module__WEBPACK_IMPORTED_MODULE_5__.TeamModule, _angular_common_http__WEBPACK_IMPORTED_MODULE_9__.HttpClientModule]
});
(function () {
  (typeof ngJitMode === "undefined" || ngJitMode) && _angular_core__WEBPACK_IMPORTED_MODULE_6__["ɵɵsetNgModuleScope"](AppModule, {
    declarations: [_app_component__WEBPACK_IMPORTED_MODULE_1__.AppComponent],
    imports: [_components_shared_shared_module__WEBPACK_IMPORTED_MODULE_3__.SharedModule, _angular_platform_browser__WEBPACK_IMPORTED_MODULE_8__.BrowserModule, _app_routing_module__WEBPACK_IMPORTED_MODULE_0__.AppRoutingModule, _components_countries_country_module__WEBPACK_IMPORTED_MODULE_2__.CountryModule, _components_leagues_league_module__WEBPACK_IMPORTED_MODULE_4__.LeagueModule, _components_teams_team_module__WEBPACK_IMPORTED_MODULE_5__.TeamModule, _angular_common_http__WEBPACK_IMPORTED_MODULE_9__.HttpClientModule]
  });
})();

/***/ }),

/***/ 3670:
/*!***************************************!*\
  !*** ./src/app/app.routing.module.ts ***!
  \***************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "AppRoutingModule": () => (/* binding */ AppRoutingModule)
/* harmony export */ });
/* harmony import */ var _angular_router__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! @angular/router */ 124);
/* harmony import */ var _components_countries_country_routes__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./components/countries/country.routes */ 2405);
/* harmony import */ var _components_leagues_league_routes__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./components/leagues/league.routes */ 6214);
/* harmony import */ var _components_teams_team_routes__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./components/teams/team.routes */ 6406);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! @angular/core */ 2560);






const routes = [..._components_countries_country_routes__WEBPACK_IMPORTED_MODULE_0__.countryRoutes, ..._components_leagues_league_routes__WEBPACK_IMPORTED_MODULE_1__.leagueRoutes, ..._components_teams_team_routes__WEBPACK_IMPORTED_MODULE_2__.teamRoutes, {
  path: '**',
  redirectTo: '/countries',
  pathMatch: 'full'
}];
class AppRoutingModule {}
AppRoutingModule.ɵfac = function AppRoutingModule_Factory(t) {
  return new (t || AppRoutingModule)();
};
AppRoutingModule.ɵmod = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵdefineNgModule"]({
  type: AppRoutingModule
});
AppRoutingModule.ɵinj = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵdefineInjector"]({
  imports: [_angular_router__WEBPACK_IMPORTED_MODULE_4__.RouterModule.forRoot(routes), _angular_router__WEBPACK_IMPORTED_MODULE_4__.RouterModule]
});
(function () {
  (typeof ngJitMode === "undefined" || ngJitMode) && _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵsetNgModuleScope"](AppRoutingModule, {
    imports: [_angular_router__WEBPACK_IMPORTED_MODULE_4__.RouterModule],
    exports: [_angular_router__WEBPACK_IMPORTED_MODULE_4__.RouterModule]
  });
})();

/***/ }),

/***/ 2618:
/*!********************************************************!*\
  !*** ./src/app/components/countries/country.module.ts ***!
  \********************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "CountryModule": () => (/* binding */ CountryModule)
/* harmony export */ });
/* harmony import */ var _angular_common_http__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! @angular/common/http */ 8987);
/* harmony import */ var _angular_platform_browser__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! @angular/platform-browser */ 4497);
/* harmony import */ var _shared_shared_module__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../shared/shared.module */ 4065);
/* harmony import */ var _get_country_get_component__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./get/country.get.component */ 7722);
/* harmony import */ var _list_country_list_component__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./list/country.list.component */ 8909);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! @angular/core */ 2560);






class CountryModule {}
CountryModule.ɵfac = function CountryModule_Factory(t) {
  return new (t || CountryModule)();
};
CountryModule.ɵmod = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵdefineNgModule"]({
  type: CountryModule
});
CountryModule.ɵinj = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵdefineInjector"]({
  imports: [_shared_shared_module__WEBPACK_IMPORTED_MODULE_0__.SharedModule, _angular_platform_browser__WEBPACK_IMPORTED_MODULE_4__.BrowserModule, _angular_common_http__WEBPACK_IMPORTED_MODULE_5__.HttpClientModule]
});
(function () {
  (typeof ngJitMode === "undefined" || ngJitMode) && _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵsetNgModuleScope"](CountryModule, {
    declarations: [_get_country_get_component__WEBPACK_IMPORTED_MODULE_1__.CountryGetComponent, _list_country_list_component__WEBPACK_IMPORTED_MODULE_2__.CountryListComponent],
    imports: [_shared_shared_module__WEBPACK_IMPORTED_MODULE_0__.SharedModule, _angular_platform_browser__WEBPACK_IMPORTED_MODULE_4__.BrowserModule, _angular_common_http__WEBPACK_IMPORTED_MODULE_5__.HttpClientModule]
  });
})();

/***/ }),

/***/ 2405:
/*!********************************************************!*\
  !*** ./src/app/components/countries/country.routes.ts ***!
  \********************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "countryRoutes": () => (/* binding */ countryRoutes)
/* harmony export */ });
/* harmony import */ var _get_country_get_component__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./get/country.get.component */ 7722);
/* harmony import */ var _list_country_list_component__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./list/country.list.component */ 8909);


const countryRoutes = [{
  path: 'countries',
  component: _list_country_list_component__WEBPACK_IMPORTED_MODULE_1__.CountryListComponent
}, {
  path: 'countries/:slug',
  component: _get_country_get_component__WEBPACK_IMPORTED_MODULE_0__.CountryGetComponent
  // children: [
  //   { path: '', component: ClubMentionComponent },
  //   { path: 'page/:page', component: ClubMentionComponent },
  //   { path: 'players', component: ClubPlayerComponent },
  // ]
}];

/***/ }),

/***/ 7722:
/*!*******************************************************************!*\
  !*** ./src/app/components/countries/get/country.get.component.ts ***!
  \*******************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "CountryGetComponent": () => (/* binding */ CountryGetComponent)
/* harmony export */ });
/* harmony import */ var tslib__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__(/*! tslib */ 4929);
/* harmony import */ var _ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! @ngneat/until-destroy */ 2777);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _shared_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../../shared/left-menu/services/left.menu.service */ 3456);
/* harmony import */ var _services_country_service__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../services/country.service */ 6399);
/* harmony import */ var _angular_router__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! @angular/router */ 124);
/* harmony import */ var src_app_shared_services_title_service__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! src/app/shared/services/title.service */ 7153);
/* harmony import */ var _shared_top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ../../shared/top-header/content/top.header.content.component */ 3278);
/* harmony import */ var _angular_common__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! @angular/common */ 4666);
var _class;









function CountryGetComponent_div_1_li_13_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "li")(1, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
  }
  if (rf & 2) {
    const league_r2 = ctx.$implicit;
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpropertyInterpolate1"]("href", "/leagues/", league_r2.slug, "", _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", league_r2.name, " ");
  }
}
function CountryGetComponent_div_1_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "div")(1, "div", 2)(2, "div", 3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](3, "div");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](4, "div", 3)(5, "div", 4)(6, "div", 5);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](7);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](8, "div")(9, "a", 6);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](11, "div")(12, "ul", 7);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](13, CountryGetComponent_div_1_li_13_Template, 3, 2, "li", 8);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](14, "div", 9);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
  }
  if (rf & 2) {
    const ctx_r0 = _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵnextContext"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵclassMapInterpolate1"]("flag flag-", ctx_r0.country.code, "");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](4);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate"](ctx_r0.country.name);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", ctx_r0.country.continent_name, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngForOf", ctx_r0.country.leagues);
  }
}
let CountryGetComponent = (_class = class CountryGetComponent {
  constructor(leftMenuService, service, route, titleService) {
    this.leftMenuService = leftMenuService;
    this.service = service;
    this.route = route;
    this.titleService = titleService;
    this.country = null;
  }
  ngOnInit() {
    this.leftMenuService.setMenu([{
      items: [{
        url: '/',
        title: 'Home',
        icon: 'fa-home'
      }]
    }]);
    this.route.params.subscribe(params => {
      this.service.get(params["slug"]).pipe((0,_ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_5__.untilDestroyed)(this)).subscribe(countryData => {
        this.country = countryData;
        this.titleService.setTitle(countryData.name + ', ' + countryData.continent_name);
      });
    });
  }
}, _class.ɵfac = function CountryGetComponent_Factory(t) {
  return new (t || _class)(_angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](_shared_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__.LeftMenuService), _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](_services_country_service__WEBPACK_IMPORTED_MODULE_1__.CountryService), _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](_angular_router__WEBPACK_IMPORTED_MODULE_6__.ActivatedRoute), _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](src_app_shared_services_title_service__WEBPACK_IMPORTED_MODULE_2__.TitleService));
}, _class.ɵcmp = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdefineComponent"]({
  type: _class,
  selectors: [["ng-component"]],
  decls: 2,
  vars: 1,
  consts: [[1, "content"], [4, "ngIf"], ["topHeader", ""], [1, "header-block"], [1, "header-title-block"], [1, "page-title"], ["href", "/countries", 1, "sub-title-link"], [1, "countries-list"], [4, "ngFor", "ngForOf"], [2, "clear", "both"], [3, "href"]],
  template: function CountryGetComponent_Template(rf, ctx) {
    if (rf & 1) {
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "div", 0);
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](1, CountryGetComponent_div_1_Template, 15, 6, "div", 1);
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    }
    if (rf & 2) {
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngIf", ctx.country);
    }
  },
  dependencies: [_shared_top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_3__.TopHeaderContentComponent, _angular_common__WEBPACK_IMPORTED_MODULE_7__.NgForOf, _angular_common__WEBPACK_IMPORTED_MODULE_7__.NgIf],
  styles: ["\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IiIsInNvdXJjZVJvb3QiOiIifQ== */"]
}), _class);
CountryGetComponent = (0,tslib__WEBPACK_IMPORTED_MODULE_8__.__decorate)([(0,_ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_5__.UntilDestroy)()], CountryGetComponent);


/***/ }),

/***/ 8909:
/*!*********************************************************************!*\
  !*** ./src/app/components/countries/list/country.list.component.ts ***!
  \*********************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "CountryListComponent": () => (/* binding */ CountryListComponent)
/* harmony export */ });
/* harmony import */ var rxjs__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! rxjs */ 745);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _shared_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../../shared/left-menu/services/left.menu.service */ 3456);
/* harmony import */ var _services_country_service__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../services/country.service */ 6399);
/* harmony import */ var src_app_shared_services_title_service__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! src/app/shared/services/title.service */ 7153);
/* harmony import */ var _shared_top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ../../shared/top-header/content/top.header.content.component */ 3278);
/* harmony import */ var _angular_common__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! @angular/common */ 4666);







function CountryListComponent_div_9_li_6_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "li")(1, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](2, "div");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
  }
  if (rf & 2) {
    const countryItem_r3 = ctx.$implicit;
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpropertyInterpolate1"]("href", "/countries/", countryItem_r3.slug, "", _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵclassMapInterpolate1"]("flag flag-", countryItem_r3.code, "");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"]("\u00A0\u00A0 ", countryItem_r3.name, " ");
  }
}
function CountryListComponent_div_9_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "div", 6)(1, "h2");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](3, "hr");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](4, "div")(5, "ul", 7);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](6, CountryListComponent_div_9_li_6_Template, 4, 5, "li", 8);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](7, "div", 9);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
  }
  if (rf & 2) {
    const country_r1 = ctx.$implicit;
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate2"]("", country_r1.name, " (", country_r1.countries.length, ")");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](4);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngForOf", country_r1.countries);
  }
}
class CountryListComponent {
  constructor(leftMenuService, service, titleService) {
    this.leftMenuService = leftMenuService;
    this.titleService = titleService;
    this.countries$ = (0,rxjs__WEBPACK_IMPORTED_MODULE_5__.of)([]);
    this.countries$ = service.getList();
    this.titleService.setTitle('Available countries');
  }
  ngOnInit() {
    this.leftMenuService.setMenu([{
      items: [{
        url: '/',
        title: 'Home',
        icon: 'fa-home'
      }]
    }]);
  }
}
CountryListComponent.ɵfac = function CountryListComponent_Factory(t) {
  return new (t || CountryListComponent)(_angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](_shared_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__.LeftMenuService), _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](_services_country_service__WEBPACK_IMPORTED_MODULE_1__.CountryService), _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](src_app_shared_services_title_service__WEBPACK_IMPORTED_MODULE_2__.TitleService));
};
CountryListComponent.ɵcmp = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdefineComponent"]({
  type: CountryListComponent,
  selectors: [["country-list"]],
  decls: 11,
  vars: 3,
  consts: [["topHeader", ""], [2, "float", "left"], [1, "header-title-block"], [1, "page-title"], [1, "content"], ["class", "country-box", 4, "ngFor", "ngForOf"], [1, "country-box"], [1, "countries-list"], [4, "ngFor", "ngForOf"], [2, "clear", "both"], [3, "href"]],
  template: function CountryListComponent_Template(rf, ctx) {
    if (rf & 1) {
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "div", 0)(1, "div", 1)(2, "div", 2)(3, "div", 3);
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](4, "Select country");
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](5, "div");
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](6, "Select any country to inspect it all");
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()();
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](7, "div", 4)(8, "div");
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](9, CountryListComponent_div_9_Template, 8, 3, "div", 5);
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpipe"](10, "async");
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    }
    if (rf & 2) {
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](9);
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngForOf", _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpipeBind1"](10, 1, ctx.countries$));
    }
  },
  dependencies: [_shared_top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_3__.TopHeaderContentComponent, _angular_common__WEBPACK_IMPORTED_MODULE_6__.NgForOf, _angular_common__WEBPACK_IMPORTED_MODULE_6__.AsyncPipe],
  styles: ["\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IiIsInNvdXJjZVJvb3QiOiIifQ== */"]
});

/***/ }),

/***/ 6399:
/*!******************************************************************!*\
  !*** ./src/app/components/countries/services/country.service.ts ***!
  \******************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "CountryService": () => (/* binding */ CountryService)
/* harmony export */ });
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _angular_common_http__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @angular/common/http */ 8987);


class CountryService {
  constructor(http) {
    this.http = http;
  }
  getList() {
    return this.http.get('/api/countries');
  }
  get(slug) {
    return this.http.get('/api/countries/' + slug);
  }
}
CountryService.ɵfac = function CountryService_Factory(t) {
  return new (t || CountryService)(_angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵinject"](_angular_common_http__WEBPACK_IMPORTED_MODULE_1__.HttpClient));
};
CountryService.ɵprov = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵdefineInjectable"]({
  token: CountryService,
  factory: CountryService.ɵfac,
  providedIn: 'root'
});

/***/ }),

/***/ 2406:
/*!****************************************************************!*\
  !*** ./src/app/components/leagues/get/league.get.component.ts ***!
  \****************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "LeagueGetComponent": () => (/* binding */ LeagueGetComponent)
/* harmony export */ });
/* harmony import */ var tslib__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__(/*! tslib */ 4929);
/* harmony import */ var _ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! @ngneat/until-destroy */ 2777);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _shared_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../../shared/left-menu/services/left.menu.service */ 3456);
/* harmony import */ var _services_league_service__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../services/league.service */ 7070);
/* harmony import */ var _angular_router__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! @angular/router */ 124);
/* harmony import */ var src_app_shared_services_title_service__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! src/app/shared/services/title.service */ 7153);
/* harmony import */ var _shared_top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ../../shared/top-header/content/top.header.content.component */ 3278);
/* harmony import */ var _angular_common__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! @angular/common */ 4666);
var _class;









function LeagueGetComponent_div_0_tr_59_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "tr")(1, "td", 37);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](3, "td")(4, "a", 38);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](5);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](6, "td", 39);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](7);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](8, "td", 39);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](9);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](10, "td", 39);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](11);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](12, "td", 39);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](13);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](14, "td", 39);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](15);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](16, "td", 39);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](17);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
  }
  if (rf & 2) {
    const row_r3 = ctx.$implicit;
    const i_r4 = ctx.index;
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", i_r4, "st ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpropertyInterpolate1"]("href", "/teams/", row_r3.team_slug, "", _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", row_r3.team_name, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", row_r3.played, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", row_r3.win, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", row_r3.draft, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", row_r3.lost, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", row_r3.goal_scored - row_r3.goal_concerned, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", row_r3.points, " ");
  }
}
function LeagueGetComponent_div_0_table_65_tr_4_span_6_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "span");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
  }
  if (rf & 2) {
    const item_r7 = _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵnextContext"]().$implicit;
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate2"](" ", item_r7.result.home_goals, " - ", item_r7.result.away_goals, " ");
  }
}
function LeagueGetComponent_div_0_table_65_tr_4_span_7_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "span");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](1, "-");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
  }
}
function LeagueGetComponent_div_0_table_65_tr_4_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "tr")(1, "td")(2, "a", 38);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](4, "td", 42)(5, "a", 38);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](6, LeagueGetComponent_div_0_table_65_tr_4_span_6_Template, 2, 2, "span", 0);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](7, LeagueGetComponent_div_0_table_65_tr_4_span_7_Template, 2, 0, "span", 0);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](8, "td")(9, "a", 38);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()();
  }
  if (rf & 2) {
    const item_r7 = ctx.$implicit;
    const ctx_r6 = _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵnextContext"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpropertyInterpolate1"]("href", "/teams/", item_r7.home_team_slug, "", _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", item_r7.home_team_name, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpropertyInterpolate2"]("href", "/match/", ctx_r6.league.slug, "/", item_r7.match_id, "", _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngIf", item_r7.result);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngIf", !item_r7.result);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpropertyInterpolate1"]("href", "/teams/", item_r7.away_team_slug, "", _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", item_r7.away_team_name, " ");
  }
}
function LeagueGetComponent_div_0_table_65_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "table", 40)(1, "tr")(2, "td", 41);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](4, LeagueGetComponent_div_0_table_65_tr_4_Template, 11, 8, "tr", 18);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
  }
  if (rf & 2) {
    const tour_schedule_r5 = ctx.$implicit;
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", tour_schedule_r5.date, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngForOf", tour_schedule_r5.matches);
  }
}
function LeagueGetComponent_div_0_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "div")(1, "div", 1)(2, "div", 2)(3, "div", 3)(4, "div", 4);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](5);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](6, "div");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](7, "div", 5);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](8, "a", 6);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](9);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](10, "div", 7)(11, "div", 8)(12, "ul", 9)(13, "li")(14, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](15, "Overview");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](16, "li")(17, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](18, "Matches");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](19, "li")(20, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](21, "News");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](22, "li")(23, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](24, "Stats");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](25, "li")(26, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](27, "Transfers");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](28, "li")(29, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](30, "Awards");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](31, "li")(32, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](33, "History");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](34, "br");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](35, "div")(36, "div", 11)(37, "div", 12)(38, "p", 13);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](39, "Stages >");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](40, "div")(41, "table", 14)(42, "tr")(43, "th", 15);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](44, " Pos ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](45, "th", 16);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](46, " Team ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](47, "th", 17);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](48, " P ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](49, "th", 17);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](50, " W ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](51, "th", 17);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](52, " D ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](53, "th", 17);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](54, " L ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](55, "th", 17);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](56, " GD ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](57, "th", 17);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](58, " PTS ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](59, LeagueGetComponent_div_0_tr_59_Template, 18, 9, "tr", 18);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](60, "div", 19)(61, "div", 20)(62, "div", 21)(63, "p", 13);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](64, "Fixtures/results >");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](65, LeagueGetComponent_div_0_table_65_Template, 5, 2, "table", 22);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](66, "div", 21)(67, "p", 13);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](68, "Competition Reputation >");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](69, "table", 23)(70, "tr")(71, "td", 24);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](72, "1");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](73, "td")(74, "p");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](75, "Spanish Premier Division");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](76, "p")(77, "div", 25);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](78, "\u00A0Spain ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](79, "tr")(80, "td", 24);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](81, "2");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](82, "td")(83, "p");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](84, "German First Division");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](85, "p")(86, "div", 26);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](87, "\u00A0\u00A0Germany ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](88, "tr")(89, "td", 24);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](90, "2");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](91, "td")(92, "p");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](93, "English Premiere Division");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](94, "p")(95, "div", 27);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](96, "\u00A0England ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](97, "div", 11)(98, "div", 28)(99, "p", 13);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](100, "Player stats >");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](101, "table", 29)(102, "tr")(103, "td")(104, "p", 30);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](105, "Goals");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](106, "table", 23)(107, "tr")(108, "td", 24);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](109, " 1st ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](110, "td", 31);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](111, "div", 32);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](112, "td");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](113, " Ezequiel Ponce ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](114, "p", 33);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](115, "Spartak moscow");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](116, "td", 34);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](117, " 14 ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](118, "tr")(119, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](120, " 2st ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](121, "td", 31);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](122, "div", 32);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](123, "td");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](124, " Jordan Larsson ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](125, "p", 33);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](126, "Spartak moscow");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](127, "td", 34);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](128, " 10 ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](129, "td")(130, "p", 30);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](131, "Average rating");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](132, "table", 23)(133, "tr")(134, "td", 24);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](135, " 1st ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](136, "td", 31);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](137, "div", 32);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](138, "td");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](139, " Guus Til ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](140, "p", 33);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](141, "Spartak moscow");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](142, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](143, " 7.33 ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](144, "tr")(145, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](146, " 2st ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](147, "td", 31);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](148, "div", 32);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](149, "td");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](150, " Andr\u00E9 Sch\u00FCrrle ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](151, "p", 33);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](152, "Spartak moscow");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](153, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](154, " 7.01 ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](155, "td")(156, "p", 30);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](157, "Assists");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](158, "table", 23)(159, "tr")(160, "td", 24);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](161, " 1st ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](162, "td", 31);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](163, "div", 32);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](164, "td");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](165, " Alex Kral ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](166, "p", 33);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](167, "Spartak moscow");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](168, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](169, " 14 ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](170, "tr")(171, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](172, " 2st ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](173, "td", 31);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](174, "div", 32);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](175, "td");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](176, " Nail Umyarov ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](177, "p", 33);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](178, "Spartak moscow");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](179, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](180, " 10 ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](181, "tr")(182, "td")(183, "p", 30);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](184, "Player of the match");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](185, "table", 23)(186, "tr")(187, "td", 24);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](188, " 1st ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](189, "td", 31);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](190, "div", 32);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](191, "td");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](192, " Alex Kral ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](193, "p", 33);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](194, "Spartak moscow");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](195, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](196, " 7 ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](197, "tr")(198, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](199, " 2st ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](200, "td", 31);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](201, "div", 32);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](202, "td");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](203, " Georgiy Dzhikiya ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](204, "p", 33);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](205, "Spartak moscow");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](206, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](207, " 5 ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](208, "td")(209, "p", 30);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](210, "Clean sheets");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](211, "table", 23)(212, "tr")(213, "td", 24);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](214, " 1st ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](215, "td", 31);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](216, "div", 32);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](217, "td");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](218, " Aleksandr Maksimenko ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](219, "p", 33);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](220, "Spartak moscow");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](221, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](222, " 5 ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](223, "tr")(224, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](225, " 2st ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](226, "td", 31);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](227, "div", 32);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](228, "td");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](229, " Aleksandr Selikhov ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](230, "p", 33);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](231, "Spartak moscow");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](232, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](233, " 3 ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](234, "td")(235, "p", 30);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](236, "Ywllow cards");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](237, "table", 23)(238, "tr")(239, "td", 24);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](240, " 1st ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](241, "td", 31);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](242, "div", 32);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](243, "td");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](244, " Reziuan Mirzov ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](245, "p", 33);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](246, "Spartak moscow");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](247, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](248, " 8 ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](249, "tr")(250, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](251, " 2st ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](252, "td", 31);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](253, "div", 32);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](254, "td");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](255, " Alex Kral ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](256, "p", 33);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](257, "Spartak moscow");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](258, "td", 35);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](259, " 4 ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()()()()()()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](260, "div", 36);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
  }
  if (rf & 2) {
    const ctx_r0 = _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵnextContext"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](5);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate"](ctx_r0.league.name);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpropertyInterpolate1"]("href", "/countries/", ctx_r0.league.country_slug, "", _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", ctx_r0.league.country_name, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](50);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngForOf", ctx_r0.league.table.rows);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](6);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngForOf", ctx_r0.league.current_tour_schedule);
  }
}
let LeagueGetComponent = (_class = class LeagueGetComponent {
  constructor(leftMenuService, service, route, titleService) {
    this.leftMenuService = leftMenuService;
    this.service = service;
    this.route = route;
    this.titleService = titleService;
    this.league = null;
  }
  ngOnInit() {
    this.leftMenuService.setMenu([{
      items: [{
        url: '/',
        title: 'Home',
        icon: 'fa-home'
      }]
    }]);
    this.route.params.subscribe(params => {
      this.service.get(params["slug"]).pipe((0,_ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_5__.untilDestroyed)(this)).subscribe(leagueData => {
        this.league = leagueData;
        this.titleService.setTitle(leagueData.name + ', ' + leagueData.country_name);
      });
    });
  }
}, _class.ɵfac = function LeagueGetComponent_Factory(t) {
  return new (t || _class)(_angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](_shared_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__.LeftMenuService), _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](_services_league_service__WEBPACK_IMPORTED_MODULE_1__.LeagueService), _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](_angular_router__WEBPACK_IMPORTED_MODULE_6__.ActivatedRoute), _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](src_app_shared_services_title_service__WEBPACK_IMPORTED_MODULE_2__.TitleService));
}, _class.ɵcmp = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdefineComponent"]({
  type: _class,
  selectors: [["ng-component"]],
  decls: 1,
  vars: 1,
  consts: [[4, "ngIf"], ["topHeader", ""], [1, "header-block"], [1, "header-title-block"], [1, "page-title"], [1, "flag", "flag-ru", "custom-flag"], [1, "sub-title-link", 3, "href"], [1, "header_row"], [1, "header_bottom"], [1, "nav", "navbar-left"], ["href", "/#"], [1, "row"], [1, "col-xs-5", "col-md-4", "col-sm-5", "delim"], [1, "header"], [1, "league-table"], [2, "width", "50px"], [2, "width", "60%", "min-width", "180px"], [2, "width", "50px", "text-align", "center"], [4, "ngFor", "ngForOf"], [1, "col-xs-7", "delim"], [1, "row", 2, "border-bottom", "2px solid #323A41"], [1, "col-xs-6"], ["class", "week-results", 4, "ngFor", "ngForOf"], [2, "width", "100%"], [2, "width", "20px"], [1, "flag", "flag-es"], [1, "flag", "flag-de"], [1, "flag", "flag-gb"], [1, "col-xs-12"], [1, "statistics-table"], [1, "header", "rating"], [2, "width", "30px"], [1, "club-logo", "club-default-logo"], [1, "club-name"], [2, "width", "25px"], [2, "width", "15px"], [2, "clear", "both"], [1, "idx"], [3, "href"], [2, "text-align", "center"], [1, "week-results"], ["colspan", "3"], [1, "score"]],
  template: function LeagueGetComponent_Template(rf, ctx) {
    if (rf & 1) {
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](0, LeagueGetComponent_div_0_Template, 261, 5, "div", 0);
    }
    if (rf & 2) {
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngIf", ctx.league);
    }
  },
  dependencies: [_shared_top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_3__.TopHeaderContentComponent, _angular_common__WEBPACK_IMPORTED_MODULE_7__.NgForOf, _angular_common__WEBPACK_IMPORTED_MODULE_7__.NgIf],
  styles: [".content[_ngcontent-%COMP%] {\n  padding: 0px 0px;\n}\n\n.header_row[_ngcontent-%COMP%] {\n  height: 55px;\n  margin-left: 0;\n}\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIndlYnBhY2s6Ly8uL3NyYy9hcHAvY29tcG9uZW50cy9sZWFndWVzL2dldC9sZWFndWUuZ2V0LmNvbXBvbmVudC5zY3NzIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQUFBO0VBQ0ksZ0JBQUE7QUFDSjs7QUFFQTtFQUNJLFlBQUE7RUFDQSxjQUFBO0FBQ0oiLCJzb3VyY2VzQ29udGVudCI6WyIuY29udGVudCB7XHJcbiAgICBwYWRkaW5nOiAwcHggMHB4XHJcbn1cclxuXHJcbi5oZWFkZXJfcm93eyBcclxuICAgIGhlaWdodDogNTVweDtcclxuICAgIG1hcmdpbi1sZWZ0OiAwXHJcbiAgfSJdLCJzb3VyY2VSb290IjoiIn0= */"]
}), _class);
LeagueGetComponent = (0,tslib__WEBPACK_IMPORTED_MODULE_8__.__decorate)([(0,_ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_5__.UntilDestroy)()], LeagueGetComponent);


/***/ }),

/***/ 481:
/*!*****************************************************!*\
  !*** ./src/app/components/leagues/league.module.ts ***!
  \*****************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "LeagueModule": () => (/* binding */ LeagueModule)
/* harmony export */ });
/* harmony import */ var _angular_common_http__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! @angular/common/http */ 8987);
/* harmony import */ var _angular_platform_browser__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! @angular/platform-browser */ 4497);
/* harmony import */ var _shared_shared_module__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../shared/shared.module */ 4065);
/* harmony import */ var _get_league_get_component__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./get/league.get.component */ 2406);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! @angular/core */ 2560);





class LeagueModule {}
LeagueModule.ɵfac = function LeagueModule_Factory(t) {
  return new (t || LeagueModule)();
};
LeagueModule.ɵmod = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵdefineNgModule"]({
  type: LeagueModule
});
LeagueModule.ɵinj = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵdefineInjector"]({
  imports: [_shared_shared_module__WEBPACK_IMPORTED_MODULE_0__.SharedModule, _angular_platform_browser__WEBPACK_IMPORTED_MODULE_3__.BrowserModule, _angular_common_http__WEBPACK_IMPORTED_MODULE_4__.HttpClientModule]
});
(function () {
  (typeof ngJitMode === "undefined" || ngJitMode) && _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵsetNgModuleScope"](LeagueModule, {
    declarations: [_get_league_get_component__WEBPACK_IMPORTED_MODULE_1__.LeagueGetComponent],
    imports: [_shared_shared_module__WEBPACK_IMPORTED_MODULE_0__.SharedModule, _angular_platform_browser__WEBPACK_IMPORTED_MODULE_3__.BrowserModule, _angular_common_http__WEBPACK_IMPORTED_MODULE_4__.HttpClientModule]
  });
})();

/***/ }),

/***/ 6214:
/*!*****************************************************!*\
  !*** ./src/app/components/leagues/league.routes.ts ***!
  \*****************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "leagueRoutes": () => (/* binding */ leagueRoutes)
/* harmony export */ });
/* harmony import */ var _get_league_get_component__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./get/league.get.component */ 2406);

const leagueRoutes = [{
  path: 'leagues/:slug',
  component: _get_league_get_component__WEBPACK_IMPORTED_MODULE_0__.LeagueGetComponent
}];

/***/ }),

/***/ 7070:
/*!***************************************************************!*\
  !*** ./src/app/components/leagues/services/league.service.ts ***!
  \***************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "LeagueService": () => (/* binding */ LeagueService)
/* harmony export */ });
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _angular_common_http__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @angular/common/http */ 8987);


class LeagueService {
  constructor(http) {
    this.http = http;
  }
  get(slug) {
    return this.http.get('/api/leagues/' + slug);
  }
}
LeagueService.ɵfac = function LeagueService_Factory(t) {
  return new (t || LeagueService)(_angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵinject"](_angular_common_http__WEBPACK_IMPORTED_MODULE_1__.HttpClient));
};
LeagueService.ɵprov = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵdefineInjectable"]({
  token: LeagueService,
  factory: LeagueService.ɵfac,
  providedIn: 'root'
});

/***/ }),

/***/ 5910:
/*!********************************************************************!*\
  !*** ./src/app/components/shared/left-menu/left.menu.component.ts ***!
  \********************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "LeftMenuComponent": () => (/* binding */ LeftMenuComponent)
/* harmony export */ });
/* harmony import */ var tslib__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! tslib */ 4929);
/* harmony import */ var _ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! @ngneat/until-destroy */ 2777);
/* harmony import */ var rxjs__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! rxjs */ 228);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./services/left.menu.service */ 3456);
/* harmony import */ var _angular_common__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! @angular/common */ 4666);
var _class;






function LeftMenuComponent_div_2_li_2_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵelementStart"](0, "li")(1, "a", 6);
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵelement"](2, "i");
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵtext"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵelementEnd"]()();
  }
  if (rf & 2) {
    const menuItem_r3 = ctx.$implicit;
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵpropertyInterpolate"]("href", menuItem_r3.url, _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵclassMapInterpolate1"]("fa ", menuItem_r3.icon, "");
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵtextInterpolate"](menuItem_r3.title);
  }
}
function LeftMenuComponent_div_2_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵelementStart"](0, "div", 3)(1, "ul", 4);
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵtemplate"](2, LeftMenuComponent_div_2_li_2_Template, 4, 5, "li", 5);
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵelementEnd"]()();
  }
  if (rf & 2) {
    const section_r1 = ctx.$implicit;
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵproperty"]("ngForOf", section_r1.items);
  }
}
let LeftMenuComponent = (_class = class LeftMenuComponent {
  constructor(leftMenuService, changeDetectorRef) {
    this.leftMenuService = leftMenuService;
    this.changeDetectorRef = changeDetectorRef;
    this.menuSections$ = new rxjs__WEBPACK_IMPORTED_MODULE_2__.Subject();
  }
  ngOnInit() {
    this.leftMenuService.items$.pipe((0,_ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_3__.untilDestroyed)(this)).subscribe(menuSections => {
      this.menuSections$.next(menuSections);
      this.changeDetectorRef.markForCheck();
    });
  }
}, _class.ɵfac = function LeftMenuComponent_Factory(t) {
  return new (t || _class)(_angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵdirectiveInject"](_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__.LeftMenuService), _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵdirectiveInject"](_angular_core__WEBPACK_IMPORTED_MODULE_1__.ChangeDetectorRef));
}, _class.ɵcmp = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵdefineComponent"]({
  type: _class,
  selectors: [["left-menu"]],
  decls: 4,
  vars: 3,
  consts: [[1, "left_col", "scroll-view"], ["id", "sidebar-menu", 1, "main_menu_side", "hidden-print", "main_menu"], ["class", "menu_section", 4, "ngFor", "ngForOf"], [1, "menu_section"], [1, "nav"], [4, "ngFor", "ngForOf"], [3, "href"]],
  template: function LeftMenuComponent_Template(rf, ctx) {
    if (rf & 1) {
      _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵelementStart"](0, "div", 0)(1, "div", 1);
      _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵtemplate"](2, LeftMenuComponent_div_2_Template, 3, 1, "div", 2);
      _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵpipe"](3, "async");
      _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵelementEnd"]()();
    }
    if (rf & 2) {
      _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵadvance"](2);
      _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵproperty"]("ngForOf", _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵpipeBind1"](3, 1, ctx.menuSections$));
    }
  },
  dependencies: [_angular_common__WEBPACK_IMPORTED_MODULE_4__.NgForOf, _angular_common__WEBPACK_IMPORTED_MODULE_4__.AsyncPipe],
  styles: ["\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IiIsInNvdXJjZVJvb3QiOiIifQ== */"],
  changeDetection: 0
}), _class);
LeftMenuComponent = (0,tslib__WEBPACK_IMPORTED_MODULE_5__.__decorate)([(0,_ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_3__.UntilDestroy)()], LeftMenuComponent);


/***/ }),

/***/ 3456:
/*!***************************************************************************!*\
  !*** ./src/app/components/shared/left-menu/services/left.menu.service.ts ***!
  \***************************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "LeftMenuService": () => (/* binding */ LeftMenuService),
/* harmony export */   "MenuItem": () => (/* binding */ MenuItem),
/* harmony export */   "MenuSection": () => (/* binding */ MenuSection)
/* harmony export */ });
/* harmony import */ var rxjs__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! rxjs */ 6317);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @angular/core */ 2560);


class LeftMenuService {
  constructor() {
    this.items$ = new rxjs__WEBPACK_IMPORTED_MODULE_0__.BehaviorSubject([]);
  }
  setMenu(items) {
    this.items$.next(items);
  }
}
LeftMenuService.ɵfac = function LeftMenuService_Factory(t) {
  return new (t || LeftMenuService)();
};
LeftMenuService.ɵprov = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵdefineInjectable"]({
  token: LeftMenuService,
  factory: LeftMenuService.ɵfac,
  providedIn: 'root'
});
class MenuSection {}
class MenuItem {}

/***/ }),

/***/ 4065:
/*!****************************************************!*\
  !*** ./src/app/components/shared/shared.module.ts ***!
  \****************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "SharedModule": () => (/* binding */ SharedModule)
/* harmony export */ });
/* harmony import */ var _angular_platform_browser__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! @angular/platform-browser */ 4497);
/* harmony import */ var _left_menu_left_menu_component__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./left-menu/left.menu.component */ 5910);
/* harmony import */ var _left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./left-menu/services/left.menu.service */ 3456);
/* harmony import */ var _top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./top-header/content/top.header.content.component */ 3278);
/* harmony import */ var _top_header_top_header_component__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./top-header/top.header.component */ 2400);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! @angular/core */ 2560);






class SharedModule {}
SharedModule.ɵfac = function SharedModule_Factory(t) {
  return new (t || SharedModule)();
};
SharedModule.ɵmod = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdefineNgModule"]({
  type: SharedModule
});
SharedModule.ɵinj = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdefineInjector"]({
  providers: [_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_1__.LeftMenuService],
  imports: [_angular_platform_browser__WEBPACK_IMPORTED_MODULE_5__.BrowserModule]
});
(function () {
  (typeof ngJitMode === "undefined" || ngJitMode) && _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsetNgModuleScope"](SharedModule, {
    declarations: [_left_menu_left_menu_component__WEBPACK_IMPORTED_MODULE_0__.LeftMenuComponent, _top_header_top_header_component__WEBPACK_IMPORTED_MODULE_3__.TopHeaderComponent, _top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_2__.TopHeaderContentComponent],
    imports: [_angular_platform_browser__WEBPACK_IMPORTED_MODULE_5__.BrowserModule],
    exports: [_left_menu_left_menu_component__WEBPACK_IMPORTED_MODULE_0__.LeftMenuComponent, _top_header_top_header_component__WEBPACK_IMPORTED_MODULE_3__.TopHeaderComponent, _top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_2__.TopHeaderContentComponent]
  });
})();

/***/ }),

/***/ 3278:
/*!**************************************************************************************!*\
  !*** ./src/app/components/shared/top-header/content/top.header.content.component.ts ***!
  \**************************************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "TopHeaderContentComponent": () => (/* binding */ TopHeaderContentComponent)
/* harmony export */ });
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _services_top_header_service__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../services/top.header.service */ 6908);


class TopHeaderContentComponent {
  constructor(topHeaderService, elem) {
    this.topHeaderService = topHeaderService;
    this.elem = elem;
  }
  ngAfterViewInit() {
    this.topHeaderService.setContent(this.elem.nativeElement.innerHTML);
    this.elem.nativeElement.innerHTML = '';
  }
}
TopHeaderContentComponent.ɵfac = function TopHeaderContentComponent_Factory(t) {
  return new (t || TopHeaderContentComponent)(_angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵdirectiveInject"](_services_top_header_service__WEBPACK_IMPORTED_MODULE_0__.TopHeaderService), _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵdirectiveInject"](_angular_core__WEBPACK_IMPORTED_MODULE_1__.ElementRef));
};
TopHeaderContentComponent.ɵdir = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵdefineDirective"]({
  type: TopHeaderContentComponent,
  selectors: [["", "topHeader", ""]]
});

/***/ }),

/***/ 6908:
/*!*****************************************************************************!*\
  !*** ./src/app/components/shared/top-header/services/top.header.service.ts ***!
  \*****************************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "TopHeaderService": () => (/* binding */ TopHeaderService)
/* harmony export */ });
/* harmony import */ var rxjs__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! rxjs */ 6317);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @angular/core */ 2560);


class TopHeaderService {
  constructor() {
    this.content$ = new rxjs__WEBPACK_IMPORTED_MODULE_0__.BehaviorSubject('');
  }
  setContent(content) {
    this.content$.next(content);
  }
}
TopHeaderService.ɵfac = function TopHeaderService_Factory(t) {
  return new (t || TopHeaderService)();
};
TopHeaderService.ɵprov = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵdefineInjectable"]({
  token: TopHeaderService,
  factory: TopHeaderService.ɵfac,
  providedIn: 'root'
});

/***/ }),

/***/ 2400:
/*!**********************************************************************!*\
  !*** ./src/app/components/shared/top-header/top.header.component.ts ***!
  \**********************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "TopHeaderComponent": () => (/* binding */ TopHeaderComponent)
/* harmony export */ });
/* harmony import */ var tslib__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! tslib */ 4929);
/* harmony import */ var _ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @ngneat/until-destroy */ 2777);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _services_top_header_service__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./services/top.header.service */ 6908);
var _class;




let TopHeaderComponent = (_class = class TopHeaderComponent {
  constructor(topHeaderService, changeDetectorRef) {
    this.topHeaderService = topHeaderService;
    this.changeDetectorRef = changeDetectorRef;
    this.content = '';
  }
  ngOnInit() {
    this.topHeaderService.content$.pipe((0,_ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_1__.untilDestroyed)(this)).subscribe(content => {
      this.content = content;
      this.changeDetectorRef.markForCheck();
    });
  }
}, _class.ɵfac = function TopHeaderComponent_Factory(t) {
  return new (t || _class)(_angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵdirectiveInject"](_services_top_header_service__WEBPACK_IMPORTED_MODULE_0__.TopHeaderService), _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵdirectiveInject"](_angular_core__WEBPACK_IMPORTED_MODULE_2__.ChangeDetectorRef));
}, _class.ɵcmp = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵdefineComponent"]({
  type: _class,
  selectors: [["top-header"]],
  decls: 1,
  vars: 1,
  consts: [[1, "top-header-container", 3, "innerHTML"]],
  template: function TopHeaderComponent_Template(rf, ctx) {
    if (rf & 1) {
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelement"](0, "div", 0);
    }
    if (rf & 2) {
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵproperty"]("innerHTML", ctx.content, _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵsanitizeHtml"]);
    }
  },
  styles: ["\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IiIsInNvdXJjZVJvb3QiOiIifQ== */"],
  changeDetection: 0
}), _class);
TopHeaderComponent = (0,tslib__WEBPACK_IMPORTED_MODULE_3__.__decorate)([(0,_ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_1__.UntilDestroy)()], TopHeaderComponent);


/***/ }),

/***/ 6051:
/*!************************************************************!*\
  !*** ./src/app/components/teams/get/team.get.component.ts ***!
  \************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "TeamGetComponent": () => (/* binding */ TeamGetComponent)
/* harmony export */ });
/* harmony import */ var tslib__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__(/*! tslib */ 4929);
/* harmony import */ var _ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! @ngneat/until-destroy */ 2777);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _shared_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../../shared/left-menu/services/left.menu.service */ 3456);
/* harmony import */ var _services_team_service__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../services/team.service */ 6961);
/* harmony import */ var _angular_router__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! @angular/router */ 124);
/* harmony import */ var src_app_shared_services_title_service__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! src/app/shared/services/title.service */ 7153);
/* harmony import */ var _shared_top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ../../shared/top-header/content/top.header.content.component */ 3278);
/* harmony import */ var _angular_common__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! @angular/common */ 4666);
var _class;









function TeamGetComponent_div_0_tr_57_div_2_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](0, "div", 32);
  }
}
function TeamGetComponent_div_0_tr_57_label_14_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](0, "label");
  }
}
function TeamGetComponent_div_0_tr_57_label_15_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](0, "label", 33);
  }
}
function TeamGetComponent_div_0_tr_57_label_18_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](0, "label");
  }
}
function TeamGetComponent_div_0_tr_57_label_19_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](0, "label", 33);
  }
}
function TeamGetComponent_div_0_tr_57_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "tr")(1, "td", 17);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](2, TeamGetComponent_div_0_tr_57_div_2_Template, 1, 0, "div", 20);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](3, "td", 21);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](4, "div", 22);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](5, "a", 23);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](6);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](7, "td", 24);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](8);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](9, "td", 17)(10, "a", 6);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](11, "div", 25);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](12, "td", 26)(13, "fieldset", 27);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](14, TeamGetComponent_div_0_tr_57_label_14_Template, 1, 0, "label", 18);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](15, TeamGetComponent_div_0_tr_57_label_15_Template, 1, 0, "label", 28);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](16, "td", 26)(17, "fieldset", 27);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](18, TeamGetComponent_div_0_tr_57_label_18_Template, 1, 0, "label", 18);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](19, TeamGetComponent_div_0_tr_57_label_19_Template, 1, 0, "label", 28);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](20, "td", 26);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](21);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](22, "td", 29);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](23);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](24, "td", 30);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](25, " 25 ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](26, "td", 31);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](27);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
  }
  if (rf & 2) {
    const player_r2 = ctx.$implicit;
    const ctx_r1 = _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵnextContext"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngIf", player_r2.injured);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpropertyInterpolate2"]("href", "/teams/", ctx_r1.team.slug, "/players/", player_r2.id, "", _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate2"](" ", player_r2.last_name, "\u00A0", player_r2.first_name, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", player_r2.position, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpropertyInterpolate1"]("href", "/countries/", player_r2.country_id, "", _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵclassMapInterpolate1"]("flag flag-", player_r2.country_code, "");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpropertyInterpolate"]("title", player_r2.country_name);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngForOf", ctx_r1.createRange(5 - player_r2.current_ability));
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngForOf", ctx_r1.createRange(player_r2.current_ability));
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngForOf", ctx_r1.createRange(5 - player_r2.potential_ability));
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngForOf", ctx_r1.createRange(player_r2.potential_ability));
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", player_r2.behaviour, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", player_r2.conditions, "% ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](4);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", player_r2.value, " ");
  }
}
function TeamGetComponent_div_0_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "div")(1, "div", 1)(2, "div", 2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](3, "div", 3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](4, "div", 2)(5, "div", 4)(6, "div", 5);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](7);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](8, "div")(9, "a", 6);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](11, "div", 7)(12, "div", 8)(13, "ul", 9)(14, "li")(15, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](16, "Players");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](17, "li")(18, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](19, "International");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](20, "li")(21, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](22, "Players on Loan");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](23, "li")(24, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](25, "Extended Leave");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](26, "li")(27, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](28, "Team Meeting");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](29, "ul", 11)(30, "li", 12)(31, "a", 13);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](32, "i", 14);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](33, "br");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](34, "div", 15)(35, "table", 16)(36, "tr")(37, "th", 17);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](38, " Inf ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](39, "th");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](40, " Name ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](41, "th");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](42, " Position ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](43, "th");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](44, " Nat ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](45, "th");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](46, " Ability ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](47, "th");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](48, " Potential ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](49, "th");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](50, " Morale ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](51, "th");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](52, " Conditions ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](53, "th");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](54, " Played ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](55, "th");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](56, " Value ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](57, TeamGetComponent_div_0_tr_57_Template, 28, 18, "tr", 18);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](58, "div", 19);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
  }
  if (rf & 2) {
    const ctx_r0 = _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵnextContext"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](7);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate"](ctx_r0.team.name);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpropertyInterpolate1"]("href", "/leagues/", ctx_r0.team.league_slug, "", _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", ctx_r0.team.league_name, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](47);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngForOf", ctx_r0.team.players);
  }
}
let TeamGetComponent = (_class = class TeamGetComponent {
  constructor(leftMenuService, service, route, titleService) {
    this.leftMenuService = leftMenuService;
    this.service = service;
    this.route = route;
    this.titleService = titleService;
    this.team = null;
  }
  ngOnInit() {
    this.route.params.subscribe(params => {
      this.service.get(params["slug"]).pipe((0,_ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_5__.untilDestroyed)(this)).subscribe(teamData => {
        this.team = teamData;
        this.initLeftMenu(teamData);
        this.titleService.setTitle(this.team.name);
      });
    });
  }
  initLeftMenu(team) {
    this.leftMenuService.setMenu([{
      items: [{
        url: '/',
        title: 'Home',
        icon: 'fa-home'
      }]
    }, {
      items: [{
        url: '/inbox',
        title: 'Inbox',
        icon: 'fa-inbox'
      }]
    }, {
      items: team.neighbor_teams.map(subteam => ({
        url: `/teams/${subteam.slug}`,
        title: subteam.name,
        icon: 'fa-user-friends'
      }))
    }, {
      items: [{
        url: `/teams/${team.slug}/schedule`,
        title: 'Schedule',
        icon: 'fa-inbox'
      }]
    }, {
      items: [{
        url: '/calendar',
        title: 'Calendar',
        icon: 'fa-calendar-alt'
      }]
    }]);
  }
  createRange(number) {
    return new Array(number).fill(0).map((n, index) => index + 1);
  }
}, _class.ɵfac = function TeamGetComponent_Factory(t) {
  return new (t || _class)(_angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](_shared_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__.LeftMenuService), _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](_services_team_service__WEBPACK_IMPORTED_MODULE_1__.TeamService), _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](_angular_router__WEBPACK_IMPORTED_MODULE_6__.ActivatedRoute), _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](src_app_shared_services_title_service__WEBPACK_IMPORTED_MODULE_2__.TitleService));
}, _class.ɵcmp = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdefineComponent"]({
  type: _class,
  selectors: [["ng-component"]],
  decls: 1,
  vars: 1,
  consts: [[4, "ngIf"], ["topHeader", ""], [1, "header-block"], [1, "club-logo", "club-default-logo", "header-player-logo"], [1, "header-title-block"], [1, "page-title"], [1, "sub-title-link", 3, "href"], [1, "header_row", 2, "height", "55px", "margin-left", "0"], [1, "header_bottom"], [1, "nav", "navbar-left"], ["href", "/#"], [1, "nav", "navbar-right", 2, "margin-right", "0"], [1, "item-user"], ["href", ""], [1, "fas", "fa-user-friends"], [1, "content"], [1, "squad-table"], [2, "width", "50px"], [4, "ngFor", "ngForOf"], [2, "clear", "both"], ["class", "status-badge injured", 4, "ngIf"], [2, "min-width", "200px"], [1, "player-info-logo"], [3, "href"], [2, "width", "80px"], [3, "title"], [2, "width", "150px"], [1, "rating"], ["class", "checked", 4, "ngFor", "ngForOf"], [2, "width", "100px", "text-align", "center"], [2, "width", "70px"], [2, "width", "170px"], [1, "status-badge", "injured"], [1, "checked"]],
  template: function TeamGetComponent_Template(rf, ctx) {
    if (rf & 1) {
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](0, TeamGetComponent_div_0_Template, 59, 4, "div", 0);
    }
    if (rf & 2) {
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngIf", ctx.team);
    }
  },
  dependencies: [_shared_top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_3__.TopHeaderContentComponent, _angular_common__WEBPACK_IMPORTED_MODULE_7__.NgForOf, _angular_common__WEBPACK_IMPORTED_MODULE_7__.NgIf],
  styles: ["\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IiIsInNvdXJjZVJvb3QiOiIifQ== */"]
}), _class);
TeamGetComponent = (0,tslib__WEBPACK_IMPORTED_MODULE_8__.__decorate)([(0,_ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_5__.UntilDestroy)()], TeamGetComponent);


/***/ }),

/***/ 7646:
/*!**********************************************************************!*\
  !*** ./src/app/components/teams/schedule/team.schedule.component.ts ***!
  \**********************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "TeamScheduleComponent": () => (/* binding */ TeamScheduleComponent)
/* harmony export */ });
/* harmony import */ var tslib__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__(/*! tslib */ 4929);
/* harmony import */ var _ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! @ngneat/until-destroy */ 2777);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _shared_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../../shared/left-menu/services/left.menu.service */ 3456);
/* harmony import */ var _services_team_service__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../services/team.service */ 6961);
/* harmony import */ var _angular_router__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! @angular/router */ 124);
/* harmony import */ var src_app_shared_services_title_service__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! src/app/shared/services/title.service */ 7153);
/* harmony import */ var _shared_top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ../../shared/top-header/content/top.header.content.component */ 3278);
/* harmony import */ var _angular_common__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! @angular/common */ 4666);
var _class;









function TeamScheduleComponent_div_0_tr_49_span_9_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "span");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](1, "H");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
  }
}
function TeamScheduleComponent_div_0_tr_49_span_10_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "span");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](1, "A");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
  }
}
function TeamScheduleComponent_div_0_tr_49_div_12_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "div");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
  }
  if (rf & 2) {
    const item_r2 = _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵnextContext"]().$implicit;
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate2"](" ", item_r2.result.home_goals, " - ", item_r2.result.away_goals, " ");
  }
}
function TeamScheduleComponent_div_0_tr_49_div_13_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "div");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](1, " - ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
  }
}
function TeamScheduleComponent_div_0_tr_49_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "tr")(1, "td");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](3, "td");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](4);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](5, "td")(6, "a", 23);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](7);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](8, "td", 24);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](9, TeamScheduleComponent_div_0_tr_49_span_9_Template, 2, 0, "span", 0);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](10, TeamScheduleComponent_div_0_tr_49_span_10_Template, 2, 0, "span", 0);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](11, "td", 24);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](12, TeamScheduleComponent_div_0_tr_49_div_12_Template, 2, 2, "div", 0);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](13, TeamScheduleComponent_div_0_tr_49_div_13_Template, 2, 0, "div", 0);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](14, "td")(15, "a", 23);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](16);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()();
  }
  if (rf & 2) {
    const item_r2 = ctx.$implicit;
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", item_r2.date, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", item_r2.time, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpropertyInterpolate1"]("href", "/teams/", item_r2.opponent_slug, "", _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate"](item_r2.opponent_name);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngIf", item_r2.is_home);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngIf", !item_r2.is_home);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngIf", item_r2.result);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngIf", !item_r2.result);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpropertyInterpolate1"]("href", "/leagues/", item_r2.competition_id, "", _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate"](item_r2.competition_name);
  }
}
function TeamScheduleComponent_div_0_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](0, "div")(1, "div", 1)(2, "div", 2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](3, "div", 3);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](4, "div", 2)(5, "div", 4)(6, "div", 5);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](7);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](8, "div")(9, "a", 6);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](11, "div", 7)(12, "div", 8)(13, "ul", 9)(14, "li")(15, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](16, "Players");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](17, "li")(18, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](19, "International");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](20, "li")(21, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](22, "Players on Loan");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](23, "li")(24, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](25, "Extended Leave");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](26, "li")(27, "a", 10);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](28, "Team Meeting");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](29, "ul", 11)(30, "li", 12)(31, "a", 13);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](32, "i", 14);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](33, "br");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](34, "div", 15)(35, "table", 16)(36, "tr")(37, "th", 17);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](38, " Date ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](39, "th", 18);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](40, " Time ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](41, "th");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](42, " Oppisition ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](43, "th", 19);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](44, " Venue ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](45, "th", 20);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](46, " Result ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementStart"](47, "th");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtext"](48, " Competition ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](49, TeamScheduleComponent_div_0_tr_49_Template, 17, 10, "tr", 21);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelement"](50, "div", 22);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵelementEnd"]()();
  }
  if (rf & 2) {
    const ctx_r0 = _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵnextContext"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](7);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate"](ctx_r0.teamSchedule.team_name);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵpropertyInterpolate1"]("href", "/leagues/", ctx_r0.teamSchedule.league_slug, "", _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtextInterpolate1"](" ", ctx_r0.teamSchedule.league_name, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵadvance"](39);
    _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngForOf", ctx_r0.teamSchedule.items);
  }
}
let TeamScheduleComponent = (_class = class TeamScheduleComponent {
  constructor(leftMenuService, service, route, titleService) {
    this.leftMenuService = leftMenuService;
    this.service = service;
    this.route = route;
    this.titleService = titleService;
    this.teamSchedule = null;
  }
  ngOnInit() {
    this.route.params.subscribe(params => {
      this.service.getSchedule(params["slug"]).pipe((0,_ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_5__.untilDestroyed)(this)).subscribe(teamScheduleData => {
        this.teamSchedule = teamScheduleData;
        this.initLeftMenu(teamScheduleData);
        this.titleService.setTitle(teamScheduleData.team_name + ' Schedule');
      });
    });
  }
  initLeftMenu(teamSchedule) {
    this.leftMenuService.setMenu([{
      items: [{
        url: '/',
        title: 'Home',
        icon: 'fa-home'
      }]
    }, {
      items: [{
        url: '/inbox',
        title: 'Inbox',
        icon: 'fa-inbox'
      }]
    }, {
      items: [{
        url: `/teams/${teamSchedule.team_slug}`,
        title: teamSchedule.team_name,
        icon: 'fa-user-friends'
      }]
    },
    // { items: [{ url: `/teams/${team.slug}/schedule`, title: 'Schedule', icon: 'fa-inbox' }] },
    {
      items: [{
        url: '/calendar',
        title: 'Calendar',
        icon: 'fa-calendar-alt'
      }]
    }]);
  }
  createRange(number) {
    return new Array(number).fill(0).map((n, index) => index + 1);
  }
}, _class.ɵfac = function TeamScheduleComponent_Factory(t) {
  return new (t || _class)(_angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](_shared_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__.LeftMenuService), _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](_services_team_service__WEBPACK_IMPORTED_MODULE_1__.TeamService), _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](_angular_router__WEBPACK_IMPORTED_MODULE_6__.ActivatedRoute), _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdirectiveInject"](src_app_shared_services_title_service__WEBPACK_IMPORTED_MODULE_2__.TitleService));
}, _class.ɵcmp = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdefineComponent"]({
  type: _class,
  selectors: [["ng-component"]],
  decls: 1,
  vars: 1,
  consts: [[4, "ngIf"], ["topHeader", ""], [1, "header-block"], [1, "club-logo", "club-default-logo", "header-player-logo"], [1, "header-title-block"], [1, "page-title"], [1, "sub-title-link", 3, "href"], [1, "header_row", 2, "height", "55px", "margin-left", "0"], [1, "header_bottom"], [1, "nav", "navbar-left"], ["href", "/#"], [1, "nav", "navbar-right", 2, "margin-right", "0"], [1, "item-user"], ["href", ""], [1, "fas", "fa-user-friends"], [1, "content"], [1, "squad-table"], [2, "width", "100px"], [2, "width", "80px"], [2, "width", "100px", "text-align", "center"], [2, "width", "150px", "text-align", "center"], [4, "ngFor", "ngForOf"], [2, "clear", "both"], [3, "href"], [2, "text-align", "center"]],
  template: function TeamScheduleComponent_Template(rf, ctx) {
    if (rf & 1) {
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵtemplate"](0, TeamScheduleComponent_div_0_Template, 51, 4, "div", 0);
    }
    if (rf & 2) {
      _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵproperty"]("ngIf", ctx.teamSchedule);
    }
  },
  dependencies: [_shared_top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_3__.TopHeaderContentComponent, _angular_common__WEBPACK_IMPORTED_MODULE_7__.NgForOf, _angular_common__WEBPACK_IMPORTED_MODULE_7__.NgIf],
  styles: ["\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IiIsInNvdXJjZVJvb3QiOiIifQ== */"]
}), _class);
TeamScheduleComponent = (0,tslib__WEBPACK_IMPORTED_MODULE_8__.__decorate)([(0,_ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_5__.UntilDestroy)()], TeamScheduleComponent);


/***/ }),

/***/ 6961:
/*!***********************************************************!*\
  !*** ./src/app/components/teams/services/team.service.ts ***!
  \***********************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "TeamService": () => (/* binding */ TeamService)
/* harmony export */ });
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _angular_common_http__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @angular/common/http */ 8987);


class TeamService {
  constructor(http) {
    this.http = http;
  }
  get(slug) {
    return this.http.get('/api/teams/' + slug);
  }
  getSchedule(slug) {
    return this.http.get('/api/teams/' + slug + '/schedule');
  }
}
TeamService.ɵfac = function TeamService_Factory(t) {
  return new (t || TeamService)(_angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵinject"](_angular_common_http__WEBPACK_IMPORTED_MODULE_1__.HttpClient));
};
TeamService.ɵprov = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵdefineInjectable"]({
  token: TeamService,
  factory: TeamService.ɵfac,
  providedIn: 'root'
});

/***/ }),

/***/ 1809:
/*!*************************************************!*\
  !*** ./src/app/components/teams/team.module.ts ***!
  \*************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "TeamModule": () => (/* binding */ TeamModule)
/* harmony export */ });
/* harmony import */ var _angular_common_http__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! @angular/common/http */ 8987);
/* harmony import */ var _angular_platform_browser__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! @angular/platform-browser */ 4497);
/* harmony import */ var _shared_shared_module__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../shared/shared.module */ 4065);
/* harmony import */ var _get_team_get_component__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./get/team.get.component */ 6051);
/* harmony import */ var _schedule_team_schedule_component__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./schedule/team.schedule.component */ 7646);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! @angular/core */ 2560);






class TeamModule {}
TeamModule.ɵfac = function TeamModule_Factory(t) {
  return new (t || TeamModule)();
};
TeamModule.ɵmod = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵdefineNgModule"]({
  type: TeamModule
});
TeamModule.ɵinj = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵdefineInjector"]({
  imports: [_shared_shared_module__WEBPACK_IMPORTED_MODULE_0__.SharedModule, _angular_platform_browser__WEBPACK_IMPORTED_MODULE_4__.BrowserModule, _angular_common_http__WEBPACK_IMPORTED_MODULE_5__.HttpClientModule]
});
(function () {
  (typeof ngJitMode === "undefined" || ngJitMode) && _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵsetNgModuleScope"](TeamModule, {
    declarations: [_get_team_get_component__WEBPACK_IMPORTED_MODULE_1__.TeamGetComponent, _schedule_team_schedule_component__WEBPACK_IMPORTED_MODULE_2__.TeamScheduleComponent],
    imports: [_shared_shared_module__WEBPACK_IMPORTED_MODULE_0__.SharedModule, _angular_platform_browser__WEBPACK_IMPORTED_MODULE_4__.BrowserModule, _angular_common_http__WEBPACK_IMPORTED_MODULE_5__.HttpClientModule]
  });
})();

/***/ }),

/***/ 6406:
/*!*************************************************!*\
  !*** ./src/app/components/teams/team.routes.ts ***!
  \*************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "teamRoutes": () => (/* binding */ teamRoutes)
/* harmony export */ });
/* harmony import */ var _get_team_get_component__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./get/team.get.component */ 6051);
/* harmony import */ var _schedule_team_schedule_component__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./schedule/team.schedule.component */ 7646);


const teamRoutes = [{
  path: 'teams/:slug',
  component: _get_team_get_component__WEBPACK_IMPORTED_MODULE_0__.TeamGetComponent
}, {
  path: 'teams/:slug/schedule',
  component: _schedule_team_schedule_component__WEBPACK_IMPORTED_MODULE_1__.TeamScheduleComponent
}];

/***/ }),

/***/ 7153:
/*!**************************************************!*\
  !*** ./src/app/shared/services/title.service.ts ***!
  \**************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "TitleService": () => (/* binding */ TitleService)
/* harmony export */ });
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _angular_platform_browser__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @angular/platform-browser */ 4497);


const DEFAULT_TITLE = "Football Simulator";
class TitleService {
  constructor(title) {
    this.title = title;
  }
  setTitle(title) {
    this.title.setTitle(title + ' - ' + DEFAULT_TITLE);
  }
}
TitleService.ɵfac = function TitleService_Factory(t) {
  return new (t || TitleService)(_angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵinject"](_angular_platform_browser__WEBPACK_IMPORTED_MODULE_1__.Title));
};
TitleService.ɵprov = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_0__["ɵɵdefineInjectable"]({
  token: TitleService,
  factory: TitleService.ɵfac,
  providedIn: 'root'
});

/***/ }),

/***/ 4431:
/*!*********************!*\
  !*** ./src/main.ts ***!
  \*********************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony import */ var _angular_platform_browser__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @angular/platform-browser */ 4497);
/* harmony import */ var _app_app_module__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./app/app.module */ 6747);


_angular_platform_browser__WEBPACK_IMPORTED_MODULE_1__.platformBrowser().bootstrapModule(_app_app_module__WEBPACK_IMPORTED_MODULE_0__.AppModule).catch(err => console.error(err));

/***/ })

},
/******/ __webpack_require__ => { // webpackRuntimeModules
/******/ var __webpack_exec__ = (moduleId) => (__webpack_require__(__webpack_require__.s = moduleId))
/******/ __webpack_require__.O(0, ["vendor"], () => (__webpack_exec__(4431)));
/******/ var __webpack_exports__ = __webpack_require__.O();
/******/ }
]);
//# sourceMappingURL=main.js.map