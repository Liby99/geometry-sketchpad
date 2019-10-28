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

    if (this.selected) {
      let offset = this.style.width / 2 + 3;
      let dir = { x: this.line.to.x - this.line.from.x, y: this.line.to.y - this.line.from.y };
      let magnitude = Math.sqrt(dir.x * dir.x + dir.y * dir.y);
      let perpDir = { x: -dir.y / magnitude * offset, y: dir.x / magnitude * offset };
      this.graphics.lineStyle(1, 0xff00ff);
      this.graphics.moveTo(this.line.from.x + perpDir.x, this.line.from.y + perpDir.y);
      this.graphics.lineTo(this.line.to.x + perpDir.x, this.line.to.y + perpDir.y);
      this.graphics.moveTo(this.line.from.x - perpDir.x, this.line.from.y - perpDir.y);
      this.graphics.lineTo(this.line.to.x - perpDir.x, this.line.to.y - perpDir.y);
    }
  }
}