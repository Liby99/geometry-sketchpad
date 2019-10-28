import { Line, LineStyle } from "../native";
import * as PIXI from "pixi.js";

export default class Point {

  line: Line;
  style: LineStyle;
  selected: boolean;
  graphics: PIXI.Graphics;

  constructor(line: Line, style: LineStyle) {

    // Basic information
    this.line = line;
    this.style = style;
    this.selected = false;

    // Render information
    this.graphics = new PIXI.Graphics();
    this.setupGraphicsStyle();
  }

  updateLine(line: Line) {
    this.line = line;
    this.setupGraphicsStyle();
  }

  updateStyle(style: LineStyle) {
    this.style = style;
    this.setupGraphicsStyle();
  }

  setSelected(selected: boolean) {
    this.selected = selected;
    this.setupGraphicsStyle();
  }

  setupGraphicsStyle() {
    this.graphics.clear();
    this.graphics.lineStyle(this.style.width, this.style.color, this.style.alpha);
    this.graphics.moveTo(this.line.from.x, this.line.from.y);
    this.graphics.lineTo(this.line.to.x, this.line.to.y);

    // if (this.selected) {
    //   this.graphics.beginFill(0x000000, 0);
    //   this.graphics.lineStyle(1, 0xff00ff);
    //   this.graphics.drawEllipse(0, 0, this.style.radius + this.style.borderWidth / 2 + 3, this.style.radius + this.style.borderWidth / 2 + 3);
    //   this.graphics.endFill();
    // }
  }
}