import {
    AfterViewInit,
    Component,
    ElementRef, Input,
    NgZone,
    OnDestroy, OnInit,
    ViewChild
} from '@angular/core';
import * as PIXI from 'pixi.js';
import {Assets, Container, Graphics, Sprite, TextStyle} from "pixi.js";
import {POLE_COORDS} from "./models/constants";
import {UntilDestroy} from "@ngneat/until-destroy";
import {MatchLineupModel, PlayerModel} from "./models/models";
import {MatchPlayService} from "../services/match.play.service";
import {MatchDataResultModel, MatchDataService} from "../services/match.data.service";
import {TitleService} from "../../shared/services/title.service";
import {TopHeaderService} from "../../shared/top-header/services/top.header.service";
import {MatchService} from "../services/match.service";

@UntilDestroy()
@Component({
    selector: 'play-match',
    templateUrl: './play.match.component.html',
    styleUrls: ['./play.match.component.scss']
})
export class MatchPlayComponent implements AfterViewInit, OnInit, OnDestroy {
    @ViewChild('matchContainer') matchContainer!: ElementRef;

    application: PIXI.Application | null = null;

    isDisposed = false;

    matchTimeMs: number = -1;

    currentTime = 0;

    isFullscreen: boolean = false;

    @Input()
    leagueSlug: string = '';
    @Input()
    matchId: string = '';

    constructor(private zone: NgZone,
                public matchPlayService: MatchPlayService,
                public matchService: MatchService,
                public matchDataService: MatchDataService,
                private titleService: TitleService,
                private topHeaderService: TopHeaderService) {
        console.log(PIXI.VERSION);
    }

    ngOnInit(): void {
        this.matchService.data(this.leagueSlug, this.matchId).subscribe(async matchData => {
            const data = this.matchDataService.matchData;

            this.titleService.setTitle(`${data.home_team.name} : ${data.away_team.name}`)
            this.topHeaderService.setContent(`${data.home_team.name} ${data.score.home_goals} : ${data.score.away_goals} ${data.away_team.name}`, '', '/', false);

            await this.initGraphics();
            await this.initGraphics(matchData);

            //this.application!.resizeTo = this.matchContainer.nativeElement;
        });

        this.matchPlayService.timeChanged$.subscribe(time => {
            this.currentTime = time;
        });

        document.addEventListener('fullscreenchange', this.onFullscreenChange.bind(this));
        document.addEventListener('webkitfullscreenchange', this.onFullscreenChange.bind(this));
        document.addEventListener('mozfullscreenchange', this.onFullscreenChange.bind(this));
        document.addEventListener('MSFullscreenChange', this.onFullscreenChange.bind(this));
    }

    async initGraphics(data: MatchDataResultModel) {
        // create ball
        const ball = await this.createBall(data);

        this.matchDataService.matchData.ball.obj = ball;
        this.application!.stage.addChild(ball);

        //create players
        data.players.forEach(player => {
            let translatedCoords = this.translateToField(player.position.x, player.data[0].y);

            const playerObj = this.createPlayer(translatedCoords.x, translatedCoords.y, player);

            this.matchDataService.setPlayerGraphicsObject(player.id, playerObj);

            this.application!.stage.addChild(playerObj);
        });

        this.matchPlayService.startMatch();
    }

    public ngAfterViewInit(): void {
        this.matchPlayService.objectPositionChanged$.subscribe(data => {
            const ballObject = this.matchDataService.matchData.ball.obj!;

            let ballCoord = this.translateToField(data.ball.x, data.ball.y);

            ballObject.x = ballCoord.x;
            ballObject.y = ballCoord.y;

            this.matchDataService.matchData.players.forEach((player, index) => {
                const playerObject = player.obj!;

                if (!playerObject) {
                    return;
                }

                const playerData = data.players.find(p => p.playerId == player.id);

                if (playerData && playerData.position) {
                    const playerPosition = playerData.position;

                    if (playerPosition) {
                        let playerTranslatedPositions = this.translateToField(
                            playerPosition.x,
                            playerPosition.y
                        );

                        playerObject.x = playerTranslatedPositions.x;
                        playerObject.y = playerTranslatedPositions.y;
                    }
                }
            });
        });
    }

