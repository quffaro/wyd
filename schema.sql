CREATE TABLE tmp_git_config (path varchar(255) not null primary key, is_selected tinyint(1) default 0);
CREATE TABLE project (id integer primary key autoincrement, path varchar(255), name varchar(255), status varchar(255), last_commit varchar(255), cat varchar(255));
CREATE TABLE sqlite_sequence(name,seq);
