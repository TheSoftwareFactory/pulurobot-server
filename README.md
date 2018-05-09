# PuluRobot Server

<center><img src="readme_img_1.jpg" alt="Drawing" style="width: 300px;"/></center>

This is the main repository for the server mananging the connections from the stations and the robots.

## What it offers

- HTTP interface for stations
- HTTP interface for robots

- WebSocket connections for stations
- WebSocket connections for robots

- Database storage

## HTTP interface for stations

Offer endpoints for:
- authentication
- gather all the available robots and their status
- request a robot

Base endpoint path: `/api/v1/station`
- POST `/api/v1/station/register`: Register a new station
    ```json
    {
        "name": "STATION_1"
    }
    // application/json
    // Result: JWT
    // example: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzIn0.yKMMt4h_vv5UvFuCf3Z_zSMRYV4rEQd1r6c4WZMalyc"
    // Use it in Header, as:
    // "Authorization": JWT
    ```

- [JWT REQUIRED] POST `/api/v1/station/pinned_location/new`: Create new pinned location
    ```json
    {
        "name": "PINNED_LOCATION_1",
        "x": "10",
        "y": "16",
        "angle": "45"
    }
    // application/json
    ```

- [JWT REQUIRED] GET `/api/v1/station/pinned_location/all`: Retrieve all pinned locations
    ```json
    // Result:
    [
        {
            "id": 1,
            "name": "CHARGE_1",
            "x": 10,
            "y": 10,
            "angle": 0
        }
    ]
    ```

- [JWT REQUIRED] POST `/api/v1/station/robot/pinned_location/new`: Pin the robot to a location
    ```json
    {
        "robot_id": 1,
        "pinned_location_id": 1,
        "tag": "CHARGE_STATION"
    }
    // application/json

    // Note that tags are important for the system, since the tag "CHARGE_STATION" is used to
    // update the current robot status.
    ```

- [JWT REQUIRED] GET `/api/v1/station/robot/location/history?<params>`: Get location history of a robot
    ```json
    // params: 
        - "robot_id=integer"
    // example: "/history?robot_id=1"

    // Result:
    [
        {
            "robot_id": 2,
            "x": 10,
            "y": 15,
            "angle": 20,
            "created_at": "1970-01-01T00:00:20Z"
        }
    ]
    ```

- [JWT REQUIRED] GET `/api/v1/station/robot/all`: Get all robots regustered into the system
    ```json
    // Result:
    [
        {
            "id": 1,
            "name": "robot1",
            "status": "UNAVAILABLE",
            "created_at": "+50278-09-20T14:06:58Z"
        }
    ]
    ```

## HTTP interface for robots

Offer endpoints for:
- authentication
- request confirmation for delivery

Base endpoint path: `/api/v1/robot`
- POST `/api/v1/robot/register`: Register a new robot
    ```json
    {
        "name": "ROBOT_QWERTY_1"
    }
    // application/json
    // Result: JWT
    // example: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzIn0.yKMMt4h_vv5UvFuCf3Z_zSMRYV4rEQd1r6c4WZMalyc"
    // Use it in Header, as:
    // "Authorization": JWT
    ```

- [JWT REQUIRED] PATCH `/api/v1/robot/battery/level`: Update battery level
    ```json
    15
    // text/plain
    ```

- [JWT REQUIRED] PATCH `/api/v1/robot/location`: Update current location
    ```json
    {
        "x": 10,
        "y": 15,
        "angle": 20
    }
    // application/json
    ```

## WebSocket connection for stations

Used for:
- realtime update of robots location
- delivery status
- robot statuses (availability, battery level)

Base endpoint path: `ws://localhost:3001`
- OPEN `ws://localhost:3001/JWT_HERE`: Open a new ws connection and auth as a valid station
    ```json
    // Success message: "OK"
    // Error message: "ERROR_INVALID_JWT" | "ERROR_UNAUTHORIZED"
    ```

