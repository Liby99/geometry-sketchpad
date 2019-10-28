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

export type Line = {
  from: Position,
  to: Position,
};

export type LineStyle = {
  color: number,
  alpha: number,
  width: number,
};

export type Circle = {
  center: Position,
  radius: number,
};

export type CircleStyle = {
  fill: number,
  fillAlpha: number,
  border: LineStyle,
};

export type Rectangle = {
  minCorner: Position,
  maxCorner: Position,
};

export type RectangleStyle = {
  fill: number,
  fillAlpha: number,
  border: LineStyle,
};

export const EVENT_TYPE_INSERTED_POINT = 1;
export const EVENT_TYPE_INSERTED_LINE = 2;
export const EVENT_TYPE_INSERTED_CIRCLE = 3;
export const EVENT_TYPE_INSERTED_RECTANGLE = 4;
export const EVENT_TYPE_UPDATED_POINT = 5;
export const EVENT_TYPE_UPDATED_LINE = 6;
export const EVENT_TYPE_UPDATED_CIRCLE = 7;
export const EVENT_TYPE_UPDATED_RECTANGLE = 8;
export const EVENT_TYPE_UPDATED_POINT_STYLE = 9;
export const EVENT_TYPE_REMOVED_ENTITY = 13;
export const EVENT_TYPE_SELECTED_ENTITY = 14;
export const EVENT_TYPE_DESELECTED_ENTITY = 15;

export type RenderUpdateEvent =
| { type: 0 } // None
| { type: 1, entity: string, position: Position, style: PointStyle }  // insert point event
| { type: 2, entity: string, line: Line, style: LineStyle } // insert line event
| { type: 3, entity: string, circle: Circle, style: CircleStyle } // insert circle event
| { type: 4, entity: string, rect: Rectangle, style: RectangleStyle }
| { type: 5, entity: string, position: Position } // update point event
| { type: 6, entity: string, line: Line }
| { type: 7, entity: string, circle: Circle }
| { type: 8, entity: string, rect: Rectangle }
| { type: 9, entity: string, style: PointStyle } // update point style event
| { type: 10, entity: string, style: LineStyle }
| { type: 11, entity: string, style: CircleStyle }
| { type: 12, entity: string, style: RectangleStyle }
| { type: 13, entity: string } // remove point event
| { type: 14, entity: string } // select point event
| { type: 15, entity: string }; // deselect point event

export class GeopadWorld {
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