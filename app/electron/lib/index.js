const { promisify } = require('util');
const { GeopadWorld: RustChannel } = require('../native');

class Point {
  constructor(position, style) {
    this.position = position;
  }

  draw(context, selected) {

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
  constructor(canvas) {
    this.canvas = canvas;
    this.context = canvas.getContext("2d");
    this.channel = new RustChannel();
    this.poll = promisify(this.channel.poll.bind(this.channel));
    this.isShutdown = false;

    // Geometry storages
    this.points = {};
    this.selectedPoints = {};
    this.lines = {};
    this.selectedLines = {};
    this.circles = {};
    this.selectedCircles = {};
    this.rectangles = {};

    // Pooling loop getting the information from rust channel
    const self = this;
    (function pollLoop() {
      if (self.isShutdown) return;
      self.channel.step();
      self.poll().then((e) => {
        self.update(e);
      }).catch((err) => {
        console.error(err);
      }).then(() => {
        setImmediate(pollLoop);
      });
    })();

    // Render interval does not depend on polling loop
    setInterval(() => {
      this.render();
    }, 33);
  }

  update(event) {

  }

  render() {

    // First clear the context
    this.context.clearRect(0, 0, this.canvas.width, this.canvas.height);

    // Then draw all the geometries
    this.drawGeometries(this.circles, false);
    this.drawGeometries(this.selectedCircles, true);
    this.drawGeometries(this.lines, false);
    this.drawGeometries(this.selectedLines, true);
    this.drawGeometries(this.points, false);
    this.drawGeometries(this.selectedPoints, true);
    this.drawGeometries(this.rectangles);
  }

  drawGeometries(geometries, selected) {
    Object.keys(geometries).forEach((key) => {
      geometries[key].draw(this.context, selected);
    });
  }

  onMouseMove(x, y, relX, relY) {
    this.channel.onMouseMove(x, y, relX, relY);
  }

  onMouseDown() {
    this.channel.onMouseDown();
  }

  onMouseUp() {
    this.channel.onMouseUp();
  }

  shutdown() {
    this.channel.shutdown();
    this.isShutdown = true;
  }
}

module.exports = GeopadWorld;