## Start Up

### Requirements

`docker` needs to be installed

`sqlx` also should be installed
this can be done with `cargo install sqlx-cli`

### Steps To Get Going

    1. cat .env.sample > .env   // copy over .env.sample to .env
    2. docker compose up -d --build  // spin up local and test databases
    3. sqlx database create // creates the database in case it isn't theres
    4. sqlx migrate run // perform migrations
    5. cargo build
    6. cargo run -- --enable_client --print-logs // the cli flags turns on the code to spawn clients, and print database calls

### Testing

The test database needs to be running for the test command to work properly work

`cargo test --test '*'`

### Local Site Testing

Below are sample cURLs for testing the localhost (assuming the port is 8020, same as the .env file)
NOTE: since it's on deployed on a free plan, expect 1-2min delay on first call when server is cold

##### create item

```
curl --request POST \
  --url http://localhost:8020/api/tables/1/items \
  --header 'Content-Type: application/json' \
  --data '{
	"items": [
		{"cook_time": 5},
		{"cook_time": 10}
	]
}'
```

##### get items

```
curl --request GET \
  --url http://localhost:8020/api/tables/1/items
```

##### get item

```
curl --request GET \
  --url http://localhost:8020/api/tables/1/items/1
```

##### delete item

```
curl --request DELETE \
  --url http://localhost:8020/api/tables/1/items/1
```

### Live Site

You can find the site right now on
https://rust-ucsq.onrender.com

To test the site live, below are some cURLs for testing

##### create item

```
curl --request POST \
  --url https://rust-ucsq.onrender.com/api/tables/1/items \
  --header 'Content-Type: application/json' \
  --data '{
	"items": [
		{"cook_time": 5},
		{"cook_time": 10}
	]
}'
```

##### get items

```
curl --request GET \
  --url https://rust-ucsq.onrender.com/api/tables/1/items
```

##### get item

```
curl --request GET \
  --url https://rust-ucsq.onrender.com/api/tables/1/items/1
```

##### delete item

```
curl --request DELETE \
  --url https://rust-ucsq.onrender.com/api/tables/1/items/1
```

### Dev Notes

#### Deployment

It is deployed currently on render.io, and the database on railway.app for ease of deployment.

In an environment which uses K8S, I'd containerize the application, and use a multi-stage build Dockerfile to minimize the image size.

In an environment which is already using AWS, dropping the binary in an EC2 instance probably works well.

#### Migrations

Migrations are run out of band. This was primarily for ease, to get going in a more timely manner. Figuring out the Dockerfile and embedding the migrations in the app was proving to be difficult while trying to deploy.

#### Table Seeding

This restaurant's app assumed amount of tables seeded is 100.
