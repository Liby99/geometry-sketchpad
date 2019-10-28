import { Circle as CircleData, CircleStyle } from "../native";
import * as PIXI from "pixi.js";

export default class Circle {

  circle: CircleData;
  style: CircleStyle;
  selected: boolean;
  graphics: PIXI.Graphics;

  constructor(circle: CircleData, style: CircleStyle) {

    // Basic information
    this.circle = circle;
    this.style = style;
    this.selected = false;

    // Render information
    this.graphics = new PIXI.Graphics();
    this.setupGraphicsStyle();
  }

  updateCircle(circle: CircleData) {
    this.circle = circle;
    this.setupGraphicsStyle();
  }

  updateStyle(style: CircleStyle) {
    this.style = style;
    this.setupGraphicsStyle();
  }

  setSelected(selected: boolean) {
    this.selected = selected;
    this.setupGraphicsStyle();
  }

  setupGraphicsStyle() {
    this.graphics.clear();

    this.graphics.beginFill(this.style.fill, this.style.fillAlpha);
    this.graphics.lineStyle(this.style.border.width, this.style.border.color, this.style.border.alpha);
    this.graphics.drawEllipse(this.circle.center.x, this.circle.center.y, this.circle.radius, this.circle.radius);
    this.graphics.endFill();

    // if (this.selected) {
    //   this.graphics.beginFill(0x000000, 0);
    //   this.graphics.lineStyle(1, 0xff00ff);
    //   this.graphics.drawEllipse(0, 0, this.style.radius + this.style.borderWidth / 2 + 3, this.style.radius + this.style.borderWidth / 2 + 3);
    //   this.graphics.endFill();
    // }
  }
}