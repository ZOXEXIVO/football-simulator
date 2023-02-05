import {Sprite} from "@pixi/sprite";
import {BallPositionDto, PlayerPositionDto} from "../services/match.data.service";

export class MatchModel {
  constructor() {
    this.players = [];
    this.ball = new BallModel();
  }

  public players: PlayerModel[];
  public  ball: BallModel;
}

export class PlayerModel {
  constructor(id: number) {
    this.id = id;
    this.obj = null;
    this.currentCoordIdx = 0;
    this.data = [];
  }

  public id: number;
  public obj: Sprite | null;
  public currentCoordIdx: number;
  public data: PlayerPositionDto[];
}

export class BallModel {
  constructor() {
    this.obj = null;
    this.currentCoordIdx = 0;
    this.data = [];
  }

  public obj?: Sprite | null;
  public currentCoordIdx: number;
  public data: BallPositionDto[];
}
