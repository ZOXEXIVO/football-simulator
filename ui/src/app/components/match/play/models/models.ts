import {Sprite} from "@pixi/sprite";
import {ObjectPositionDto} from "../../services/match.api.service";
import {Graphics} from "pixi.js";

export class MatchModel {
  constructor() {
    this.players = [];
    this.ball = new BallModel([]);
  }

  public players: PlayerModel[];
  public ball: BallModel;
}

export class PlayerModel {
  constructor(id: number, data: ObjectPositionDto[]) {
    this.id = id;
    this.obj = null;
    this.currentCoordIdx = 0;
    this.data = data;
  }

  public id: number;
  public obj: Graphics | null;
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
