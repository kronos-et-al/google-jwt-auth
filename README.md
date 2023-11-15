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

## Current Version: 0.1.1 (BETA)
## Current State: Stable!
## To be done

The next updates take care of the following:

* [X] Improved error messages
* [X] Detect error returns
* [X] Usage Enum Types
* [ ] Token-Buffer (for now: every token generation results in a rest request)

## Example / Usage

```rust
...
let config = AuthConfig::build(                                     //<-- Config can be reused
    fs::read_to_string("service_account.json").unwrap(),            //<-- JSON as string
    &Usage::CloudVision,                                            //<-- Api-Usage
).unwrap(); 
let token = config.generate_auth_token(3600).await.unwrap();        //<-- Generate token
println!("{}", token);
...
```

## Errors

This chapter should help to determine who/which causes the problem and how to fix it:

| LibError            | Type          | Solution (not guaranteed)                                                                                                                           |
|---------------------|---------------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| AuthenticationError | invalid_grant | Your service-client.json is no longer valid as the key got deleted in the google console. Replace this file with a new one by generating a new key. |

This table is WIP and will change if new errors occur or someone requests a related issue.