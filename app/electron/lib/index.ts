import { promisify } from "util";
import { GeopadWorld as RustChannel, Position, PointStyle, RenderUpdateEvent } from "../native";
import * as PIXI from "pixi.js";
import * as $ from "jquery";

const EVENT_TYPE_INSERTED_POINT = 1;
const EVENT_TYPE_INSERTED_LINE = 2;
const EVENT_TYPE_INSERTED_CIRCLE = 3;
const EVENT_TYPE_INSERTED_RECTANGLE = 4;
const EVENT_TYPE_UPDATED_POINT = 5;
const EVENT_TYPE_UPDATED_LINE = 6;
const EVENT_TYPE_UPDATED_CIRCLE = 7;
const EVENT_TYPE_UPDATED_RECTANGLE = 8;
const EVENT_TYPE_UPDATED_POINT_STYLE = 9;
const EVENT_TYPE_REMOVED_ENTITY = 13;
const EVENT_TYPE_SELECTED_ENTITY = 14;
const EVENT_TYPE_DESELECTED_ENTITY = 15;

class Point {

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

interface Storage<T> {
  [entity: string]: T
}

export default class GeopadWorld {

  $canvas: JQuery<HTMLElement>;
  app: PIXI.Application;
  channel: RustChannel;

  isShutdown: boolean;

  points: Storage<Point>;
  // lines: Storage<

  constructor($canvas: JQuery<HTMLElement>) {
    this.$canvas = $canvas;

    // Initialize PIXI application
    const $window = $(window);
    this.app = new PIXI.Application({
      width: $window.width(),
      height: $window.height(),
      antialias: true,
    });
    this.app.renderer.backgroundColor = 0xffffff;
    this.app.renderer.autoResize = true;
    $canvas[0].appendChild(this.app.view);

    // Initialize Backend
    this.channel = new RustChannel();
    this.isShutdown = false;

    // Geometry storages
    this.points = {};
    // this.lines = {};
    // this.circles = {};
    // this.rectangles = {};

    const poll = promisify(this.channel.poll.bind(this.channel));

    // Pooling loop getting the information from rust channel
    const pollLoop = () => {
      if (this.isShutdown) return;
      poll().then(this.update.bind(this)).catch(console.error).then(() => setImmediate(pollLoop));
    };
    pollLoop();

    // Render interval does not depend on polling loop
    setInterval(() => {
      this.channel.step();
    }, 10);

    // Setup callbacks to canvas
    this.$canvas.mousedown(() => {
      this.channel.onMouseDown();
    });

    this.$canvas.mouseup(() => {
      this.channel.onMouseUp();
    });

    let currPosition = [0, 0];
    this.$canvas.mousemove((event) => {
      const x = event.pageX, y = event.pageY;
      const relX = x - currPosition[0], relY = y - currPosition[1];
      currPosition = [x, y];
      this.channel.onMouseMove(x, y, relX, relY);
    });

    $(document).keydown((event) => {
      let key = event.which;
      this.channel.onKeyDown(key);
    });

    $(document).keyup((event) => {
      let key = event.which;
      this.channel.onKeyUp(key);
    });
  }

  update(event: RenderUpdateEvent) {
    if (!event) { return; }
    switch (event.type) {
      case EVENT_TYPE_INSERTED_POINT: {
        const point = new Point(event.position, event.style);
        this.points[event.entity] = point;
        this.app.stage.addChild(point.graphics);
      } break;
      case EVENT_TYPE_UPDATED_POINT: {
        this.points[event.entity].updatePosition(event.position);
      } break;
      case EVENT_TYPE_UPDATED_POINT_STYLE: {
        this.points[event.entity].updateStyle(event.style);
      } break;
      case EVENT_TYPE_REMOVED_ENTITY: {
        if (event.entity in this.points) {
          this.app.stage.removeChild(this.points[event.entity].graphics);
          delete this.points[event.entity];
        }
      } break;
      case EVENT_TYPE_SELECTED_ENTITY: {
        if (event.entity in this.points) {
          this.points[event.entity].setSelected(true);
        }
      } break;
      case EVENT_TYPE_DESELECTED_ENTITY: {
        if (event.entity in this.points) {
          this.points[event.entity].setSelected(false);
        }
      }
    }
  }

  shutdown() {
    this.channel.shutdown();
    this.isShutdown = true;
  }
}