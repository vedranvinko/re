# r(edirection) e(ngine)

Usage:

re.exe -i `<inputfile>` -c `<configfile>` where `inputfile` contains list of redirections, and `configfile` is a `Toml` file with two fields

```Toml
delimiter = ","
url = "https://example.org"
```

Delimiter could be `,.:;` etc, or some other String (it uses Rust's `split` function, so whatever Rust can handle, could be fair play)
