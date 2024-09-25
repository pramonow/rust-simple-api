# rust-simple-api

Simple rust api example with Axum and mysql library
Provides 3 api

Initialize your local database and use `cargo run` to run the app

1. Hello World
`
curl http://localhost:3000/
Hello, world!
`

2. Random
`
curl http://localhost:3000/random\?start=5\&end=50
Random number 
`

3. Employee (connecting to db)
`
curl http://localhost:3000/employee\?id=1         
[Employee { id: 1, name: "Sauron" }]
`
