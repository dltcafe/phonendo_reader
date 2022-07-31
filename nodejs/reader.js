import "dotenv/config";
import { start, stop, get_manager } from "./libp2p-node.js";

import uuid4 from "uuid4";
import { pipe } from "it-pipe";

import { fromString } from "uint8arrays/from-string";
import { toString } from "uint8arrays/to-string";

var fake_interval = undefined;

const dial = async (node, protocol) => {
  let manager = await get_manager();
  if (manager) {
    const { stream } = await node.dialProtocol(manager, `/${protocol}/1.0.0`);
    return stream;
  }
  return undefined;
};

const pipe_wrapper = async (input, stream, callback) => {
  pipe([fromString(input)], stream, async function (source) {
    for await (const data of source) {
      await callback(data);
    }
  });
};

const init_fake = async (node) => {
  if (!fake_interval) {
    fake_interval = setInterval(async () => {
      let message = {
        value: uuid4(),
        timestamp: new Date().getTime(),
      };
      console.debug("Simulate capture", message);
      if (await get_manager()) {
        await triggers.phonendo_manager.capture(node, message);
      } else {
        console.warn("phonendo_manager unavailable. Capture will be lost");
      }
    }, 5000);
  }
};

const phonendo_manager = {
  connect: () => {},

  capture: async (node, message) => {
    if (await get_manager()) {
      await pipe_wrapper(
        JSON.stringify(message),
        await dial(node, "capture"),
        async (data) => {
          console.log(
            `Capture ${
              toString(data) == "true" ? "stored" : "discarded"
            } by manager`
          );
        }
      );
    }
  },
};

const triggers = {
  phonendo_manager,
};

start(init_fake).then().catch(console.error);

const exit = async () => {
  await stop();
  process.exit(0);
};

process.on("SIGTERM", exit);
process.on("SIGINT", exit);
