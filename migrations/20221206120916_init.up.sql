-- Add up migration script here
CREATE TABLE "drinks" (
    "id" INTEGER PRIMARY KEY NOT NULL,
    "duration" INTEGER DEFAULT 60
);

CREATE TABLE "dreams" (
    "id" INTEGER PRIMARY KEY NOT NULL,
    "duration" INTEGER DEFAULT 60,
    "message" TEXT DEFAULT 'Denk an deine Traumroutine!'
)
