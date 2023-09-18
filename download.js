let fs = require("fs").promises;

var addon = require("./index.node");

let curIndex = 0;

async function fetchAll() {
  let env = await fs.readFile("./.env");
  let histId = env.toString().split("\n")[1].split("=")[1];
  while (true) {
    console.log(`Downloading ${curIndex}.`);
    const response = await fetch(
      `https://cloud.culturedcode.com/version/1/history/${histId}/items?start-index=${curIndex}`
    );
    const json = await response.json();

    fs.writeFile(`./downloads/${curIndex}.json`, JSON.stringify(json));
    console.log(`Downloaded ${curIndex}.`);
    curIndex += json.items.length;
    if (curIndex == json["current-item-index"]) {
      break;
    }
  }
  console.log("\n\nDone");
}

async function parseAll() {
  let dir = await fs.readdir("./downloads");
  for (const fileName of dir.reverse()) {
    if (fileName.startsWith(".") || !fileName.endsWith(".json")) {
      continue;
    }
    console.log(`Parsing File: ${fileName}`);
    addon.parseFile(await fs.readFile(`./downloads/${fileName}`));
  }

  // let rust read all stuff
}
fetchAll();
parseAll();
