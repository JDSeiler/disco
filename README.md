# Disco: Passwords With a Beat
A password generator focused on producing readable output that is easy to hand-type.

The password format is inspired by [1Password's](https://1password.com/password-management)
"Smart Password" format. Passwords are broken into chunks where each chunk is comprised 
exclusively with uppercase or lowercase letters. Chunks are then separated by numbers or
special characters. For example:
```
TRM%aix$oky3ozw=awr
```
I find this chunked format much easier to type out and check for errors.

## Why Does This Exist?
I occasionally find myself in situations where I am only allowed to use the password manager
built into Chrome. This is highly unfortunate because Chrome's password manager is not very
good. Specifically, it doesn't include the ability to... y'know... generate a password...

To solve this highly specific pet peeve of mine, and because I wanted a side project,
I wrote Disco. Ultimately, I'd be just as well off with `openssl rand -base64` and
inserting `_` every ~5 characters. But what's the fun in that?

