# google-jwt-auth
Generate authentication tokens for Google api requests.

See [Using OAuth 2.0 for Server to Server Applications](https://developers.google.com/identity/protocols/oauth2/service-account?hl=en#httprest) for a description of the procedure.

## Setup
To obtain a token, some tasks need to be done:

- Access to the Google console.
- Access to (create) a Google service account.
- Rights to create a key for that service account.
- Access to the json key file (can be downloaded during the service account key generation).
- Select the needed api usage from this [Website](https://developers.google.com/identity/protocols/oauth2/scopes?hl=en). More than one usage can be used by seperating them with commas. An empty usage results in an error.

This json file is important and necessary to use this crate. See in Examples/Usage its usage.

Each token request has a lifetime. This lifetime need to be provided as param in seconds. Allowed are values between 30 to 3600.

## Current Version: 0.0.1 (ALPHA)
## To be done

The next updates take care of the following:

* [ ] Improved error messages
* [ ] Detect error returns
* [ ] Usage Enum Types
* [ ] Token-Buffer (for now: every token generation results in a rest request)

## Example / Usage

```rust
...
let config = AuthConfig::build(                                     //<-- This config struct can be reused to generate tokens
    fs::read_to_string("service_account.json").unwrap(),            //<-- JSON as String
    String::from("https://www.googleapis.com/auth/cloud-vision"),   //<-- Api-Usage
).unwrap(); 
let token = config.generate_auth_token(3600).await.unwrap();         //<-- Generate token
println!("{}", token);
...
```