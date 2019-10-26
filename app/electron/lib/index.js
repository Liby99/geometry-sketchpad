const { promisify } = require('util');
const { GeopadWorld: RustChannel } = require('../native');

class GeopadWorld {
  constructor() {

    const channel = new RustChannel();
    const poll = promisify(channel.poll.bind(channel));

    this.channel = channel;

    this.interval = setInterval(() => {
      channel.step();
      poll().then((e) => {
        console.log(e);
      }).catch((err) => {
        console.error(err);
        clearInterval(this.interval);
      });
    }, 30);
  }

  // Mark the channel for shutdown
  shutdown() {
    this.channel.shutdown();
    clearInterval(this.interval);
  }
}

module.exports = GeopadWorld;