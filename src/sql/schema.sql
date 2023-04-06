CREATE TABLE tmp_git_config (path varchar(255) not null primary key, is_selected tinyint(1) default 0);
CREATE TABLE project (path varchar(255) primary key, name varchar(255), cat varchar(255), status varchar(255), last_commit varchar(255));
CREATE TABLE todo (id integer primary key autoincrement, parent_id integer, project_id integer, todo varchar(255), is_complete tinyint(1) default 0);
CREATE TABLE sqlite_sequence(name,seq);
