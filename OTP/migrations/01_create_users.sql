CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email varchar UNIQUE NOT NULL,
    password varchar NOT NULL ,
    phone varchar NOT NULL
);

INSERT INTO users (id, email, password, phone) VALUES
    ('00000000-0000-0000-0000-000000000001', 'abc@gmail.com',   '!@#$%',   '+919000540262');