# Football Simulator 
[![Build Status](http://drone.zoxexivo.com/api/badges/ZOXEXIVO/football-simulator/status.svg)](http://drone.zoxexivo.com/ZOXEXIVO/football-simulator)

Attempt to implement Sigames Football Manager simulation engine without manual control.

**[football-simulator.org](https://football-simulator.org)**

It is **not gamable** right now and I need a lot of work to make it workable.

Currently, most of elements are stubs, but I change it step by step

Currently available nation for simulation - **Russia**

#### How to run?

1) Local run
```console
// run frontend (Angular)
cd football-simulator/ui
npm start
...
// run backend
cd football-simulator
cargo run
...
open chrome at http://localhost:18000
```
2) Run in Docker
```console
cd football-simulator
docker build -f .\build\Football.Dockerfile -t football-simulator .
docker run -d -p 18000:18000 --name football-simulator football-simulator

open chrome at http://localhost:18000
```

[Player page example](https://football-simulator.org/teams/spartak-moscow/players/1)

![alt text](docs/images/player.jpg "Player page")

[Club page example](https://football-simulator.org/teams/spartak-moscow)

![alt text](docs/images/club.jpg "Club page")

[League page example](https://football-simulator.org/leagues/tinkoff-premier_league)

![alt text](docs/images/league.jpg "League page")

[Match page example](https://football-simulator.org/match/tinkoff-premier_league/2024-07-06_1_24)

![alt text](docs/images/match.jpg "Match page")

### License

Apache License 2.0