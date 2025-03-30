# SaturnClient API

The api for AuraClient which is a Minecraft mod Client

# Handshake
In order to communicate with the server, you will have to make a handshake, here is how
```
<MOJANG-SESSION-TOKEN>
```
The session token is stored on your minecraft client when it communicates with Mojang servers, as you might understand, this token is a private token used for authentication with Mojang servers, Why do we need it? Because our server needs to understand that whoever logs in with SaturnClient is you and when you make changes in your client, the changes get synced with others, Example: Changing Cloak.

# Protocol base
Every request should be only one line

```
method@key1=value1@key2=value2
```

# Error handling
request
```
invalid-method
```
response
```
!Invalid method
```
On a response `!` means it's an error.

# Methods
### `set_cloak`
- `cloak` -> `string`
