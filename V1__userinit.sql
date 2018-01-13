CREATE EXTENSION IF NOT EXISTS CITEXT;

SET SYNCHRONOUS_COMMIT = 'off';


CREATE TABLE IF NOT EXISTS userprofiles (
  id       SERIAL PRIMARY KEY,
  about    TEXT DEFAULT NULL,
  email    CITEXT UNIQUE,
  fullname TEXT DEFAULT NULL,
  nickname CITEXT COLLATE ucs_basic UNIQUE
);

create index if not exists user_name_idx on userprofiles(nickname);


----------------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS forums (
  id      SERIAL PRIMARY KEY,
  owner_id INTEGER REFERENCES userprofiles (id) ON DELETE CASCADE NOT NULL,
  owner_name CITEXT,
  title   TEXT NOT NULL,
  slug    CITEXT UNIQUE                                   NOT NULL,
  posts   INTEGER DEFAULT 0,
  threads INTEGER DEFAULT 0
);


create index if not exists forums_slug_idx on forums(slug);

CREATE INDEX IF NOT EXISTS forums_userprofiles_for_id_idx
ON forums (owner_id);

-------------------------------------------------------------------------------------


CREATE TABLE IF NOT EXISTS threads (
  id SERIAL PRIMARY KEY,
  author_id  INTEGER REFERENCES userprofiles (id) ON DELETE CASCADE  NOT NULL,
  author_name CITEXT,
  forum_id INTEGER REFERENCES forums (id) ON DELETE CASCADE NOT NULL,
  forum_slug CITEXT,
  title    TEXT  NOT NULL,
  created  TIMESTAMPTZ DEFAULT NOW(),
  message  TEXT        DEFAULT NULL,
  votes    INTEGER     DEFAULT 0,
  slug     CITEXT UNIQUE
);


CREATE INDEX IF NOT EXISTS threads_user_id_idx
  ON threads (author_id);
CREATE INDEX IF NOT EXISTS threads_forum_id_idx
ON threads (forum_id);

--------------------------------------------------------------------------------------


CREATE TABLE IF NOT EXISTS posts (
  id SERIAL PRIMARY KEY,
  parent_id    INTEGER     DEFAULT 0,
  author_id   INTEGER REFERENCES userprofiles (id) ON DELETE CASCADE   NOT NULL,
  author_name CITEXT,
  created   TIMESTAMPTZ DEFAULT NOW(),
  forum_id  INTEGER REFERENCES forums (id) ON DELETE CASCADE  NOT NULL,
  forum_slug CITEXT,
  is_edited BOOLEAN     DEFAULT FALSE,
  message   TEXT        DEFAULT NULL,
  thread_id INTEGER REFERENCES threads (id) ON DELETE CASCADE NOT NULL,
  id_of_root INTEGER,
  path_to_post INTEGER []
);



CREATE INDEX IF NOT EXISTS posts_user_id_idx
  ON posts (author_id);

CREATE INDEX IF NOT EXISTS posts_forum_id_idx
  ON posts (forum_id);

CREATE INDEX IF NOT EXISTS posts_flat_idx
  ON posts (thread_id, created, id);

CREATE INDEX IF NOT EXISTS posts_path_thread_id_idx
  ON posts (thread_id, path_to_post);

CREATE INDEX IF NOT EXISTS posts_path_help_idx
  ON posts (id_of_root, path_to_post);

CREATE INDEX IF NOT EXISTS posts_multi_idx
ON posts (thread_id, parent_id, id);

create index if not exists post_root ON posts(id_of_root);


CREATE TABLE IF NOT EXISTS forums_and_users (
  user_id INTEGER REFERENCES userprofiles (id) ON DELETE CASCADE NOT NULL,
  forum_id INTEGER REFERENCES forums (id) ON DELETE CASCADE NOT NULL
);

CREATE INDEX IF NOT EXISTS forum_users_user_id_idx
  ON forums_and_users (user_id);
