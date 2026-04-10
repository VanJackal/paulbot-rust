CREATE TABLE cats (id INTEGER PRIMARY KEY, name TEXT);
CREATE TABLE images (id INTEGER PRIMARY KEY, url TEXT, cat_id INTEGER, FOREIGN KEY(cat_id) REFERENCES cats(id));
INSERT INTO cats (id, name) VALUES ('1', 'Oliver');
INSERT INTO images (id, url, cat_id) VALUES ('1', 'https://upload.wikimedia.org/wikipedia/commons/thumb/b/b6/Felis_catus-cat_on_snow.jpg/1920px-Felis_catus-cat_on_snow.jpg', '1');