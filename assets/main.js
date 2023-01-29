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
  decls: 46,
  vars: 0,
  consts: [[1, "main_container"], [1, "main_content"], [1, "col-md-3", "left_col"], [1, "header_row"], [1, "header_top"], [1, "col-md-6", 2, "padding-left", "40px"], [1, "col-md-6"], [1, "nav", "navbar-right"], [1, "item-msg", 2, "cursor", "pointer"], ["id", "process-game-btn", 2, "text-transform", "uppercase", "text-align", "center"], [2, "margin-top", "5px", "width", "100px", "padding-top", "20px"], ["id", "process-game-btn-spinner", 1, "spinner", 2, "display", "none"], [1, "item-date"], ["id", "date-up", 1, "date-title"], ["id", "date-down", 1, "date-body"], [1, "btn-group", "btn-drop", "btn-globe"], ["data-toggle", "dropdown", "href", "/#", "aria-expanded", "false", 1, "btn", "dropdown-toggle"], [1, "fas", "fa-globe-americas"], [1, "dropdown-menu"], ["href", "/#"], ["href", "/#", 1, "link-search"], [1, "fas", "fa-search"], [2, "margin", "0", "padding", "0"], [1, "right_col"], [1, "content"], [2, "clear", "both"]],
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
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementStart"](40, "div", 23)(41, "div", 1)(42, "div", 24)(43, "div");
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelement"](44, "router-outlet");
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementEnd"]();
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelement"](45, "div", 25);
      _angular_core__WEBPACK_IMPORTED_MODULE_2__["ɵɵelementEnd"]()()()()()();
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
/* harmony import */ var _angular_platform_browser__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! @angular/platform-browser */ 4497);
/* harmony import */ var _app_routing_module__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./app.routing.module */ 3670);
/* harmony import */ var _app_component__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./app.component */ 5041);
/* harmony import */ var _components_countries_country_module__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./components/countries/country.module */ 2618);
/* harmony import */ var _components_shared_shared_module__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./components/shared/shared.module */ 4065);
/* harmony import */ var _angular_common__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! @angular/common */ 4666);
/* harmony import */ var _angular_common_http__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! @angular/common/http */ 8987);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! @angular/core */ 2560);








class AppModule {}
AppModule.ɵfac = function AppModule_Factory(t) {
  return new (t || AppModule)();
};
AppModule.ɵmod = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdefineNgModule"]({
  type: AppModule,
  bootstrap: [_app_component__WEBPACK_IMPORTED_MODULE_1__.AppComponent]
});
AppModule.ɵinj = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵdefineInjector"]({
  providers: [{
    provide: _angular_common__WEBPACK_IMPORTED_MODULE_5__.APP_BASE_HREF,
    useValue: '/'
  }],
  imports: [_components_shared_shared_module__WEBPACK_IMPORTED_MODULE_3__.SharedModule, _angular_platform_browser__WEBPACK_IMPORTED_MODULE_6__.BrowserModule, _app_routing_module__WEBPACK_IMPORTED_MODULE_0__.AppRoutingModule, _components_countries_country_module__WEBPACK_IMPORTED_MODULE_2__.CountryModule, _angular_common_http__WEBPACK_IMPORTED_MODULE_7__.HttpClientModule]
});
(function () {
  (typeof ngJitMode === "undefined" || ngJitMode) && _angular_core__WEBPACK_IMPORTED_MODULE_4__["ɵɵsetNgModuleScope"](AppModule, {
    declarations: [_app_component__WEBPACK_IMPORTED_MODULE_1__.AppComponent],
    imports: [_components_shared_shared_module__WEBPACK_IMPORTED_MODULE_3__.SharedModule, _angular_platform_browser__WEBPACK_IMPORTED_MODULE_6__.BrowserModule, _app_routing_module__WEBPACK_IMPORTED_MODULE_0__.AppRoutingModule, _components_countries_country_module__WEBPACK_IMPORTED_MODULE_2__.CountryModule, _angular_common_http__WEBPACK_IMPORTED_MODULE_7__.HttpClientModule]
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
/* harmony import */ var _angular_router__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! @angular/router */ 124);
/* harmony import */ var _components_countries_country_routes__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./components/countries/country.routes */ 2405);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @angular/core */ 2560);




const routes = [..._components_countries_country_routes__WEBPACK_IMPORTED_MODULE_0__.countryRoutes, {
  path: '**',
  redirectTo: '/countries',
  pathMatch: 'full'
}];
class AppRoutingModule {}
AppRoutingModule.ɵfac = function AppRoutingModule_Factory(t) {
  return new (t || AppRoutingModule)();
};
AppRoutingModule.ɵmod = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵdefineNgModule"]({
  type: AppRoutingModule
});
AppRoutingModule.ɵinj = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵdefineInjector"]({
  imports: [_angular_router__WEBPACK_IMPORTED_MODULE_2__.RouterModule.forRoot(routes), _angular_router__WEBPACK_IMPORTED_MODULE_2__.RouterModule]
});
(function () {
  (typeof ngJitMode === "undefined" || ngJitMode) && _angular_core__WEBPACK_IMPORTED_MODULE_1__["ɵɵsetNgModuleScope"](AppRoutingModule, {
    imports: [_angular_router__WEBPACK_IMPORTED_MODULE_2__.RouterModule],
    exports: [_angular_router__WEBPACK_IMPORTED_MODULE_2__.RouterModule]
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
/* harmony import */ var tslib__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! tslib */ 4929);
/* harmony import */ var _ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! @ngneat/until-destroy */ 2777);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _shared_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../../shared/left-menu/services/left.menu.service */ 3456);
/* harmony import */ var _services_country_service__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../services/country.service */ 6399);
/* harmony import */ var _angular_router__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! @angular/router */ 124);
/* harmony import */ var _shared_top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ../../shared/top-header/content/top.header.content.component */ 3278);
/* harmony import */ var _angular_common__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! @angular/common */ 4666);
var _class;








