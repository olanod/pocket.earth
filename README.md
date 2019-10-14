# pocket.earth

An experimental attempt at representing OSM data in lighter weight formats for constrained environments like the mobile web.

## How
The initial experiment revolves around representing OSM data in a similar way to how the traditional [protobuf format](https://wiki.openstreetmap.org/wiki/PBF_Format) does, but removing not frequently used metadata like the `Info` type and using [S2](https://s2geometry.io/) lv30 cell ids to replace OSM node ids + latitude and longitude data. Also besides caring about size [Cap'nProto](https://capnproto.org/) and Rust tooling are used to maximize portability, memory usage and performance.

## Ideas
Representing world data in a more optimized way and provide tooling is already a great start for anyone who wants to process OSM data quickly, but more can be done in the way this data is represented and distributed. I'm particularly interested in exploiting the herarchical features of the S2 cells to achive routable tiles and different ways of distributing map data in a decentralized fashion, likely using IPFS(the way [pocket.earth](https://pocket.earth) landing page is _hosted_) and standardized file names and folder structures.
