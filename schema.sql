CREATE TABLE IF NOT EXISTS tmp_git_config 
(path varchar(255) not null primary key
,is_selected tinyint(1) default 0);
CREATE TABLE IF NOT EXISTS project 
(id integer primary key autoincrement
,path varchar(255)
,name varchar(255)
,desc varchar(4000)
,status varchar(255)
,last_commit varchar(255)
,cat varchar(255));
CREATE TABLE IF NOT EXISTS todo 
(id integer primary key autoincrement
,parent_id integer
,project_id integer
,todo varchar(255)
,is_complete tinyint(1) default 0);

