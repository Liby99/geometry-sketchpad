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
  constructor($canvas) {
    this.$canvas = $canvas;
    this.canvas = $canvas[0];
    this.context = this.canvas.getContext("2d");
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
      self.poll().then((event) => {
        self.update(event);
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
      let x = event.screenX, y = event.screenY;
      let relX = x - currPosition[0], relY = y - currPosition[1];
      currPosition = [x, y];
      if (relX !== 0 && relY !== 0) {
        this.channel.onMouseMove(x, y, relX, relY);
      }
    });

    $canvas.keydown((event) => {
      if (this.isInFocus) {
        let key = event.which;
        this.channel.onKeyDown(key);
      }
    });

    $canvas.keyup((event) => {
      let key = event.which;
      this.channel.onKeyUp(key);
    });
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

  shutdown() {
    this.channel.shutdown();
    this.isShutdown = true;
  }
}

module.exports = GeopadWorld;