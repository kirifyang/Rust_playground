# Rust_playground

Playground for Clickhouse.
Which is based on **[klickhouse](https://github.com/Protryon/klickhouse/)** and **[actix-web](https://github.com/actix/actix-web)** library.

## Process

- **Frontend [not start]**

- **Backend [built the backbond]**

## Files structure

```text
Project                      
├─ config                    
│  ├─ build_db.sh            
│  ├─ clickhouse_config.xml  
│  └─ setting.sh             
├─ src                       
│  ├─ db                     
│  │  ├─ file.rs             
│  │  ├─ mod.rs              
│  │  ├─ ops.rs              
│  │  └─ schema.rs           
│  ├─ macro.rs               
│  └─ main.rs                
├─ Cargo.lock                
├─ Cargo.toml                
├─ README.md                 
├─ prejob.sh                 
└─ run.sh                    
```

## Features

- [x] Basic Clickhouse Operation
  - [x] Connect
  - [x] Create Table
    - [x] Job Table
    - [x] UserData Table
  - [x] Insert Data
  - [x] Query Data

- [ ] Actix-web Server

## How to Use

Run the following script for environment configuration at first time：

```bash
bash setting.sh
cargo run
```

Then, you can start the client to check the result:

```bash
./clickhouse/clickhouse client
```