function CountryGetComponent_div_0_li_13_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementStart"](0, "li")(1, "a", 9);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtext"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementEnd"]()();
  }
  if (rf & 2) {
    const league_r2 = ctx.$implicit;
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵpropertyInterpolate1"]("href", "/leagues/", league_r2.slug, "", _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtextInterpolate1"](" ", league_r2.name, " ");
  }
}
function CountryGetComponent_div_0_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementStart"](0, "div")(1, "div", 1)(2, "div", 2);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelement"](3, "div");
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementStart"](4, "div", 2)(5, "div", 3)(6, "div", 4);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtext"](7);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementStart"](8, "div")(9, "a", 5);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtext"](10);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementEnd"]()()()()();
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementStart"](11, "div")(12, "ul", 6);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtemplate"](13, CountryGetComponent_div_0_li_13_Template, 3, 2, "li", 7);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelement"](14, "div", 8);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementEnd"]();
  }
  if (rf & 2) {
    const ctx_r0 = _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵnextContext"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵadvance"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵclassMapInterpolate1"]("flag flag-", ctx_r0.country.code, "");
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵadvance"](4);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtextInterpolate"](ctx_r0.country.name);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵadvance"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtextInterpolate1"](" ", ctx_r0.country.continent_name, " ");
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵadvance"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵproperty"]("ngForOf", ctx_r0.country.leagues);
  }
}
let CountryGetComponent = (_class = class CountryGetComponent {
  constructor(leftMenuService, service, route) {
    this.leftMenuService = leftMenuService;
    this.service = service;
    this.route = route;
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
      this.service.get(params["slug"]).pipe((0,_ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_4__.untilDestroyed)(this)).subscribe(countryData => {
        this.country = countryData;
      });
    });
  }
}, _class.ɵfac = function CountryGetComponent_Factory(t) {
  return new (t || _class)(_angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵdirectiveInject"](_shared_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__.LeftMenuService), _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵdirectiveInject"](_services_country_service__WEBPACK_IMPORTED_MODULE_1__.CountryService), _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵdirectiveInject"](_angular_router__WEBPACK_IMPORTED_MODULE_5__.ActivatedRoute));
}, _class.ɵcmp = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵdefineComponent"]({
  type: _class,
  selectors: [["ng-component"]],
  decls: 1,
  vars: 1,
  consts: [[4, "ngIf"], ["topHeader", ""], [1, "header-block"], [1, "header-title-block"], [1, "page-title"], ["href", "/countries", 1, "sub-title-link"], [1, "countries-list"], [4, "ngFor", "ngForOf"], [2, "clear", "both"], [3, "href"]],
  template: function CountryGetComponent_Template(rf, ctx) {
    if (rf & 1) {
      _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtemplate"](0, CountryGetComponent_div_0_Template, 15, 6, "div", 0);
    }
    if (rf & 2) {
      _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵproperty"]("ngIf", ctx.country);
    }
  },
  dependencies: [_shared_top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_2__.TopHeaderContentComponent, _angular_common__WEBPACK_IMPORTED_MODULE_6__.NgForOf, _angular_common__WEBPACK_IMPORTED_MODULE_6__.NgIf],
  styles: ["\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IiIsInNvdXJjZVJvb3QiOiIifQ== */"]
}), _class);
CountryGetComponent = (0,tslib__WEBPACK_IMPORTED_MODULE_7__.__decorate)([(0,_ngneat_until_destroy__WEBPACK_IMPORTED_MODULE_4__.UntilDestroy)()], CountryGetComponent);


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
/* harmony import */ var rxjs__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! rxjs */ 745);
/* harmony import */ var _angular_core__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! @angular/core */ 2560);
/* harmony import */ var _shared_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../../shared/left-menu/services/left.menu.service */ 3456);
/* harmony import */ var _services_country_service__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../services/country.service */ 6399);
/* harmony import */ var _shared_top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ../../shared/top-header/content/top.header.content.component */ 3278);
/* harmony import */ var _angular_common__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! @angular/common */ 4666);






