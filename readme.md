A program to change a users description on https://roblox.com.
It uses the rust HTTP library hyper and a HTTPS connector called hyper-tls.
It has a configurable example struct which uses composition with the main struct. It changes your description every n seconds to the time till my birthday given in unix millis. You can configure it to display seconds, minutes, hours or days.

You need to give the authencation cookie in the request to log in to the api each request. This can be set by an environment variable (.env files are supported). The variable's name is `COOKIE`.

Everything is configured and compile time minus the cookie; this is because I can't be bothered to add runtime configuration.
