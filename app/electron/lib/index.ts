import { promisify } from "util";
import * as Geopad from "../native";
import * as $ from "jquery";

import * as PIXI from "pixi.js";
window.PIXI = PIXI;
import "pixi-layers";

import Point from "./point";
import Line from "./line";
import Circle from "./circle";

type RustChannel = Geopad.GeopadWorld;
const RustChannel = Geopad.GeopadWorld;

interface Storage<T> {
  [entity: string]: T
}

export default class GeopadWorld {

  $canvas: JQuery<HTMLElement>;
  channel: RustChannel;
  isShutdown: boolean;

  app: PIXI.Application;
  pointGroup: PIXI.display.Group;
  lineGroup: PIXI.display.Group;
  circleGroup: PIXI.display.Group;
  // rectangleLayer: PIXI.display.Layer;

  points: Storage<Point>;
  lines: Storage<Line>;
  circles: Storage<Circle>;

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

    // Create the groups
    this.pointGroup = new PIXI.display.Group(3, false);
    this.lineGroup = new PIXI.display.Group(2, false);
    this.circleGroup = new PIXI.display.Group(1, false);

    // Setup stages
    this.app.stage = new PIXI.display.Stage();
    this.app.stage.sortableChildren = true;
    this.app.stage.addChild(new PIXI.display.Layer(this.pointGroup));
    this.app.stage.addChild(new PIXI.display.Layer(this.lineGroup));
    this.app.stage.addChild(new PIXI.display.Layer(this.circleGroup));

    // Setup canvas
    $canvas[0].appendChild(this.app.view);

    // Initialize Backend
    this.channel = new RustChannel();
    this.isShutdown = false;

    // Geometry storages
    this.points = {};
    this.lines = {};
    this.circles = {};
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
    }, 16);

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

  update(event: Geopad.RenderUpdateEvent) {
    if (!event) { return; }
    switch (event.type) {
      case Geopad.EVENT_TYPE_INSERTED_POINT: {
        const point = new Point(event.point, event.style);
        this.points[event.entity] = point;
        this.app.stage.addChild(point.graphics);
        point.graphics.parentGroup = this.pointGroup;
      } break;
      case Geopad.EVENT_TYPE_INSERTED_LINE: {
        const line = new Line(event.line, event.style);
        this.lines[event.entity] = line;
        this.app.stage.addChild(line.graphics);
        line.graphics.parentGroup = this.lineGroup;
      } break;
      case Geopad.EVENT_TYPE_INSERTED_CIRCLE: {
        const circle = new Circle(event.circle, event.style);
        this.circles[event.entity] = circle;
        this.app.stage.addChild(circle.graphics);
        circle.graphics.parentGroup = this.circleGroup;
      } break;
      case Geopad.EVENT_TYPE_UPDATED_POINT: {
        if (event.entity in this.points) {
          this.points[event.entity].updatePoint(event.point);
        }
      } break;
      case Geopad.EVENT_TYPE_UPDATED_LINE: {
        if (event.entity in this.lines) {
          this.lines[event.entity].updateLine(event.line);
        }
      } break;
      case Geopad.EVENT_TYPE_UPDATED_CIRCLE: {
        if (event.entity in this.circles) {
          this.circles[event.entity].updateCircle(event.circle);
        }
      } break;
      case Geopad.EVENT_TYPE_UPDATED_POINT_STYLE: {
        this.points[event.entity].updateStyle(event.style);
      } break;
      case Geopad.EVENT_TYPE_UPDATED_LINE_STYLE: {
        this.lines[event.entity].updateStyle(event.style);
      } break;
      case Geopad.EVENT_TYPE_UPDATED_CIRCLE_STYLE: {
        this.circles[event.entity].updateStyle(event.style);
      } break;
      case Geopad.EVENT_TYPE_REMOVED_ENTITY: {
        if (event.entity in this.points) {
          this.app.stage.removeChild(this.points[event.entity].graphics);
          delete this.points[event.entity];
        } else if (event.entity in this.lines) {
          this.app.stage.removeChild(this.lines[event.entity].graphics);
          delete this.lines[event.entity];
        } else if (event.entity in this.circles) {
          this.app.stage.removeChild(this.circles[event.entity].graphics);
          delete this.circles[event.entity];
        }
      } break;
      case Geopad.EVENT_TYPE_SELECTED_ENTITY: {
        if (event.entity in this.points) {
          this.points[event.entity].setSelected(true);
        } else if (event.entity in this.lines) {
          this.lines[event.entity].setSelected(true);
        } else if (event.entity in this.circles) {
          this.circles[event.entity].setSelected(true);
        }
      } break;
      case Geopad.EVENT_TYPE_DESELECTED_ENTITY: {
        if (event.entity in this.points) {
          this.points[event.entity].setSelected(false);
        } else if (event.entity in this.lines) {
          this.lines[event.entity].setSelected(false);
        } else if (event.entity in this.circles) {
          this.circles[event.entity].setSelected(false);
        }
      }
    }
  }

  shutdown() {
    this.channel.shutdown();
    this.isShutdown = true;
  }
}