// update db and output today
var addon = require("./index.node");

// let lastHistID = addon.getLastID(__dirname);
let lastHistID = addon.updateDB(__dirname);

let today = addon.getTodayEntries(__dirname);
let tomorrow = addon.getTomorrowEntries(__dirname);
let inbox = addon.getInboxEntries(__dirname);

console.log("lastHistID", lastHistID);
console.log("today", today);
// console.log("tomorrow", tomorrow);
// console.log("inbox", inbox);

console.log("len", today.length, tomorrow.length, inbox.length);
