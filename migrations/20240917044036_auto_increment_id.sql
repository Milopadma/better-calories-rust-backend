-- Create sequences
CREATE SEQUENCE IF NOT EXISTS users_id_seq;
CREATE SEQUENCE IF NOT EXISTS days_id_seq;
CREATE SEQUENCE IF NOT EXISTS meals_id_seq;
CREATE SEQUENCE IF NOT EXISTS food_items_id_seq;
CREATE SEQUENCE IF NOT EXISTS meal_food_items_id_seq;

-- Modify users table
ALTER TABLE users
ALTER COLUMN id SET DEFAULT nextval('users_id_seq'),
ALTER COLUMN id SET NOT NULL;

-- Modify days table
ALTER TABLE days
ALTER COLUMN id SET DEFAULT nextval('days_id_seq'),
ALTER COLUMN id SET NOT NULL;

-- Modify meals table
ALTER TABLE meals
ALTER COLUMN id SET DEFAULT nextval('meals_id_seq'),
ALTER COLUMN id SET NOT NULL;

-- Modify food_items table
ALTER TABLE food_items
ALTER COLUMN id SET DEFAULT nextval('food_items_id_seq'),
ALTER COLUMN id SET NOT NULL;

-- Modify meal_food_items table
ALTER TABLE meal_food_items
ALTER COLUMN id SET DEFAULT nextval('meal_food_items_id_seq'),
ALTER COLUMN id SET NOT NULL;

-- Set the sequences to the current maximum value
SELECT setval('users_id_seq', COALESCE((SELECT MAX(id) FROM users), 1));
SELECT setval('days_id_seq', COALESCE((SELECT MAX(id) FROM days), 1));
SELECT setval('meals_id_seq', COALESCE((SELECT MAX(id) FROM meals), 1));
SELECT setval('food_items_id_seq', COALESCE((SELECT MAX(id) FROM food_items), 1));
SELECT setval('meal_food_items_id_seq', COALESCE((SELECT MAX(id) FROM meal_food_items), 1));