export interface PositionModel {
  x: number,
  y: number,
}

export interface PoleCoordModel {
  tl: PositionModel,
  tr: PositionModel,
  bl: PositionModel,
  br: PositionModel
}

export const POLE_COORDS: PoleCoordModel = {
  tl: { x: 38, y: 38 },
  tr: { x: 603, y: 38 },
  bl: { x: 38, y: 422 },
  br: { x: 603, y: 422 }
}
