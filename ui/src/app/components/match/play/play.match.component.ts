import {AfterViewInit, Component, ElementRef, Input, NgZone, ViewChild} from '@angular/core';
import * as PIXI from 'pixi.js';
import {Sprite} from '@pixi/sprite';
import {ActivatedRoute} from "@angular/router";
import {Container, Graphics} from "pixi.js";
import {MatchDataService} from "../services/match.data.service";
import {POLE_COORDS} from "./models/constants";

@Component({
  selector: 'play-match',
  template: '<div #matchContainer style="height: 600px;margin-left: auto;margin-right: auto;"></div>'
})
export class MatchPlayComponent implements AfterViewInit {
  @ViewChild('matchContainer') matchContainer!: ElementRef;

  @Input()
  leagueSlug: string;
  @Input()
  matchId: string;

  application: PIXI.Application | null = null;

  currentTime: number = 0;

  constructor(private matchDataService: MatchDataService,
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
        this.application = new PIXI.Application({
          //resizeTo: this.matchContainer.nativeElement
        });

        this.matchContainer.nativeElement.appendChild(this.application.view);

        this.application.stage.addChild(this.createBackground(this.application));

        const ball = this.createBall();
        this.matchDataService.matchData.ball.obj = ball;
        this.application.stage.addChild(ball);

        this.matchDataService.matchData.players.forEach(player => {
          const playerObj = this.createPlayer(player.data[0].x, player.data[0].y);

          player.obj = playerObj;

          this.application?.stage.addChild(playerObj);
        });

        // DEBUG
        // this.application.stage.addChild(this.createPlayer(POLE_COORDS.tl.x, POLE_COORDS.tl.y));
        // this.application.stage.addChild(this.createPlayer(POLE_COORDS.tr.x, POLE_COORDS.tr.y));
        // this.application.stage.addChild(this.createPlayer(POLE_COORDS.bl.x, POLE_COORDS.bl.y));
        // this.application.stage.addChild(this.createPlayer(POLE_COORDS.br.x, POLE_COORDS.br.y));

        this.application.ticker.add((delta) => {
          this.currentTime += 10;

          this.matchDataService.getData(this.currentTime).subscribe(data => {
            if(!data){
              return;
            }

            const ballObject = this.matchDataService.matchData.ball.obj!;
            ballObject.x = data.ball.x;
            ballObject.y = data.ball.y;

            this.matchDataService.matchData.players.forEach(player => {
              const playerObject = player.obj!;

              playerObject.x = data.players[player.id].position.x;
              playerObject.y = data.players[player.id].position.y;
            });
          });

          this.application?.render();
        });

        this.application.render();
      }
    );
  }

  createPlayer(x: number, y: number) {
    const circle: Graphics = new PIXI.Graphics();

    circle.beginFill(0xff0000);
    circle.drawCircle(x, y, 4);
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
