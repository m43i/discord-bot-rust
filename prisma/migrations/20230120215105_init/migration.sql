-- CreateTable
CREATE TABLE "dreams" (
    "id" BIGINT NOT NULL PRIMARY KEY,
    "duration" INTEGER DEFAULT 60,
    "message" TEXT DEFAULT 'Denk an deine Traumroutine!'
);

-- CreateTable
CREATE TABLE "drinks" (
    "id" BIGINT NOT NULL PRIMARY KEY,
    "duration" INTEGER DEFAULT 60
);
