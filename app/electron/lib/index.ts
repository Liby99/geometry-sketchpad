import { promisify } from "util";
import * as Geopad from "../native";
import * as PIXI from "pixi.js";
import * as $ from "jquery";

import Point from "./point";

type RustChannel = Geopad.GeopadWorld;
const RustChannel = Geopad.GeopadWorld;

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

  update(event: Geopad.RenderUpdateEvent) {
    if (!event) { return; }
    switch (event.type) {
      case Geopad.EVENT_TYPE_INSERTED_POINT: {
        const point = new Point(event.position, event.style);
        this.points[event.entity] = point;
        this.app.stage.addChild(point.graphics);
      } break;
      case Geopad.EVENT_TYPE_UPDATED_POINT: {
        this.points[event.entity].updatePosition(event.position);
      } break;
      case Geopad.EVENT_TYPE_UPDATED_POINT_STYLE: {
        this.points[event.entity].updateStyle(event.style);
      } break;
      case Geopad.EVENT_TYPE_REMOVED_ENTITY: {
        if (event.entity in this.points) {
          this.app.stage.removeChild(this.points[event.entity].graphics);
          delete this.points[event.entity];
        }
      } break;
      case Geopad.EVENT_TYPE_SELECTED_ENTITY: {
        if (event.entity in this.points) {
          this.points[event.entity].setSelected(true);
        }
      } break;
      case Geopad.EVENT_TYPE_DESELECTED_ENTITY: {
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