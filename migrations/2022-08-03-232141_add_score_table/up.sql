CREATE TABLE scores (
  id serial PRIMARY KEY,
  user_id serial NOT NULL,
  score smallint NOT NULL,
  created_at timestamp NOT NULL DEFAULT now(),
  updated_at timestamp NOT NULL DEFAULT now()
);