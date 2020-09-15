
CREATE TABLE video_metadatas (
  id SERIAL PRIMARY KEY,

  video_duration numeric null,
  video_width numeric null,
  video_height numeric null,
  video_codec varchar null,
  audio_track_id numeric null,
  audio_codec varchar null,

  media_item_id UUID NOT NULL references media_datas(id),

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
