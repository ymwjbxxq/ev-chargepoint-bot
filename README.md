# Telegram Bot for finding EV charger nearby

This project is a demo of using telegram BOT in conjunction with Lambda Function Url. The SAM template deploys a Lambda function reacting to Telegram BOT commands.

## Requirements

You must have a [PlugShare](https://developer.plugshare.com/) access key. PlugShare is a free EV driver's app for iOS, Android, and the web, allowing users to find charging stations, leave reviews, and connect with other plug-in vehicle owners. It has many charging points worldwide, with stations from every major network in North America, Europe, and much of the globe. As a result, PlugShare is also home to the world's largest community of EV drivers.

**ATTENTION** without the access key, you cannot use PlugShare API

## Note

In different languages, you can find many [Bot Code examples](https://core.telegram.org/bots/samples).
About Rust, I tried two, and they did not work for ARM64, so because I wanted to do a simple demo, I have implemented my wrapper around the Telegram API (it is just a POST). Of course, I spent time writing all the DTOs needed for the requests, but it was an excellent exercise.

## Setup 

1. [Create telegram bot](https://core.telegram.org/bots#3-how-do-i-create-a-bot)
2. Deploy The SAM template
3. Hook your lambda url to the BOT using this URL https://api.telegram.org/bot{BOT_TOKEN}/setWebhook?url={LAMBDA_URL}

## How it works

https://user-images.githubusercontent.com/78874812/164454941-bee00de4-ad1f-4b3a-a40f-5c500d207749.mov

The API could return based on the location selection and empty values, so I default to Unknown.
Tracking and Stop commands are not implemented. These commands are sending Telegram messages. In theory, I could save it into Dynamodb, the station selected and with a scheduler, ping the EV API until the status moves to AVAILABLE and send a notification to the chat.

## Deployment Instructions

1. Create a new directory, navigate to that directory in a terminal and clone the GitHub repository:

    ``` 
    git clone https://github.com/ymwjbxxq/ev-chargepoint-tracker
    ```
    
2. Change the directory to the pattern directory:
    ```
    cd ev-chargepoint-tracker
    ```
3. Install dependencies and build:
    ```
    make build
    ```
4. From the command line, use AWS SAM to deploy the AWS resources for the pattern as specified in the template.yml file:
    ```
    make deploy
    ```
5. During the prompts:
    * Enter a stack name
    * Enter the desired AWS Region
    * Allow SAM CLI to create IAM roles with the required permissions.

6. Note the outputs from the SAM deployment process. These contain the resource names and/or ARNs used for testing. 

### Cleanup ###
```
make delete
```
