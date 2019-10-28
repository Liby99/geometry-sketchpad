export type Position = {
  x: number,
  y: number,
};

export type PointStyle = {
  color: number,
  alpha: number,
  radius: number,
  borderColor: number,
  borderAlpha: number,
  borderWidth: number,
};

export type RenderUpdateEvent = {
  type: 0, // None
} | {
  type: 1, // insert point event
  entity: string,
  position: Position,
  style: PointStyle,
} | {
  type: 5, // update point event
  entity: string,
  position: Position,
} | {
  type: 9, // update point style event
  entity: string,
  style: PointStyle,
} | {
  type: 13, // remove point event
  entity: string,
} | {
  type: 14, // select point event
  entity: string,
} | {
  type: 15, // deselect point event
  entity: string,
};

export default class GeopadWorld {
  constructor();
  poll(callback: (event: RenderUpdateEvent) => void) : void;
  step() : void;
  onMouseMove(x: number, y: number, relX: number, relY: number) : void;
  onMouseDown() : void;
  onMouseUp() : void;
  onKeyDown(key: number) : void;
  onKeyUp(key: number) : void;
  shutdown() : void;
}