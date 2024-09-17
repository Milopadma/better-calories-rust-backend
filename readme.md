# Better Calories Rust Backend

A Rust-based backend for an application that manages meals, food items, and daily nutritional tracking.

## Table of Contents

- [Better Calories Rust Backend](#better-calories-rust-backend)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
  - [API Endpoints](#api-endpoints)
  - [Database Schema](#database-schema)

## Features

- User management
- Day tracking
- Meal logging
- Food item database
- Nutritional information tracking

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Cargo

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/better-calories-rust-backend.git
   cd better-calories-rust-backend
   ```

2. Set up the environment variables:

   ```
   cp .env.example .env
   ```

   Edit the `.env` file with your database credentials and other configuration.

3. Install dependencies:

   ```
   cargo build
   ```

4. Set up the database:
   ```
   cargo run --bin migrate
   ```

## Usage

To run the server:

```
cargo run
```

The server will start on `http://localhost:3000` by default.

## API Endpoints

For a detailed list of API endpoints and their payloads, please refer to the [endpoints.md](endpoints.md) file.

## Database Schema

The database schema includes tables for users, days, meals, food items, and meal-food item relationships. For more details, check the migration files in the `migrations/` directory.