CREATE INDEX IF NOT EXISTS forum_users_forum_id_idx
ON forums_and_users (forum_id);




CREATE TABLE IF NOT EXISTS votes (
  owner_id INTEGER REFERENCES userprofiles (id) ON DELETE CASCADE,
  thread_id  INTEGER REFERENCES threads (id) ON DELETE CASCADE,
  vote INTEGER DEFAULT 0,
  CONSTRAINT one_owner_thread_pair UNIQUE (owner_id, thread_id)
);



create index if not exists thread_vote on votes(thread_id);

CREATE OR REPLACE FUNCTION create_thread(u_name citext, created timestamptz, f_slug citext,
 message text, t_slug citext, title text)
  RETURNS INTEGER as '
  DECLARE
    u_id integer;
    t_id integer;
    f_id integer;
    forum_slug citext;
    u_nickname citext;
--     date TIMESTAMPTZ;
  BEGIN
      select id, nickname from userprofiles where nickname = u_name into u_id, u_nickname;
      select id, slug from forums where slug = f_slug into f_id, forum_slug;
      if created is null then
        created = NOW();
      end if;
      insert into threads(author_id, author_name, forum_id, forum_slug, title, created, message, slug)
      values(u_id, u_nickname, f_id, forum_slug, title, created, message, t_slug) returning id into t_id;
      update forums set threads = threads + 1 where id = f_id;
      insert into forums_and_users(forum_id, user_id) values(f_id, u_id);
      RETURN t_id;
   END;'
LANGUAGE plpgsql;

--------------------- TRIGGER FOR UPDAT..E forums ------------------------
CREATE OR REPLACE FUNCTION insert_forum_func() RETURNS TRIGGER AS
$insert_forums_trigger$
  BEGIN
    UPDATE forums SET owner_name = (SELECT nickname FROM userprofiles WHERE id = NEW.owner_id)
      WHERE id = NEW.id;
    RETURN NULL;
  END;
$insert_forums_trigger$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS insert_forums_trigger ON forums;
CREATE TRIGGER insert_forums_trigger AFTER INSERT ON forums
  FOR EACH ROW EXECUTE PROCEDURE insert_forum_func();
---------------------------------------------------------------------

CREATE OR REPLACE FUNCTION insert_posts_func() RETURNS TRIGGER AS
$insert_posts_trigger$
  DECLARE
    arr INTEGER[];
    root INTEGER;
  BEGIN
      IF NEW.parent_id = 0 THEN
       SELECT array_append(NULL, NEW.id) into arr;
       root = NEW.id;
      ELSE
        SELECT array_append((SELECT path_to_post from posts WHERE id = NEW.parent_id), NEW.id) into arr;
        root = arr[1];
      END IF;
      NEW.path_to_post = arr;
      NEW.id_of_root = root;
      UPDATE forums set posts = posts + 1 WHERE id = NEW.forum_id;
      INSERT INTO forums_and_users(user_id, forum_id) VALUES(NEW.author_id, NEW.forum_id);
    RETURN NEW;
  END;
$insert_posts_trigger$ LANGUAGE plpgsql;
--

DROP TRIGGER IF EXISTS insert_posts_trigger ON posts;
CREATE TRIGGER insert_posts_trigger BEFORE INSERT ON posts
  FOR EACH ROW EXECUTE PROCEDURE insert_posts_func();

---------------------- vote -----------------------------------------
CREATE OR REPLACE FUNCTION create_or_update_vote(u_id INTEGER, t_id INTEGER, v INTEGER)
  RETURNS VOID AS '
BEGIN
  INSERT INTO votes (owner_id, thread_id, vote) VALUES (u_id, t_id, v)
  ON CONFLICT (owner_id, thread_id)
    DO UPDATE SET vote = v;
  UPDATE threads
  SET votes = (SELECT SUM(vote)
               FROM votes
               WHERE thread_id = t_id)
  WHERE id = t_id;
END;'
LANGUAGE plpgsql;




-- old functions



