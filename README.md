# Dockerized Todo App with React, Chakra UI, and Rust

This repository hosts a full-stack Todo application combining modern front-end and back-end technologies. The app leverages React, Chakra UI, Rust, and Docker to deliver a performant and visually appealing Todo List solution.


### Technologies used
1. Frontend - React + Chakra UI
2. Backend - Rust
3. Docker and Docker Compose 
4. Database - MongoDB

### Getting Started

#### Step 1: Clone the repository 

``` sh
git clone https://github.com/your-username/dockerized-todo-app.git
cd dockerized-todo-app
```

#### Step 2: Run the App with Docker Compose

``` sh
docker-compose up --build
```

#### The Docker Compose command will perform the following

* Build and run the Rust backend API on localhost:8000
* Build the React frontend and server it on localhost:3000
* Set up MongoDB on localhost:27017
* Open your browser and navigate to http://localhost:3000 to access the application.