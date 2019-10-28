import { Rectangle as RectangleData, RectangleStyle } from "../native";
import * as PIXI from "pixi.js";

export default class Rectangle {

  rect: RectangleData;
  style: RectangleStyle;
  graphics: PIXI.Graphics;

  constructor(rect: RectangleData, style: RectangleStyle) {

    // Basic information
    this.rect = rect;
    this.style = style;

    // Render information
    this.graphics = new PIXI.Graphics();
    this.setupGraphicsStyle();
  }

  updateRectangle(rect: RectangleData) {
    this.rect = rect;
    this.setupGraphicsStyle();
  }

  updateStyle(style: RectangleStyle) {
    this.style = style;
    this.setupGraphicsStyle();
  }

  setupGraphicsStyle() {
    this.graphics.clear();
    this.graphics.lineStyle(this.style.border.width, this.style.border.color, this.style.border.alpha);
    this.graphics.beginFill(this.style.fill, this.style.fillAlpha);
    this.graphics.drawRect(this.rect.x, this.rect.y, this.rect.width, this.rect.height);
    this.graphics.endFill();
  }
}