    initGraphics(): Promise<void> {
        return this.zone.runOutsideAngular(
            async () => {
                this.application = new PIXI.Application();

                await this.application.init({
                    antialias: true,
                    autoDensity: true,
                    resolution: window.devicePixelRatio,
                    resizeTo: this.matchContainer.nativeElement,
                    width: 1000,
                    height: 700
                });

                this.matchContainer.nativeElement.appendChild(this.application.canvas);

                this.application.stage.addChild(await this.createBackground(this.application));

                this.application.ticker.add((delta) => {
                    if (this.isDisposed) {
                        return;
                    }

                    this.matchPlayService.tick();
                });

                this.application!.render();
            }
        );
    }

    translateToField(x: number, y: number) {
        const real_field_width = 840;
        const real_field_height = 545;

        const screen_field_width = POLE_COORDS.tr.x - POLE_COORDS.tl.x;
        const screen_field_height = POLE_COORDS.br.y - POLE_COORDS.tr.y;

        const scaleX = screen_field_width / 840;
        const scaleY = screen_field_height / 545;

        return {
            x: POLE_COORDS.tl.x + (x * scaleX),
            y: POLE_COORDS.tl.y + (y * scaleY)
        };
    }

    createPlayer(x: number, y: number, player: PlayerModel): Container {
        const container = new Container();

        container.position.x = x - 10;
        container.position.y = y - 10;

        const circle: Graphics = new PIXI.Graphics();

        circle
            .circle(6, 6, 12)
            .fill(this.getColor(player));

        container.addChild(circle);

        const style = new TextStyle({
            fontFamily: 'Arial',
            fontSize: 13,
            fill: 'white',
            wordWrap: false,
            align: 'center'
        });

        const text = new PIXI.Text({text: player.displayName, style});

        text.x = 10;
        text.y = 30;

        text.anchor.set(0.5); // Set anchor to center for center alignment

        container.addChild(text);

        return container;
    }

    getColor(player: PlayerModel) {
        if (player.position == "GK") {
            return 0xf7e300;
        }

        const homeColor = 0x00307d;
        const awayColor = 0xb33f00;

        return player.isHome ? homeColor : awayColor;
    }

    async createBackground(app: PIXI.Application) {
        const landscapeTexture = await Assets.load('assets/images/match/field.svg');
        const background = new PIXI.Sprite(landscapeTexture);

        background.width = app.screen.width;
        background.height = app.screen.height;

        return background;
    }

    async createBall(lineupData: MatchLineupModel): Promise<Sprite> {
        const texture = await Assets.load('assets/images/match/ball.png');
        const ball: PIXI.Sprite = new Sprite(texture);

        ball.position.x = lineupData.ball.data[0].x;
        ball.position.y = lineupData.ball.data[0].y;

        return ball;
    }

    toggleFullscreen() {
        if (!this.isFullscreen) {
            this.openFullscreen();
        } else {
            this.closeFullscreen();
        }
    }

    openFullscreen() {
        const elem = this.matchContainer.nativeElement;

        if (elem.requestFullscreen) {
            elem.requestFullscreen();
        } else if (elem.mozRequestFullScreen) { /* Firefox */
            elem.mozRequestFullScreen();
        } else if (elem.webkitRequestFullscreen) { /* Chrome, Safari & Opera */
            elem.webkitRequestFullscreen();
        } else if (elem.msRequestFullscreen) { /* IE/Edge */
            elem.msRequestFullscreen();
        }
    }

    closeFullscreen() {
        if (document.exitFullscreen) {
            document.exitFullscreen();
        } else if ((document as any).mozCancelFullScreen) { /* Firefox */
            (document as any).mozCancelFullScreen();
        } else if ((document as any).webkitExitFullscreen) { /* Chrome, Safari & Opera */
            (document as any).webkitExitFullscreen();
        } else if ((document as any).msExitFullscreen) { /* IE/Edge */
            (document as any).msExitFullscreen();
        }
    }

    onFullscreenChange() {
        const fullscreenElement =
            document.fullscreenElement ||
            (document as any).webkitFullscreenElement ||
            (document as any).mozFullScreenElement ||
            (document as any).msFullscreenElement;

        this.isFullscreen = !!fullscreenElement;

        // Resize the PIXI application
        if (this.application) {
            this.application.resize();
        }
    }

    ngOnDestroy(): void {
        this.isDisposed = true;
        this.application?.ticker.stop();

        document.removeEventListener('fullscreenchange', this.onFullscreenChange.bind(this));
        document.removeEventListener('webkitfullscreenchange', this.onFullscreenChange.bind(this));
        document.removeEventListener('mozfullscreenchange', this.onFullscreenChange.bind(this));
        document.removeEventListener('MSFullscreenChange', this.onFullscreenChange.bind(this));
    }
}