-- DROP TRIGGER IF EXISTS insert_posts_trigger ON posts;
-- CREATE TRIGGER insert_posts_trigger AFTER INSERT ON posts
--   FOR EACH ROW EXECUTE PROCEDURE insert_posts_func();

-- -------------------------------------------------------------------
-- CREATE OR REPLACE FUNCTION create_or_update_vote(u_id integer, t_id integer, v integer)
--   RETURNS VOID as '
--   DECLARE
--     flag integer;
--   BEGIN
--     select 1 from tb2 where a = u_id and b = t_id into flag;
--     IF flag = 1 THEN
--       UPDATE tb2 SET c = v WHERE a = u_id and b = t_id;
--     ELSE
--       INSERT into tb2(a, b, c) VALUES(u_id, t_id, v);
--     END IF;
--   END;'
-- LANGUAGE plpgsql;




-- -- to_do trigger
-- CREATE OR REPLACE FUNCTION create_or_update_vote(u_id integer, t_id integer, v integer)
--   RETURNS VOID as '
--   DECLARE
-- --     flag integer;
--     old_vote integer;
--   BEGIN
--     select voice from votes where owner_id = u_id and thread_id = t_id into vote;
--     IF vote is null THEN
--       INSERT into votes(owner_id, thread_id, vote) VALUES(u_id, t_id, v);
--       UPDATE threads set votes = votes + v;'
--     ELSE
--       IF old_vote != vote
--
--       INSERT into votes(owner_id, thread_id, vote) VALUES(u_id, t_id, v);
--       UPDATE threads set votes = votes + v;
--     END IF;
--    END;'
-- LANGUAGE plpgsql;
-- -- CREATE PROCEDURE fill_created_forum();
-- --
-- -- CREATE PROCEDURE fill_r
-- --
-- -- CREATE PROCEDURE fill_inserted_post();
-- --
-- -- CREATE TRIGGER
--
--
--
--
-- --------------------- TRIGGER FOR UPDATE forums ------------------------
-- CREATE OR REPLACE FUNCTION insert_forum_func() RETURNS TRIGGER AS
-- $insert_forums_trigger$
--   BEGIN
--     UPDATE forums SET owner_name = (SELECT nickname FROM userprofiles WHERE id = NEW.owner_id)
--       WHERE id = NEW.id;
--     RETURN NULL;
--   END;
-- $insert_forums_trigger$ LANGUAGE plpgsql;
--
-- DROP TRIGGER IF EXISTS insert_forums_trigger ON forums;
-- CREATE TRIGGER insert_forums_trigger AFTER INSERT ON forums
--   FOR EACH ROW EXECUTE PROCEDURE insert_forum_func();
-- ---------------------------------------------------------------------
--
--
--
-- ------------------- TRIGGER FOR UPDATE threads ---------------
-- CREATE OR REPLACE FUNCTION insert_threads_func() RETURNS TRIGGER AS
-- $insert_threads_trigger$
--   BEGIN
--       UPDATE threads SET author_name = (SELECT nickname FROM userprofiles WHERE id = NEW.author_id),
--                       forum_slug = (SELECT slug FROM forums WHERE id = NEW.forum_id)
--       WHERE id = NEW.id;
--       UPDATE forums SET threads = threads + 1 WHERE id = NEW.forum_id;
--        INSERT INTO forums_and_users(user_id, forum_id) VALUES(NEW.author_id, NEW.forum_id);
--     RETURN NULL;
--   END;
-- $insert_threads_trigger$ LANGUAGE plpgsql;
--
-- DROP TRIGGER IF EXISTS insert_threads_trigger ON threads;
-- CREATE TRIGGER insert_threads_trigger AFTER INSERT ON threads
--   FOR EACH ROW EXECUTE PROCEDURE insert_threads_func();
--
-- -------------------------------------------------------------------
--
--
-- ------------------- TRIGGER FOR UPDATE posts ------------------------
--
-- -- CREATE OR REPLACE FUNCTION insert_posts_func() RETURNS TRIGGER AS
-- -- $insert_posts_trigger$
-- --   BEGIN
-- --       UPDATE posts SET author_name = (SELECT nickname FROM userprofiles WHERE id = NEW.author_id),
-- --                       forum_slug = (SELECT slug FROM forums WHERE id = NEW.forum_id)
-- --       WHERE id = NEW.id;
-- --       INSERT INTO forums_and_users(user_id, forum_id) VALUES(NEW.author_id, NEW.forum_id);
-- --     RETURN NULL;
-- --   END;
-- -- $insert_posts_trigger$ LANGUAGE plpgsql;
-- -- --
-- -- DROP TRIGGER IF EXISTS insert_posts_trigger ON posts;
-- -- CREATE TRIGGER insert_posts_trigger AFTER INSERT ON posts
-- --   FOR EACH ROW EXECUTE PROCEDURE insert_posts_func();
--
-- -- -------------------------------------------------------------------
-- -- CREATE OR REPLACE FUNCTION create_or_update_vote(u_id integer, t_id integer, v integer)
-- --   RETURNS VOID as '
-- --   DECLARE
-- --     flag integer;
-- --   BEGIN
-- --     select 1 from tb2 where a = u_id and b = t_id into flag;
-- --     IF flag = 1 THEN
-- --       UPDATE tb2 SET c = v WHERE a = u_id and b = t_id;
-- --     ELSE
-- --       INSERT into tb2(a, b, c) VALUES(u_id, t_id, v);
-- --     END IF;
-- --   END;'
-- -- LANGUAGE plpgsql;
--         SELECT path_to_post from posts WHERE id = NEW.parent_id into arr;
--        UPDATE posts SET path_to_post = array_append(arr, NEW.id), id_of_root = arr[1] WHERE id = NEW.id;




