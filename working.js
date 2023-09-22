// update db and output today
var addon = require("./index.node");

// let lastHistID = addon.getLastID(__dirname);
let lastHistID = addon.updateDB(__dirname);

let today = addon.getTodayEntries(__dirname);
let tomorrow = addon.getTomorrowEntries(__dirname);
let inbox = addon.getInboxEntries(__dirname);

console.log(
  "today",
  today.map((item) => "" + item.title + " -> " + item.uuid)
);
// console.log("tomorrow", tomorrow);
// console.log("inbox", inbox);
console.log("lastHistID", lastHistID);

console.log("len", today.length, tomorrow.length, inbox.length);

console.log(addon.get_canonical_id("65B00B03-0995-4EE3-BFF1-828A470D89AF"));
