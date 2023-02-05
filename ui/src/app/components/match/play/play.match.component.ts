import {AfterViewInit, Component, ElementRef, Input, NgZone} from '@angular/core';
import * as PIXI from 'pixi.js';
import {Sprite} from '@pixi/sprite';
import {ActivatedRoute} from "@angular/router";
import {Graphics} from "pixi.js";
import {MatchDataService} from "../services/match.data.service";
import {POLE_COORDS} from "./constants";

@Component({
  selector: 'play-match',
  template: ''
})
export class MatchPlayComponent implements AfterViewInit {
  @Input()
  leagueSlug: string;
  @Input()
  matchId: string;

  constructor(public elRef: ElementRef,
              private matchDataService: MatchDataService,
              private zone: NgZone,
              private router: ActivatedRoute) {

    this.leagueSlug = router.snapshot.params["league_slug"];
    this.matchId = router.snapshot.params["match_id"];

    console.log(PIXI.VERSION);
  }

  public ngAfterViewInit(): void {
    this.matchDataService.init(this.leagueSlug, this.matchId).subscribe(_ => {
      this.initGraphics();
    });
  }

  initGraphics() {
    this.zone.runOutsideAngular(
      (): void => {
        const app: PIXI.Application = new PIXI.Application({});

        this.elRef.nativeElement.appendChild(app.view);

        app.stage.addChild(this.createBackground(app));

        app.stage.addChild(this.createPlayer(100, 100));

        app.ticker.add((delta) => {
          app.render();
        });

        app.render();
      }
    );
  }

  createPlayer(x: number, y: number) {
    const circle: Graphics = new PIXI.Graphics();

    circle.beginFill(0xff0000);
    circle.drawCircle(x, y, 5);
    circle.endFill();

    return circle;
  }

  createBackground(app: PIXI.Application) {
    // Background
    const landscapeTexture = PIXI.Texture.from('assets/images/match/field.svg');

    const background = new PIXI.Sprite(landscapeTexture);
    background.width = app.screen.width;
    background.height = app.screen.height;

    return background;
  }

  createBall(): Sprite {
    let center_x = POLE_COORDS.tl.x + ((POLE_COORDS.tr.x - POLE_COORDS.tl.x) / 2);
    let center_y = POLE_COORDS.tl.y + ((POLE_COORDS.bl.y - POLE_COORDS.tl.y) / 2);

    const ball: PIXI.Sprite = PIXI.Sprite.from(
      'assets/images/match/ball.png'
    );

    ball.x = center_x;
    ball.y = center_y;

    return ball;
  }
}
