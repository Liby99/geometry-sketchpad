import { Position, PointStyle } from "../native";
import * as PIXI from "pixi.js";

export default class Point {

  point: Position;
  style: PointStyle;
  selected: boolean;
  graphics: PIXI.Graphics;

  constructor(point: Position, style: PointStyle) {

    // Basic information
    this.point = point;
    this.style = style;
    this.selected = false;

    // Render information
    this.graphics = new PIXI.Graphics();
    this.setupGraphicsPoint();
    this.setupGraphicsStyle();
  }

  updatePoint(point: Position) {
    this.point = point;
    this.setupGraphicsPoint();
  }

  updateStyle(style: PointStyle) {
    this.style = style;
    this.setupGraphicsStyle();
  }

  setSelected(selected: boolean) {
    this.selected = selected;
    this.setupGraphicsStyle();
  }

  setupGraphicsPoint() {
    this.graphics.x = this.point.x;
    this.graphics.y = this.point.y;
  }

  setupGraphicsStyle() {
    this.graphics.clear();
    this.graphics.beginFill(this.style.color, this.style.alpha);
    this.graphics.lineStyle(this.style.borderWidth, this.style.borderColor, this.style.borderAlpha);
    this.graphics.drawEllipse(0, 0, this.style.radius - this.style.borderWidth / 2, this.style.radius - this.style.borderWidth / 2);
    this.graphics.endFill();

    if (this.selected) {
      this.graphics.beginFill(0x000000, 0);
      this.graphics.lineStyle(1, 0xff00ff);
      this.graphics.drawEllipse(0, 0, this.style.radius + this.style.borderWidth / 2 + 3, this.style.radius + this.style.borderWidth / 2 + 3);
      this.graphics.endFill();
    }
  }
}