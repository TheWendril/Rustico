CREATE TABLE "publishers" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"email"	TEXT NOT NULL,
	"access_level"	INTEGER NOT NULL,
	"password"	TEXT NOT NULL,
	"picture"	TEXT NOT NULL,
	"bio"	TEXT NOT NULL,
	"education"	TEXT NOT NULL,
	"age"	INTEGER NOT NULL,
	PRIMARY KEY("id")
);