-- /* This is for Postgres only MySQL would be different */
-- tag::enums[]
CREATE TYPE media_enum AS ENUM ('image', 'video', 'unknown');

CREATE TYPE location_enum AS ENUM ('s3', 'local');

CREATE TYPE media_audience_enum AS ENUM ('personal', 'friends'' family');
-- end::enums[]

CREATE TABLE media_datas (
  id UUID PRIMARY KEY,
  name VARCHAR NOT NULL,
  note VARCHAR NULL,
  size INT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 't',
  location VARCHAR NOT NULL,
  device_id UUID NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
-- tag::media_datas_with_enum[]
  media_type media_enum NULL,
  location_type location_enum NOT NULL,
  media_audience_type media_audience_enum[] NULL
-- end::media_datas_with_enum[]
);