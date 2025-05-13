# Note Taker

A simple note-taking application built in Rust with PostgreSQL for persistence. This app allows you to create, view, and manage notes. Each note contains a title, content, and the date/time when it was taken. This repository is configured with Docker Compose to allow you to run the app without needing to install Rust or PostgreSQL locally.

## Features
- Create and store notes with a title, content, and timestamp.
- View all notes.
- Persistent storage using PostgreSQL.
- Run the project with Docker without needing to install Rust or PostgreSQL on your system.

## Tech Stack
- **Backend:** Rust
- **Database:** PostgreSQL
- **Containerization:** Docker

## Getting Started

### Prerequisites
- Docker
- Docker Compose

### Running the Application

   Clone this repository to your local machine and run it using the following command:

   ```bash
   git clone https://github.com/yourusername/note_taker.git
   cd note_taker
   docker-compose up --build
    