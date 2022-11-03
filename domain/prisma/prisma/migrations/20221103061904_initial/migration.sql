-- CreateTable
CREATE TABLE "Model" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "spec" TEXT NOT NULL
);

-- CreateTable
CREATE TABLE "FixedInstance" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "model_id" TEXT NOT NULL,
    "driver_id" TEXT,
    "engine_id" TEXT,
    "config" TEXT NOT NULL,
    CONSTRAINT "FixedInstance_model_id_fkey" FOREIGN KEY ("model_id") REFERENCES "Model" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "FixedInstance_driver_id_fkey" FOREIGN KEY ("driver_id") REFERENCES "FixedInstanceDriver" ("id") ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT "FixedInstance_engine_id_fkey" FOREIGN KEY ("engine_id") REFERENCES "Engine" ("id") ON DELETE SET NULL ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "FixedInstanceDriver" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "url" TEXT NOT NULL,
    "last_seen" DATETIME,
    "config" TEXT NOT NULL
);

-- CreateTable
CREATE TABLE "Engine" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "url" TEXT NOT NULL,
    "last_seen" DATETIME,
    "config" TEXT NOT NULL
);

-- CreateTable
CREATE TABLE "MediaFile" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "path" TEXT,
    "metadata" TEXT,
    "last_used" DATETIME,
    "revision" BIGINT NOT NULL DEFAULT 0
);

-- CreateTable
CREATE TABLE "MediaJob" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "media_id" TEXT NOT NULL,
    "status" TEXT NOT NULL,
    "config" TEXT NOT NULL,
    "kind" TEXT NOT NULL,
    "in_progress" BOOLEAN NOT NULL DEFAULT false,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME NOT NULL,
    CONSTRAINT "MediaJob_media_id_fkey" FOREIGN KEY ("media_id") REFERENCES "MediaFile" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "Task" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "engine_id" TEXT,
    "spec" TEXT NOT NULL,
    CONSTRAINT "Task_engine_id_fkey" FOREIGN KEY ("engine_id") REFERENCES "Engine" ("id") ON DELETE SET NULL ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "SysProp" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "value" TEXT NOT NULL
);