function CountryListComponent_div_8_li_6_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementStart"](0, "li")(1, "a", 9);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelement"](2, "div");
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtext"](3);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementEnd"]()();
  }
  if (rf & 2) {
    const countryItem_r3 = ctx.$implicit;
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵpropertyInterpolate1"]("href", "/countries/", countryItem_r3.slug, "", _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵsanitizeUrl"]);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵclassMapInterpolate1"]("flag flag-", countryItem_r3.code, "");
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵadvance"](1);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtextInterpolate1"]("\u00A0\u00A0 ", countryItem_r3.name, " ");
  }
}
function CountryListComponent_div_8_Template(rf, ctx) {
  if (rf & 1) {
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementStart"](0, "div", 5)(1, "h2");
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtext"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementEnd"]();
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelement"](3, "hr");
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementStart"](4, "div")(5, "ul", 6);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtemplate"](6, CountryListComponent_div_8_li_6_Template, 4, 5, "li", 7);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementEnd"]()();
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelement"](7, "div", 8);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementEnd"]();
  }
  if (rf & 2) {
    const country_r1 = ctx.$implicit;
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵadvance"](2);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtextInterpolate2"]("", country_r1.name, " (", country_r1.countries.length, ")");
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵadvance"](4);
    _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵproperty"]("ngForOf", country_r1.countries);
  }
}
class CountryListComponent {
  constructor(leftMenuService, service) {
    this.leftMenuService = leftMenuService;
    this.countries$ = (0,rxjs__WEBPACK_IMPORTED_MODULE_4__.of)([]);
    this.countries$ = service.getList();
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
  return new (t || CountryListComponent)(_angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵdirectiveInject"](_shared_left_menu_services_left_menu_service__WEBPACK_IMPORTED_MODULE_0__.LeftMenuService), _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵdirectiveInject"](_services_country_service__WEBPACK_IMPORTED_MODULE_1__.CountryService));
};
CountryListComponent.ɵcmp = /*@__PURE__*/_angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵdefineComponent"]({
  type: CountryListComponent,
  selectors: [["country-list"]],
  decls: 10,
  vars: 3,
  consts: [["topHeader", ""], [2, "float", "left"], [1, "header-title-block"], [1, "page-title"], ["class", "country-box", 4, "ngFor", "ngForOf"], [1, "country-box"], [1, "countries-list"], [4, "ngFor", "ngForOf"], [2, "clear", "both"], [3, "href"]],
  template: function CountryListComponent_Template(rf, ctx) {
    if (rf & 1) {
      _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementStart"](0, "div", 0)(1, "div", 1)(2, "div", 2)(3, "div", 3);
      _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtext"](4, "Select country");
      _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementEnd"]();
      _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementStart"](5, "div");
      _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtext"](6, "Select any country to inspect it all");
      _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementEnd"]()()()();
      _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementStart"](7, "div");
      _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵtemplate"](8, CountryListComponent_div_8_Template, 8, 3, "div", 4);
      _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵpipe"](9, "async");
      _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵelementEnd"]();
    }
    if (rf & 2) {
      _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵadvance"](8);
      _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵproperty"]("ngForOf", _angular_core__WEBPACK_IMPORTED_MODULE_3__["ɵɵpipeBind1"](9, 1, ctx.countries$));
    }
  },
  dependencies: [_shared_top_header_content_top_header_content_component__WEBPACK_IMPORTED_MODULE_2__.TopHeaderContentComponent, _angular_common__WEBPACK_IMPORTED_MODULE_5__.NgForOf, _angular_common__WEBPACK_IMPORTED_MODULE_5__.AsyncPipe],
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