const GeopadWorld = require("../../lib/index");
const $ = require("jquery");

const $canvas = $("#geopad-main-canvas");
const world = new GeopadWorld($canvas[0]);

$canvas.mousedown(() => {
  world.onMouseDown();
});

$canvas.mouseup(() => {
  world.onMouseUp();
});

let curr_position = [0, 0];
$canvas.mousemove((event) => {
  console.log("mouse over");
  let x = event.screenX, y = event.screenY;
  let relX = x - curr_position[0], relY = y - curr_position[1];
  curr_position = [x, y];
  if (relX !== 0 && relY !== 0) {
    world.onMouseMove(x, y, relX, relY);
  }
});