------------------- TRIGGER FOR UPDATE threads ---------------

/*CREATE OR REPLACE FUNCTION insert_threads_func() RETURNS TRIGGER AS
$insert_threads_trigger$
  BEGIN
      UPDATE threads SET author_name = (SELECT nickname FROM userprofiles WHERE id = NEW.author_id),
                      forum_slug = (SELECT slug FROM forums WHERE id = NEW.forum_id)
      WHERE id = NEW.id;
      UPDATE forums SET threads = threads + 1 WHERE id = NEW.forum_id;
       INSERT INTO forums_and_users(user_id, forum_id) VALUES(NEW.author_id, NEW.forum_id);
    RETURN NULL;
  END;
$insert_threads_trigger$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS insert_threads_trigger ON threads;
CREATE TRIGGER insert_threads_trigger AFTER INSERT ON threads
  FOR EACH ROW EXECUTE PROCEDURE insert_threads_func();*/

-------------------------------------------------------------------


------------------- TRIGGER FOR UPDATE posts ------------------------
--  author_name = (SELECT nickname FROM userprofiles WHERE id = NEW.author_id),
-- CREATE OR REPLACE FUNCTION insert_posts_func() RETURNS TRIGGER AS
-- $insert_posts_trigger$
--   DECLARE
--     arr INTEGER[];
--   BEGIN
--       IF NEW.parent_id = 0 THEN
--        UPDATE posts SET path_to_post = array_append(NULL, NEW.id), id_of_root = NEW.id WHERE id = NEW.id;
--       ELSE
--         SELECT path_to_post from posts WHERE id = NEW.parent_id into arr;
--        UPDATE posts SET path_to_post = array_append(arr, NEW.id), id_of_root = arr[1] WHERE id = NEW.id;
--       END IF;
--
-- --       WHERE id = NEW.id;
--       UPDATE forums set posts = posts + 1 WHERE id = NEW.forum_id;
--       INSERT INTO forums_and_users(user_id, forum_id) VALUES(NEW.author_id, NEW.forum_id);
--     RETURN NULL;
--   END;
-- $insert_posts_trigger$ LANGUAGE plpgsql;
