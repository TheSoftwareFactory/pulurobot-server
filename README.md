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

## HTTP interface for robots

Offer endpoints for:
- authentication
- request confirmation for delivery

## WebSocket connection for stations

Used for:
- realtime update of robots location
- delivery status
- robot statuses (availability, battery level)

## WebSocket connection for robots

Used for:
- receiving updates about location
- receiving updates about battery level
- sending high level commands to the robot (go to location, stop moving, ...)

## Database storage

Stores informations about:

- robot status, location