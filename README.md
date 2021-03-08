# TON wallet update service

### How to install:
```shell
# Clone repo
> git clone https://github.com/LazyMechanic/wussup
> cd wussup

# Install into $HOME/.cargo/bin
> cargo install --path .
```

### How to use:
1. Create `config.yaml` file with config
2. Check `example_config.yaml` file to fill your config
3. Check cli:
   ```shell
   > ./wussup --help
   wussup 0.1
   Lazy Mechanic
   
   USAGE:
   wussup [OPTIONS]
   
   FLAGS:
   -h, --help       Prints help information
   -V, --version    Prints version information
   
   OPTIONS:
   -c, --config <config>    Config path [default: config.yaml]
   ```

### How to apply migrations:
As a migration tool is used `goose`
0. Install and start Postgres
1. Install `goose` [link](https://github.com/pressly/goose)
2. Go to migrations directory 
   ```shell
   cd ./migrations
   ```
3. Start migration<br>
   Up migration:
    ```shell
    goose postgres "postgres://user:password@localhost:5432/wussup?sslmode=disable" up
    ```
   Down migration:
    ```shell
    goose postgres "postgres://user:password@localhost:5432/wussup?sslmode=disable" down
    ```