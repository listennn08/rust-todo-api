CREATE TABLE users (
           id INTEGER PRIMARY KEY AUTOINCREMENT,
     user_name VARCHAR(50) NOT NULL,
      password TEXT NOT NULL,
         email VARCHAR(100),
          role VARCHAR(10) DEFAULT 'user',
  salt_version INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

