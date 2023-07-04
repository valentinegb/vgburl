# vgburl

A simple personal URL shortener backend.

## How to use

To shorten a link, you can use something like `curl` to POST the long link.

```zsh
curl https://vgburl.shuttleapp.rs -d "https://example.com/a/really/very/extremely/long/link/that/is/too/long/to/reasonably/type"
```

If successful, you'll get a HTTP OK along with a 9-digit code in the body.
You can then use the code as a route which will redirect to the long link.

```text
https://vgburl.shuttleapp.rs/abcdefghi
```
