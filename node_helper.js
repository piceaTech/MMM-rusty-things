var addon = require("./native");

const util = require("util");
const exec = util.promisify(require("child_process").exec);

async function displayTurnedOn() {
  try {
    const { stdout, stderr } = await exec("vcgencmd display_power");
    return stdout == "display_power=1\n";
  } catch (e) {
    console.log(
      "Got Error while checking for display on. Assuming it is turned on."
    );
    return true;
  }
}

var NodeHelper = require("node_helper");
module.exports = NodeHelper.create({
  timer: null,
  lastClientPing: +new Date(),
  lastHistID: 0,
  lastUpdate: 0,
  start: function () {
    const self = this;
    console.log("Starting node helper for: " + this.name);
    try {
      self.lastHistID = addon.updateDB(__dirname);
    } catch (e) {
      console.log("err while starting:");
      console.log(e);
      self.lastHistID = addon.getLastID(__dirname);
    }
    this.lastUpdate = +new Date();
    setInterval(function () {
      if (!self.isClientActive() && self.timer != null) {
        console.log("Clearing Rusty Update");
        clearInterval(self.timer);
        self.timer = null;
      }
    }, 40 * 1000); //perform every 40 secs.
  },
  socketNotificationReceived: function (notification, payload) {
    console.log("server:", notification, payload);
    if (notification === "CLIENT_ACTIVE") {
      this.lastClientPing = +new Date();
      if (this.timer == null) {
        console.log("sending reload");
        this.sendSocketNotification("RELOAD");
        this.createTimer();
      }
      if (payload && payload.currentState !== this.lastHistID) {
        this.sendSocketNotification("DB_UPDATED", {
          currentState: this.lastHistID,
        });
      }
    } else if (notification === "REQUEST_ENTRIES") {
      let today = addon.getTodayEntries(__dirname);
      let tomorrow = addon.getTomorrowEntries(__dirname);
      let inbox = addon.getInboxEntries(__dirname);
      this.sendSocketNotification("RESPONSE_ENTRIES", {
        today,
        tomorrow,
        inbox,
        currentState: this.lastHistID,
      });
    }
  },
  createTimer: function () {
    const self = this;
    this.timer = setInterval(async function () {
      if (self.isClientActive()) {
        const before = self.lastHistID;
        const isDisplayOn = await displayTurnedOn();
        if (
          !isDisplayOn &&
          self.lastUpdate + 2 * 60 * 60 * 1000 > +new Date()
        ) {
          console.log("Display is off. Updating only every 2 hours");
          return;
        }
        try {
          self.lastHistID = addon.updateDB(__dirname);
          this.lastUpdate = +new Date();
        } catch (e) {
          console.log("err bei update");
          console.log(e);
        }
        if (self.lastHistID > before) {
          self.sendSocketNotification("DB_UPDATED", {
            currentState: this.lastHistID,
          });
          console.log("updated DB");
        } else {
          console.log("nothing to update");
        }
      }
    }, 5 * 60 * 1000); //perform every 5 minutes.
  },
  isClientActive() {
    return this.lastClientPing + 30 * 1000 > +new Date();
  },
});
