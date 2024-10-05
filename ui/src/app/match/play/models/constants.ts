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
    tl: {x: 55, y: 53},
    tr: {x: 745, y: 53},
    bl: {x: 55, y: 543},
    br: {x: 745, y: 543}
}
