# Server Dashboard API

## Libraries / Crates
* sqlx - Driver to interact with postgresql database
* actix_web - http crate to make api endpoints

## Authentication

### Functionnalities 
* [x] Log in
* [ ] Verify token
* [ ] Log out
* [ ] Log out from all devices

Authentication is made with sessions, when user log in we generate a token from random 20 bytes string encrypted in base32. The token is then gave back to the user.

### Database  
```
users
    - id
    - username
    - email
    - password

user_session
    - id 
    - token
    - expires_at
    - user_id
```

### Code

First we check if the email and password provided match a user in the database, if not giving back a ``401 Unauthorized`` error to the user.

If the user exist and the password correct then we generate 20 random bytes and transform the resulting string to base32. We then generrate the session and set an expires_at to ``3hours later``.
We then store the session to the ``user_session`` table.

Lastly we return the token to the user in a response containing only the token : 
```json
{"token": "<TokenHere>"}
```
