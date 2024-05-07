﻿import {
    AfterViewInit, ChangeDetectorRef,
    Component,
    ElementRef,
    Input,
    NgZone,
    OnDestroy,
    ViewChild
} from '@angular/core';
import * as PIXI from 'pixi.js';
import {Assets, Graphics, Sprite} from "pixi.js";
import {MatchDataService} from "../services/match.data.service";
import {POLE_COORDS} from "./models/constants";
import {UntilDestroy, untilDestroyed} from "@ngneat/until-destroy";

@UntilDestroy()
@Component({
    selector: 'play-match',
    template: '<div #matchContainer style="min-height: 500px;"></div>'
})
export class MatchPlayComponent implements AfterViewInit, OnDestroy {
    @ViewChild('matchContainer') matchContainer!: ElementRef;

    application: PIXI.Application | null = null;

    isDisposed = false;

    @Input()
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

                this.matchDataService.matchData.players.forEach(player => {
                    let translatedCoords = this.translateToField(player.data[0].x, player.data[0].y);
                    const playerObj = this.createPlayer(translatedCoords.x, translatedCoords.y, player.isHome)

                    player.obj = playerObj;

                    this.application?.stage.addChild(playerObj);
                });

                // console.log('players count = ' + this.matchDataService.matchData.players.length);

                // DEBUG
                // this.application.stage.addChild(this.createPlayer(POLE_COORDS.tl.x, POLE_COORDS.tl.y));
                // this.application.stage.addChild(this.createPlayer(POLE_COORDS.tr.x, POLE_COORDS.tr.y));
                // this.application.stage.addChild(this.createPlayer(POLE_COORDS.bl.x, POLE_COORDS.bl.y));
                // this.application.stage.addChild(this.createPlayer(POLE_COORDS.br.x, POLE_COORDS.br.y));

                this.application.ticker.add((delta) => {
                    if (this.isDisposed) {
                        return;
                    }

                    // console.log('time=' + this.currentTime);

                    this.matchDataService.getData(this.currentTime).pipe(untilDestroyed(this)).subscribe(data => {
                        const ballObject = this.matchDataService.matchData.ball.obj!;

                        let coord = this.translateToField(data.ball.x, data.ball.y);

                        //if(ballObject.x != coord.x && ballObject.y != coord.y){
                        //console.log(`ball move x = ${ballObject.x}, y = ${ballObject.y}`);

                        ballObject.x = coord.x;
                        ballObject.y = coord.y;

                        const scaleFactor = (data.ball.z + 20) / 20;

                        ballObject.scale.set(scaleFactor, scaleFactor);
                        //}

                        this.matchDataService.matchData.players.forEach((player, index) => {
                            const playerObject = player.obj!;
                            const playerData = data.players.find(p => p.playerId == player.id);

                            if (playerData && playerData.position) {

                                const playerPosition = playerData.position;

                                if (index == 0) {
                                    //console.log(`time = ${this.currentTime}: player position = (${playerPosition.x}, ${playerPosition.y})`);
                                }

                                if (playerPosition) {
                                    let playerTranslatedPositions = this.translateToField(
                                        playerPosition.x,
                                        playerPosition.y
                                    );

                                    playerObject.x = playerTranslatedPositions.x;
                                    playerObject.y = playerTranslatedPositions.y;

                                    const scaleFactor = (playerPosition.z + 20) / 20;

                                    playerObject.scale.set(scaleFactor, scaleFactor);
                                }
                            }
                        });
                    });
                });

                this.application.render();
                //this.changeDetectorRef.detectChanges();
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

    createPlayer(x: number, y: number, isHome: boolean): Graphics {
        const homeColor = 0x0000ff;
        const awayColor = 0xff0000;

        const circle: Graphics = new PIXI.Graphics();

        circle
            .fill(isHome ? homeColor : awayColor)
            .circle(x, y, 6);

        return circle;
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
