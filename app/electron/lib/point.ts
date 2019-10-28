import { Position, PointStyle } from "../native";
import * as PIXI from "pixi.js";

export default class Point {

  position: Position;
  style: PointStyle;
  selected: boolean;
  graphics: PIXI.Graphics;

  constructor(position: Position, style: PointStyle) {

    // Basic information
    this.position = position;
    this.style = style;
    this.selected = false;

    // Render information
    this.graphics = new PIXI.Graphics();
    this.setupGraphicsPosition();
    this.setupGraphicsStyle();
  }

  updatePosition(position: Position) {
    this.position = position;
    this.setupGraphicsPosition();
  }

  updateStyle(style: PointStyle) {
    this.style = style;
    this.setupGraphicsStyle();
  }

  setSelected(selected: boolean) {
    this.selected = selected;
    this.setupGraphicsStyle();
  }

  setupGraphicsPosition() {
    this.graphics.x = this.position.x;
    this.graphics.y = this.position.y;
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