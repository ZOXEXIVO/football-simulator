import {ObjectPositionDto} from "../../services/match.api.service";
import {Container, Graphics, Sprite} from "pixi.js";

export class MatchModel {
    constructor() {
        this.players = [];
        this.ball = new BallModel([]);
        this.squad = new SquadModel();

        this.home_team = new TeamModel('', '');
        this.away_team = new TeamModel('', '');

        this.score = new ScoreModel(0, 0);
    }

    public players: PlayerModel[];
    public ball: BallModel;
    public squad: SquadModel;

    public home_team: TeamModel;
    public away_team: TeamModel;

    public score: ScoreModel
}

export class TeamModel {
    constructor(name: string, slug: string) {
        this.name = name;
        this.slug = slug;
    }

    public name: string;
    public slug: string;
}


export class ScoreModel {
    constructor(home_goals: number, away_goals: number) {
        this.home_goals = home_goals;
        this.away_goals = away_goals;
    }

    public home_goals: number;
    public away_goals: number;
}

export class PlayerModel {
    constructor(id: number, displayName: string, idHome: boolean, data: ObjectPositionDto[]) {
        this.id = id;
        this.displayName = displayName;
        this.isHome = idHome;
        this.obj = null;
        this.currentCoordIdx = 0;
        this.data = data;
    }

    public id: number;
    public displayName: string;
    public isHome: boolean;
    public obj: Container | null;
    public currentCoordIdx: number;
    public data: ObjectPositionDto[];
}

export class BallModel {
    constructor(data: ObjectPositionDto[]) {
        this.obj = null;
        this.currentCoordIdx = 0;
        this.data = data;
    }

    public obj?: Sprite | null;
    public currentCoordIdx: number;
    public data: ObjectPositionDto[];
}

// Squad

export class SquadModel {
    constructor() {
        this.home = [];
        this.home_subs = [];

        this.away = [];
        this.away_subs = [];
    }

    public home: SquadPlayerModel[];
    public home_subs: SquadPlayerModel[];

    public away: SquadPlayerModel[];
    public away_subs: SquadPlayerModel[];
}

export class SquadPlayerModel {
    constructor(id: number,
                first_name: string, last_name: string, middle_name: string,
                position: string, team_slug: string) {
        this.id = id;
        this.first_name = first_name;
        this.last_name = last_name;
        this.middle_name = middle_name;
        this.position = position;
        this.team_slug = team_slug;
    }

    public id: number;
    public first_name: string;
    public last_name: string;
    public middle_name: string;
    public position: string;
    public team_slug: string;
}

export class MatchLineupModel {
    constructor(matchTimeMs: number, ball: BallModel, players: PlayerModel[]) {
        this.matchTimeMs = matchTimeMs;
        this.ball = ball;
        this.players = players;
    }

    public matchTimeMs: number;
    public ball: BallModel;
    public players: PlayerModel[];
}
