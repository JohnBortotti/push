# Push
This project is designed to be a simple and unified notification system for my homelab. It can be used by any application 
running in the homelab to generate notifications, alerts, and reports. Feel free to extend and 
customize it to suit your specific needs and preferences, the project is absolutely open to contributions.

## Project Overview
The project consists of two main components: ´notifications-api-publisher´ and ´notifications-api-consumer´. 
These components are located in separate directories: "notifications-api-consumer" and "notifications-api-publisher", 
and each has a Dockerfile for building the respective images. You can use the provided docker-compose.yml 
file at the root of the project to build and run the entire system.

### notifications-api-publisher
The notifications-api-publisher is a straightforward HTTP API built with the Rocket framework. 
It exposes a single endpoint, POST /notify, which accepts JSON data in the following format:
´´´json
{
  "title": "title string",
  "description": "description string",
  "category": "Alert", "Critical", or "Report"
}
´´´
When a request is made to this endpoint, the API publishes a message to the RabbitMQ queue. 
The configuration for RabbitMQ is managed via environment variables set in the Docker Compose file.

### notifications-api-consumer
On the other side of the project, the notifications-api-consumer consumes messages from the RabbitMQ queue. 
It uses the SendGrid HTTP API to send notification emails. All necessary credentials for SendGrid are configured via 
environment variables in the Docker Compose file.

## Getting Started
To run this project, follow these steps:

- Configure the environment variables in the docker-compose.yml file to match your requirements. 
This includes setting up the necessary credentials for SendGrid (api-key).

- Ensure that you have a running RabbitMQ instance accessible to the project. 

- With the environment variables properly configured, use the provided docker-compose.yml to start the project:
´´´bash
docker compose up
´´´

- The project is now up and running. You can start sending requests to the notifications-api-publisher endpoint, 
and the notifications-api-consumer will process them and send out notification emails accordingly
