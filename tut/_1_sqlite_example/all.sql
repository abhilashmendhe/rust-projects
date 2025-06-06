BEGIN TRANSACTION;
DROP TABLE IF EXISTS "pets";
CREATE TABLE IF NOT EXISTS "pets" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL
);
DROP TABLE IF EXISTS "people";
CREATE TABLE IF NOT EXISTS "people" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL
);

DROP TABLE IF EXISTS "pet_owners";
CREATE TABLE IF NOT EXISTS "pet_owners" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "person_id" INTEGER NOT NULL,
    "pet_id" INTEGER NOT NULL,
    FOREIGN KEY("person_id") REFERENCES "people",
    FOREIGN KEY("pet_id") REFERENCES "pets",
    UNIQUE("person_id","pet_id")
);

INSERT INTO "pets" VALUES (1, "Seedu");

INSERT INTO "people" VALUES (1, "Alice");
INSERT INTO "people" VALUES (2, "Bob");

INSERT INTO "pet_owners" VALUES (1,1,1);
COMMIT;