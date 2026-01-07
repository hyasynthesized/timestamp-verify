# Timestamp Verifier
A web app that allows users to create a verifiable timestamp using HMAC codes. Built for the UNDERTALE speedrunning community to allow for verification of runs with multiple video segments. This is MIT licensed, so other speedrun communities can use their own versions of the tool.

Currently hosted at https://timestamp-verify.undertale.zip by @BenjaminUrquhart

# Building and running
## Docker
To run this with docker, first build the container with `docker build --tag "timestamp_verify" . --build-arg SECRET_KEY="<your secret key>"`, then run it with `docker run -d -p <port>:3000`. This will expose the web interface on whichever port 

## Local build
To build this locally, run `SECRET_KEY=<your secret key> cargo run`. This will expose the web interface at localhost:3000.

# Secret key
This app uses HMAC codes which rely on a "secret key". Anyone with knowledge of the secret key can trivially falsify timestamps. In order for the verification to be work the secret key needs to be known only by trusted individuals. As such, the secret key should be long and random, and knowledge of it should be kept to one or a few highly trusted community members.

# Verifying corrupted tokens
In the event that a user forgets to add the verification token into the speedrun.com run description, it is possible that the secret key may not be fully legible due to video compression. If this happens, there is a potential way to verify the token anyway. In order to do this, someone who knows the secret key should run `SECRET_KEY="<your secret key> cargo run --bin verify-corrupted-token`. This will prompt you for a timestamp. Give the tool the timestamp from the video, and it will respond with the correct signature. You can then compare the signature with the signature shown in the video. Because of the security implications of all verifiers knowing the secret key, it is up to moderator discretion whether to use this method or to reject offending runs.
