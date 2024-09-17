# API Endpoints

## User Endpoints

### Create User

- **Method:** POST
- **Path:** /
- **Payload:**
  ```json
  {
    "username": "johndoe",
    "email": "john@example.com",
    "password": "securepassword123"
  }
  ```

### Get All Users

- **Method:** GET
- **Path:** /

### Get User by ID

- **Method:** GET
- **Path:** /:id

## Day Endpoints

### Create Day

- **Method:** POST
- **Path:** /days
- **Payload:**
  ```json
  {
    "user_id": 1,
    "date": "2024-03-17"
  }
  ```

### Get Day by ID

- **Method:** GET
- **Path:** /days/:id

## Meal Endpoints

### Create Meal

- **Method:** POST
- **Path:** /meals
- **Payload:**
  ```json
  {
    "day_id": 1,
    "name": "Breakfast"
  }
  ```

### Get All Meals

- **Method:** GET
- **Path:** /meals

### Get Meal by ID

- **Method:** GET
- **Path:** /meals/:id

## Food Item Endpoints

### Create Food Item

- **Method:** POST
- **Path:** /food_items
- **Payload:**
  ```json
  {
    "name": "Apple",
    "calories": 95,
    "protein": 0,
    "carbohydrate": 25,
    "fat": 0,
    "sugar": 19
  }
  ```

### Get Food Item by ID

- **Method:** GET
- **Path:** /food_items/:id
