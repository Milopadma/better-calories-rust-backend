-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

-- Create days table
CREATE TABLE IF NOT EXISTS days (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    date DATE NOT NULL,
    UNIQUE(user_id, date)
);

-- Create meals table
CREATE TABLE IF NOT EXISTS meals (
    id BIGSERIAL PRIMARY KEY,
    day_id BIGINT NOT NULL REFERENCES days(id),
    name TEXT NOT NULL
);

-- Create food_items table
CREATE TABLE IF NOT EXISTS food_items (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    calories INTEGER NOT NULL,
    protein INTEGER NOT NULL DEFAULT 0,
    carbohydrate INTEGER NOT NULL DEFAULT 0,
    fat INTEGER NOT NULL DEFAULT 0,
    sugar INTEGER NOT NULL DEFAULT 0
);

-- Create meal_food_items table
CREATE TABLE IF NOT EXISTS meal_food_items (
    id BIGSERIAL PRIMARY KEY,
    meal_id BIGINT NOT NULL REFERENCES meals(id),
    food_item_id BIGINT NOT NULL REFERENCES food_items(id),
    quantity INTEGER NOT NULL
);