- SUBSCRIBE TO EVENT:
    ```json
    {
        "action": "SUBSCRIBE_TO_EVENT",
        "payload": {
            "event": "LOCATION_UPDATE#1"
        }
    }

    // Success message: "OK"
    // Error message: "ERROR_MALFORMED_INPUT"

    // This subscribes to an event. When someone in the system will trigger this specific event,
    // the ws is going to receive a message from it.
    // Currently there are 3 messages supported:
    // 1) CONNECTED_ROBOT: A robot just established a connection with the server. Will yield the id of the robot
    // 2) LOCATION_UPDATE#<id>: Location updates from a specific robot
    // 3) BATTERY_LEVEL_UPDATE#<id>: Battery level updates from a specific robot

    // <id> is the id of the robot we want to listen to
    ```

## WebSocket connection for robots

Used for:
- receiving updates about location
- receiving updates about battery level
- sending high level commands to the robot (go to location, stop moving, ...)

Base endpoint path: `ws://localhost:3002`
- OPEN `ws://localhost:3002/JWT_HERE`: Open a new ws connection and auth as a valid robot
    ```json
    // Success message: "OK"
    // Error message: "ERROR_INVALID_JWT" | "ERROR_UNAUTHORIZED"
    ```

- UPDATE BATTERY LEVEL:
    ```json
    {
        "action": "BATTERY_LEVEL_UPDATE",
        "payload": {
            "level": 23
        }
    }

    // Success message: "OK"
    // Error message: "ERROR_MALFORMED_INPUT"
    ```

- UPDATE LOCATION:
    ```json
    {
        "action": "LOCATION_UPDATE",
        "payload": {
            "x": 10,
            "y": 15,
            "angle": 45
        }
    }

    // Success message: "OK"
    // Error message: "ERROR_MALFORMED_INPUT"
    ```

## Database storage

The database is based on sqlite, because it allows us to have a fast and complete db, that is also embeddable easily, since
it is based on a single file.

Stores informations about:
- robot status, location

## Logic inside the server

The server inferes automatically the status of the robot based on some conditions, such as current robot position,
history of immediately previous positions, presence of stations close by and last communication status with the server.
Based on all these conditions, the server flags the robot with one of these statuses:

- **AVAILABLE**: Robot powered on and waiting at a charging point
- **WAITING**: Robot available for work but not at the charging station (maybe just rebooted from failure or waiting at a cafe)
- **BUSY**: Robot currently moving and doing some work
- **UNREACHABLE**: Lost connection with the robot or powered off not at the charging station
- **UNAVAILABLE**: Robot powered off at a charging station

**Important**: When pinning a robot to a location, the tag is important for the client, but also for the server:
the tag "CHARGE_STATION" is used by the server to retrieve all pinned locations that are charging stations, and then checking
if the robot is inside one of them (this is used in the auto-updating mechanism for the robot status, explained above).


============

### My todo I was following through this project

============

*In case of future reference to some work I've done on this project and how I've managed to complete the tasks, I'll leave here the todo list I was following and adding new tasks when discovering them.*

- [x] Implement basic auth for stations and robots
- [x] Refactor robot struct to move battery level outside - Add new fields
- [x] update_battery_level should return a 200 status without any text
- [x] Add update location endpoint && insert also in robot_history_locations
    - [x] Change robot status to "BUSY" since it's moving
    - [x] Detect if the location the robot is currently is inside the pinned_locations, and if so update the robot           according to that
- [x] Write endpoint for adding new pinned_locations
- [x] Fix registration for stations
- [x] Write endpoint for retrieving all the robot location history
- [x] Write endpoint for retrieving all robots
- [x] Write cyclic function that verifies that the robot is working properly. If not, update robot status
- [x] Change times with time crate and Timespec
- [x] Write ws for robot
- [x] Write ws for station
- [x] Write endpoint for pinning the robot to a location (create charging station)
- [x] Endpoint for all pinned locations
- [x] Maintain pool of connected stations (subscribed to a specific event), and when the event is triggered, 
    despatch it to all the interested parties
- [ ] Maintain pool of connected robots and when a station sends a command to a robot 
    (stop, go to charge point, to go point b), it dispatches to the correct robot the event

*As you can see, only the last point is missing, unfortunately. In case you want to tackle the last one, you can base it on the dispatcher/event system I've already built.
The main goal of the last point is to have more control over the robot, and possibly coordinate multiple robots through a centralized system, such as the server itself.*