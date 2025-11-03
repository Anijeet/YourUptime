-- Your SQL goes here
CREATE TYPE "website_status" AS ENUM ('Up', 'Down', 'Unknown');

CREATE TABLE "website" (
    "id" TEXT NOT NULL,
    "url" TEXT NOT NULL,
    "time_added" TIMESTAMP(3) NOT NULL,
    CONSTRAINT "website_pkey" PRIMARY KEY ("id")
);

CREATE TABLE "region" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    CONSTRAINT "region_pkey" PRIMARY KEY ("id")
);

CREATE TABLE "website_tick" (
    "id" TEXT NOT NULL,
    "response_response_ms" INTEGER NOT NULL,
    "status" "website_status" NOT NULL,
    "region_id" TEXT NOT NULL,
    "website_id" TEXT NOT NULL,
    CONSTRAINT "website_tick_pkey" PRIMARY KEY ("id")
);

ALTER TABLE "website_tick" 
ADD CONSTRAINT "website_tick_region_id_fkey"
FOREIGN KEY ("region_id") REFERENCES "region" ("id") 
ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "website_tick" 
ADD CONSTRAINT "website_tick_website_id_fkey"
FOREIGN KEY ("website_id") REFERENCES "website" ("id") 
ON DELETE RESTRICT ON UPDATE CASCADE;
