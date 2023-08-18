-- CreateTable
CREATE TABLE "Novel" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "title" TEXT NOT NULL,
    "novelId" INTEGER NOT NULL,
    "authorId" INTEGER NOT NULL,
    "listName" TEXT NOT NULL,
    "inList" BOOLEAN NOT NULL DEFAULT false,
    "createdAt" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" DATETIME
);

-- CreateTable
CREATE TABLE "NovelStatistics" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "novelId" INTEGER NOT NULL,
    "firstChapterClicks" INTEGER NOT NULL DEFAULT 0,
    "lastChapterClicks" INTEGER NOT NULL DEFAULT 0,
    "reviews" INTEGER NOT NULL DEFAULT 0,
    "collected" INTEGER NOT NULL DEFAULT 0,
    "rewards" INTEGER NOT NULL DEFAULT 0,
    "createdAt" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" DATETIME
);

-- CreateTable
CREATE TABLE "ListLinks" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "link" TEXT NOT NULL,
    "name" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Novel_novelId_key" ON "Novel"("novelId");

-- CreateIndex
CREATE UNIQUE INDEX "ListLinks_link_key" ON "ListLinks"("link");
