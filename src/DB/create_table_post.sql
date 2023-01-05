CREATE TABLE "posts" (
	"title"	TEXT NOT NULL,
	"image"	TEXT NOT NULL,
	"content"	TEXT NOT NULL,
	"author"	INTEGER NOT NULL,
	"tag"	TEXT NOT NULL,
	"likes"	INTEGER NOT NULL,
	PRIMARY KEY("title")
);