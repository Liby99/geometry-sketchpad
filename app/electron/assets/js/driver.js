const GeopadWorld = require("../../lib/index");

const world = new GeopadWorld();

setTimeout(() => {
  world.shutdown();
}, 10000);