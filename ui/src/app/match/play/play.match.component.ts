import {
    AfterViewInit,
    Component,
    ElementRef, HostListener, Input,
    NgZone,
    OnDestroy, OnInit,
    ViewChild
} from '@angular/core';
import * as PIXI from 'pixi.js';
import {Assets, Container, Graphics, Sprite, TextStyle} from "pixi.js";
import {UntilDestroy} from "@ngneat/until-destroy";
import {MatchPlayService} from "../services/match.play.service";
import {MatchDataService} from "../services/match.data.service";
import {MatchDataDto, MatchPlayerDto, MatchService, ObjectPositionDto} from "../services/match.service";

@UntilDestroy()
@Component({
    selector: 'play-match',
    templateUrl: './play.match.component.html',
    styleUrls: ['./play.match.component.scss']
})
export class MatchPlayComponent implements AfterViewInit, OnInit, OnDestroy {
    isDisposed = false;

    @ViewChild('matchContainer') matchContainer!: ElementRef;

    private background: PIXI.Sprite | null = null;
    private gameContainer: PIXI.Container | null = null;

    application: PIXI.Application | null = null;

    dataLoaded = false;

    matchTimeMs: number = -1;

    currentTime = 0;

    isFullscreen: boolean = false;

    private aspectRatio: number = 16 / 11; // Typical aspect ratio for a football field
    private maxWidth: number = 1400; // Maximum width of the game area
    private maxHeight: number = 890; // Maximum height of the game area

    @Input()
    leagueSlug: string = '';
    @Input()
    matchId: string = '';

    constructor(private zone: NgZone,
                public matchPlayService: MatchPlayService,
                public matchService: MatchService,
                public matchDataService: MatchDataService) {
        console.log(PIXI.VERSION);
    }

    ngOnInit(): void {
        this.matchPlayService.timeChanged$.subscribe(time => {
            this.currentTime = time;
        });

        document.addEventListener('fullscreenchange', this.onFullscreenChange.bind(this));
        document.addEventListener('webkitfullscreenchange', this.onFullscreenChange.bind(this));
        document.addEventListener('mozfullscreenchange', this.onFullscreenChange.bind(this));
        document.addEventListener('MSFullscreenChange', this.onFullscreenChange.bind(this));
    }

    @HostListener('window:resize', ['$event'])
    onResize(event: Event) {
        this.resizePixiApp();
    }

    resizePixiApp() {
        if (!this.application || !this.background || !this.gameContainer) return;

        const parent = this.matchContainer.nativeElement.parentElement;
        let width = parent.clientWidth;
        let height = parent.clientHeight;

        // Calculate the dimensions while maintaining aspect ratio
        const containerAspectRatio = width / height;

        if (containerAspectRatio > this.aspectRatio) {
            // Container is wider than needed
            width = height * this.aspectRatio;
        } else {
            // Container is taller than needed
            height = width / this.aspectRatio;
        }

        // Update the application size
        this.application.renderer.resize(width, height);

        // Scale the background to fill the screen
        const scaleX = width / this.background.texture.width;
        const scaleY = height / this.background.texture.height;
        const scale = Math.max(scaleX, scaleY);
        this.background.scale.set(scale);

        // Center the background
        this.background.position.set(
            (width - this.background.width) / 2,
            (height - this.background.height) / 2
        );

        // Scale the game container
        const gameScale = Math.min(width / this.maxWidth, height / this.maxHeight);
        this.gameContainer.scale.set(gameScale);

        // Center the game container
        this.gameContainer.position.set(
            (width - this.maxWidth * gameScale) / 2,
            (height - this.maxHeight * gameScale) / 2
        );

        console.log("SET SIZE: " + width + ", " + height);

        this.matchDataService.setResolution(this.maxWidth, this.maxHeight);
    }

    async setupGraphics(data: MatchDataDto) {
        if (!this.gameContainer) return;

        //create players
        Object.entries(data.player_positions).forEach(([key, value]: [string, ObjectPositionDto[]]) => {
            let player = this.getPlayer(Number(key));

            if(player) {
                const playerObj = this.createPlayer(value[0].position[0], value[0].position[1], player);

                this.matchDataService.setPlayerGraphicsObject(Number(key), playerObj);

                this.gameContainer!.addChild(playerObj);
            }
        });

        // create ball
        const ball = await this.createBall(data);

        this.gameContainer.addChild(ball);

        this.matchDataService.match!.ball.obj = ball;

        this.matchPlayService.startMatch();
    }

    getPlayer(playerId: number): MatchPlayerDto{
        return this.matchDataService.match!.players.find((player) => player.id == playerId)!;
    }

    public ngAfterViewInit(): void {
        this.matchDataService.setResolution(this.maxWidth, this.maxHeight);

        this.matchService.data(this.leagueSlug, this.matchId).subscribe(async matchData => {
            this.dataLoaded = true;

            this.matchDataService.setMatchData(matchData);

            await this.initGraphics();
            await this.setupGraphics(matchData);
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
                    width: this.maxWidth,
                    height: this.maxHeight,
                    backgroundColor: 0x022C0D // Dark green background color
                });

                this.matchContainer.nativeElement.appendChild(this.application.canvas);

                this.background = await this.createBackground();
                this.application.stage.addChild(this.background);

                this.gameContainer = new PIXI.Container();
                this.application.stage.addChild(this.gameContainer);

                this.resizePixiApp();

                this.application.ticker.add((delta) => {
                    if (this.isDisposed) {
                        return;
                    }

                    this.matchPlayService.tick();
                });

                this.application.render();
            }
        );
    }

    createPlayer(x: number, y: number, player: MatchPlayerDto): Container {
        const container = new Container();

        container.position.x = x - 10;
        container.position.y = y - 10;

        const circle: Graphics = new PIXI.Graphics();

        circle
            .circle(6, 6, 15)
            .fill(this.getColor(player));

        container.addChild(circle);

        const style = new TextStyle({
            fontFamily: 'Arial',
            fontSize: 15,
            fill: 'white',
            wordWrap: false,
            align: 'center'
        });

        const text = new PIXI.Text({text: player.last_name + ' ' + player.first_name.charAt(0).toUpperCase() + '.', style});

        text.x = 10;
        text.y = 30;

        text.anchor.set(0.5); // Set anchor to center for center alignment

        container.addChild(text);

        return container;
    }

    getColor(player: MatchPlayerDto) {
        if (player.position == "GK") {
            return 0xf7e300;
        }

        const homeColor = 0x00307d;
        const awayColor = 0xb33f00;

        return player.is_home ? homeColor : awayColor;
    }

    async createBackground() {
        const landscapeTexture = await Assets.load('assets/images/match/field.svg');
        const background = new PIXI.Sprite(landscapeTexture);

        background.width = this.maxWidth;
        background.height = this.maxHeight;

        return background;
    }

    async createBall(data: MatchDataDto): Promise<Sprite> {
        const texture = await Assets.load('assets/images/match/ball.png');
        const ball: PIXI.Sprite = new Sprite(texture);

        ball.position.x = data.ball_positions[0].position[0];
        ball.position.y = data.ball_positions[0].position[1];

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
