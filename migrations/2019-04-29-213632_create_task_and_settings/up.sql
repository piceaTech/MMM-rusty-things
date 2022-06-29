CREATE TABLE tasks (
  uuid TEXT PRIMARY KEY NOT NULL,
  title VARCHAR NOT NULL,
  trashed INTEGER,
  start INTEGER,
  status INTEGER,
  type INTEGER,
  todayIndexReferenceDate REAL,
  todayIndex INTEGER,
  "index" INTEGER,
  stopDate REAL
);
create TABLE meta (
  key VARCHAR Not Null,
  value VARCHAR Not Null
);