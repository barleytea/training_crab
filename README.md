# Training Crab

[![CircleCI](https://circleci.com/gh/barleytea/training_crab.svg?style=svg)](https://github.com/barleytea/training_crab/commits)

## set up develop environment

### 1. add configuration and credentials

```
.
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── LICENSE
├── README.md
├── configuration
│   ├── base.yaml <-
│   └── local.yaml <-
├── credentials
│   └── develop
│       └── ${FIREBASE_APP_NAME}-firebase-adminsdk-xxxxx-xxxxxxxxxx.json <-
├── database
│   └── .env <-
```

#### 2.1. example base.yaml

```
application:
  port: 8088
  host: 0.0.0.0
database:
  uri: "mongodb://mongo:27017/"
  user: "${USER_NAME}"
  password: "${PASSWORD}"
  name: "${DATABASE_NAME}"
firebase:
  secret_path: "credentials/develop/${FIREBASE_APP_NAME}-firebase-adminsdk-xxxxx-xxxxxxxxxx.json"
```

### 2.2 example database/.env
```
MONGO_INITDB_ROOT_USERNAME=${ROOT_USERNAME}
MONGO_INITDB_ROOT_PASSWORD=${ROOT_PASSWORD}
MONGO_INITDB_DATABASE=${DATABASE_NAME}
```

### 3. start server
```
docker-compose up -d --build
```
