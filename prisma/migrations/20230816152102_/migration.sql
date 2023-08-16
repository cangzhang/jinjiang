-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Novel" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "title" TEXT NOT NULL,
    "novelId" INTEGER NOT NULL,
    "authorId" INTEGER NOT NULL,
    "listName" TEXT NOT NULL,
    "inList" BOOLEAN NOT NULL DEFAULT false,
    "createdAt" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" DATETIME
);
INSERT INTO "new_Novel" ("authorId", "createdAt", "id", "listName", "novelId", "title", "updatedAt") SELECT "authorId", "createdAt", "id", "listName", "novelId", "title", "updatedAt" FROM "Novel";
DROP TABLE "Novel";
ALTER TABLE "new_Novel" RENAME TO "Novel";
CREATE UNIQUE INDEX "Novel_novelId_key" ON "Novel"("novelId");
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
