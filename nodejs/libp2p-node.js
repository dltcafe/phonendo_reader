import "dotenv/config";

import { createLibp2p } from "libp2p";
import { TCP } from "@libp2p/tcp";
import { Noise } from "@chainsafe/libp2p-noise";
import { Mplex } from "@libp2p/mplex";
import { MulticastDNS } from "@libp2p/mdns";

import { pipe } from "it-pipe";

import { toString } from "uint8arrays/to-string";
import { fromString } from "uint8arrays/from-string";

var manager = undefined;
var addresses = new Set();

const node = await createLibp2p({
  addresses: {
    listen: [`/ip4/127.0.0.1/tcp/0`],
  },
  transports: [new TCP()],
  connectionEncryption: [new Noise()],
  streamMuxers: [new Mplex()],
  peerDiscovery: [
    new MulticastDNS({
      interval: 20e3,
    }),
  ],
});

const get_manager = async () => {
  if (manager) {
    try {
      await node.ping(manager);
    } catch (err) {
      // If ping launches an exception the node is down or unavailable
      manager = undefined;
    }
  }
  return manager;
};

const connect = async (id) => {
  if (!addresses.has(id.toString())) {
    addresses.add(id.toString());
    if (!manager) {
      let aux_stream = undefined;
      try {
        const { stream } = await node.dialProtocol(id, "/discover/1.0.0");
        aux_stream = stream;
      } catch (err) {
        // discover protocol unsupported
      }
      if (aux_stream) {
        await pipe(
          [fromString("discover")],
          aux_stream,
          async function (source) {
            for await (const data of source) {
              let node_type = toString(data);
              if (node_type === "phonendo_manager") {
                console.log(`Added ${node_type} peer ${id.toString()}`);
                manager = id;
              }
            }
          }
        );
      }
    }
  }
};

const start = async (callback) => {
  await node.start();

  node.getMultiaddrs().forEach((addr) => {
    console.log("Listening on", addr.toString());
  });

  node.addEventListener("peer:discovery", async (peerData) => {
    await connect(peerData.detail.id);
  });

  callback(node);
};

const stop = async () => {
  await node.stop();
  console.log(`${process.env.SERVICE_NAME} has stopped`);
};

export { start, stop, get_manager };
