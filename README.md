# Open Football

[![Build Status](http://drone.zoxexivo.com/api/badges/ZOXEXIVO/open-football/status.svg)](http://drone.zoxexivo.com/ZOXEXIVO/open-football)

Attempt to implement Sigames Football Manager simulation engine without manual control.

**[open-football.org](https://open-football.org)**

It is **not gamable** right now and I need a lot of work to make it workable.

Currently, most of elements are stubs, but I change it step by step

Currently available nation for simulation - **Russia**, **Italy**, **England**

#### How to run?

1) Local run

```console
// run frontend (Angular)
cd ui
npm install --force
npm start
...
// run backend
cargo run
...
open chrome at http://localhost:18000
```

2) Run in Docker

```console
cd open-football
docker build -f .\build\Football.Dockerfile -t open-football .
docker run -d -p 18000:18000 --name open-football open-football

open chrome at http://localhost:18000
```

[Match page example (click on any goals)](https://open-football.org/leagues/english-premier-league)

![alt text](docs/images/match.jpg "Match page")

[Player page example](https://open-football.org/teams/spartak-moscow/players/1)
![alt text](docs/images/player.jpg "Player page")

[Club page example](https://open-football.org/teams/spartak-moscow)

![alt text](docs/images/club.jpg "Club page")

[League page example](https://open-football.org/leagues/tinkoff-premier_league)

![alt text](docs/images/league.jpg "League page")

### License

Apache License 2.0