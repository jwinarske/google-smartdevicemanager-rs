# google-smartdevicemanager-rs

Rust code to access the Google Smart Device Management API.  This current version uses Tonic.  Tonic could be replaced with hyper/hyper-tls, request, or activ_web, then use serde_json to serialize/deserialize JSON.  This would also eliminate the protobuf deps.  For now it works.

The Google Smart Device Management API enables Read/Write parameters of:
* Nest Cameras
* Nest Doorbells
* Nest Thermostats
* Google Hub (Display)

It also allows getting events from above devices.

For more details see:
https://developers.google.com/nest/device-access/api

### Pre-requisites

1. Register project: https://developers.google.com/nest/device-access/registration
*Save Project ID to export as variable*
   
2. Authorize project: https://developers.google.com/nest/device-access/authorize 

3. Copy OAuth 2.0 file to local machine

### Required Environmental Variables

This file is used to cache refresh token

     export GOOGLE_APPLICATION_TOKEN_STORAGE=$HOME/.secrets/tokenstorage.json

This file is downloaded via https://console.developers.google.com/apis/credentials:

     export GOOGLE_APPLICATION_CREDENTIALS=$HOME/.secrets/client_secret_xxx_xxxx.apps.googleusercontent.com.json

This value comes from registering project:

     export PROJECT_GUID={project-id}
