# Readme

The directories here each contain examples showcased in the book. If you have the [just](https://just.systems/) tool installed, you can launch any of the examples using

```shell
just <recipe>
```

To see the list of all available (_just recipes_) run

```shell
just --list
```

Alternatively, you can use cargo to run the examples directly using something like

```shell
cd syncing/<cratename>
cargo run --bin <cratename>
```


### rust-analyzer vscode extention
To enable rust-analyzer features for a rust example in the project you must add the path to the `Cargo.toml` file for the example to `.vscode/settings.json`