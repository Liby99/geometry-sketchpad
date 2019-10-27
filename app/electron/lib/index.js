const { promisify } = require('util');
const { GeopadWorld: RustChannel } = require('../native');
const PIXI = require("pixi.js");

const EVENT_TYPE_UPDATE_POINT = 1;
const EVENT_TYPE_REMOVE_ENTITY = 4;
const EVENT_TYPE_SELECT_ENTITY = 5;
const EVENT_TYPE_DESELECT_ENTITY = 6;

class Point {
  constructor(position, style) {

    // Basic information
    this.position = position;
    this.style = style;
    this.selected = false;

    // Render information
    this.graphics = new PIXI.Graphics();
    this.setupGraphics();
  }

  update(position, style) {
    this.position = position;
    this.style = style;

    this.graphics.clear();
    this.setupGraphics();
  }

  setupGraphics() {
    this.graphics.beginFill(this.style.color, this.style.alpha);
    this.graphics.lineStyle(this.style.borderWidth, this.style.borderColor, this.style.borderAlpha);
    this.graphics.drawEllipse(0, 0, this.style.radius, this.style.radius);
    this.graphics.endFill();
    this.graphics.x = this.position.x;
    this.graphics.y = this.position.y;
  }
}

class Line {
  constructor(from, to, style) {
    this.from = from;
    this.to = to;
    this.style = style;
  }

  draw(context, selected) {

  }
}

class Circle {
  constructor(center, radius, style) {
    this.center = center;
    this.radius = radius,
    this.style = style;
  }

  draw(context, selected) {

  }
}

class Rectangle {
  constructor(min, max, style) {
    this.min = min;
    this.max = max;
    this.style = style;
  }

  draw(context) {

  }
}

class GeopadWorld {
  constructor($canvas) {
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
    this.poll = promisify(this.channel.poll.bind(this.channel));
    this.isShutdown = false;

    // Geometry storages
    this.points = {};
    this.lines = {};
    this.circles = {};
    this.rectangles = {};

    // Pooling loop getting the information from rust channel
    const pollLoop = () => {
      if (this.isShutdown) return;
      this.poll().then(this.update.bind(this)).catch(console.error).then(() => setImmediate(pollLoop));
    };
    pollLoop();

    // Render interval does not depend on polling loop
    setInterval(() => {
      this.channel.step();
    }, 10);

    // In focus check
    this.isInFocus = false;
    this.$canvas.mouseover(() => {
      this.isInFocus = true;
    });
    this.$canvas.mouseleave(() => {
      this.isInFocus = false;
    });

    // Setup callbacks to canvas
    this.$canvas.mousedown(() => {
      this.channel.onMouseDown();
    });

    this.$canvas.mouseup(() => {
      this.channel.onMouseUp();
    });

    let currPosition = [0, 0];
    this.$canvas.mousemove((event) => {
      this.isInFocus = true;
      let x = event.pageX, y = event.pageY;
      let relX = x - currPosition[0], relY = y - currPosition[1];
      currPosition = [x, y];
      this.channel.onMouseMove(x, y, relX, relY);
    });

    $(document).keydown((event) => {
      if (this.isInFocus) {
        let key = event.which;
        this.channel.onKeyDown(key);
      }
    });

    $(document).keyup((event) => {
      let key = event.which;
      this.channel.onKeyUp(key);
    });
  }

  update(event) {
    if (!event) { return; }
    switch (event.type) {
      case EVENT_TYPE_UPDATE_POINT: {
        if (event.entity in this.points) {
          this.points[event.entity].update(event.position, event.style);
        } else {
          const point = new Point(event.position, event.style);
          this.points[event.entity] = point;
          this.app.stage.addChild(point.graphics);
        }
      } break;
    }
  }

  drawGeometries(geometries, selected) {
    Object.keys(geometries).forEach((key) => {
      geometries[key].draw(this.context, selected);
    });
  }

  shutdown() {
    this.channel.shutdown();
    this.isShutdown = true;
  }
}

module.exports = GeopadWorld;