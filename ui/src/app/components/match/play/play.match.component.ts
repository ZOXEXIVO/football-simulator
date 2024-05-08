import {
    AfterViewInit, ChangeDetectorRef,
    Component,
    ElementRef, EventEmitter,
    NgZone,
    OnDestroy, Output,
    ViewChild
} from '@angular/core';
import * as PIXI from 'pixi.js';
import {Assets, Container, Graphics, Sprite, TextStyle} from "pixi.js";
import {MatchDataService} from "../services/match.data.service";
import {POLE_COORDS} from "./models/constants";
import {UntilDestroy, untilDestroyed} from "@ngneat/until-destroy";
import {PlayerModel} from "./models/models";

@UntilDestroy()
@Component({
    selector: 'play-match',
    template: '<div #matchContainer style="min-height: 500px;"></div>'
})
export class MatchPlayComponent implements AfterViewInit, OnDestroy {
    @ViewChild('matchContainer') matchContainer!: ElementRef;

    application: PIXI.Application | null = null;

    isDisposed = false;

    @Output()
    @Output() currentTimeChanged = new EventEmitter<number>();

    currentTime = 0;

    constructor(private zone: NgZone,
                private matchDataService: MatchDataService,
                private changeDetectorRef: ChangeDetectorRef) {
        console.log(PIXI.VERSION);
    }

    public ngAfterViewInit(): void {

        this.initGraphics();
    }

    initGraphics() {
        this.zone.runOutsideAngular(
            async (): Promise<void> => {
                this.application = new PIXI.Application();

                await this.application.init({
                    antialias: true,
                });

                this.matchContainer.nativeElement.appendChild(this.application.canvas);

                this.application.stage.addChild(await this.createBackground(this.application));

                const ball = await this.createBall();

                this.matchDataService.matchData.ball.obj = ball;

                this.application.stage.addChild(ball);

                const app = this.application;

                this.matchDataService.matchData.players.forEach(player => {
                    let translatedCoords = this.translateToField(player.data[0].x, player.data[0].y);

                    debugger;
                    const playerObj = this.createPlayer(translatedCoords.x, translatedCoords.y, player);

                    player.obj = playerObj;

                    app.stage.addChild(playerObj);
                });

                this.application.ticker.add((delta) => {
                    if (this.isDisposed) {
                        return;
                    }

                    return;

                    // this.matchDataService.matchData.players.forEach((player, index) => {
                    //     const playerObject = player.obj!;
                    //
                    //     if (Number.isNaN(playerObject.y)) {
                    //         playerObject.y = 0;
                    //     }
                    //     console.log("Id = " + player.id + ', xy= (' + playerObject.x, +',' + playerObject.y + ')')
                    // });

                    // this.matchDataService.getData(this.currentTime).pipe(untilDestroyed(this)).subscribe(data => {
                    //     const ballObject = this.matchDataService.matchData.ball.obj!;
                    //
                    //     let coord = this.translateToField(data.ball.x, data.ball.y);
                    //
                    //     ballObject.x = coord.x;
                    //     ballObject.y = coord.y;
                    //
                    //     this.matchDataService.matchData.players.forEach((player, index) => {
                    //         const playerObject = player.obj!;
                    //         const playerData = data.players.find(p => p.playerId == player.id);
                    //
                    //         if (playerData && playerData.position) {
                    //             const playerPosition = playerData.position;
                    //
                    //             if (playerPosition) {
                    //                 let playerTranslatedPositions = this.translateToField(
                    //                     playerPosition.x,
                    //                     playerPosition.y
                    //                 );
                    //
                    //                 playerObject.x = playerTranslatedPositions.x;
                    //                 playerObject.y = playerTranslatedPositions.y;
                    //             }
                    //         }
                    //     });
                    //
                    //     this.currentTime += 10;
                    //     this.currentTimeChanged.next(this.currentTime);
                    // });
                });

                this.application.render();
            }
        );
    }

    translateToField(x: number, y: number) {
        let scaleX = (POLE_COORDS.tr.x - POLE_COORDS.tl.x) / 840;
        let scaleY = (POLE_COORDS.bl.y - POLE_COORDS.tl.y) / 545;

        return {
            x: POLE_COORDS.tl.x + (x * scaleX),
            y: POLE_COORDS.tl.y + (y * scaleY)
        }
    }

    createPlayer(x: number, y: number, player: PlayerModel): Container {
        const container = new Container();

        container.x = x;
        container.y = y;

        const homeColor = 0x6d02f7;
        const awayColor = 0xc93ecf;

        const circle: Graphics = new PIXI.Graphics();

        circle
            .circle(x, y, 10)
            .fill(player.isHome ? homeColor : awayColor);

        container.addChild(circle);

        const style = new TextStyle({
            fontFamily: 'Arial',
            fontSize: 12,
            fill: 'white',
            wordWrap: false,
            align: "center"
        });

        const text = new PIXI.Text({text: player.displayName, style});

        text.x = x - 30;
        text.y = y + 10;

        text.anchor.set(-0.2);

        container.addChild(text);

        return container;
    }

    async createBackground(app: PIXI.Application) {
        const landscapeTexture = await Assets.load('assets/images/match/field.svg');
        const background = new PIXI.Sprite(landscapeTexture);

        background.width = app.screen.width;
        background.height = app.screen.height;

        return background;
    }

    async createBall(): Promise<Sprite> {
        const texture = await Assets.load('assets/images/match/ball.png');
        const ball: PIXI.Sprite = new Sprite(texture);

        ball.x = POLE_COORDS.tl.x + ((POLE_COORDS.tr.x - POLE_COORDS.tl.x) / 2);
        ball.y = POLE_COORDS.tl.y + ((POLE_COORDS.bl.y - POLE_COORDS.tl.y) / 2);

        return ball;
    }

    ngOnDestroy(): void {
        this.isDisposed = true;
        this.application?.ticker.stop();
    }
}
