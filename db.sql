PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE tmp_git_config (
    path        varchar(255) not null primary key, 
    is_selected tinyint(1) default 0, 
    UNIQUE(path)
);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Latex/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Latex/Fall2022/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Latex/Fall2022/IMO/Test1/rust/temp/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Latex/Knots/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Latex/NoteGoodwinStaton/',1);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Latex/PAPER_CURRENT_GammaFunction/',1);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Latex/QCE/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Latex/Replicators/',1);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Latex/Templates/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Latex/beamer-torino/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Latex/mHoTT/',1);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/i3-gnome-pomodoro-git/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/julia-bin/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/pandoc-bin/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/paperboy-bin/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/sc-im/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/st/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/surf/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/tabbed-git/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/tabbed-git/src/tabbed/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/tensorflow/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/termusic/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/tomatoshell/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/tomatoshell/src/tomatoshell/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/youtube-music-bin/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/ytui-music-bin/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/zeal/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Packages/zotero-bin/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Haskell/OperadTree/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Haskell/operad/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Julia/abm/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Python/MLDiff/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/Recycling/nav/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/celegans_aging/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/conway/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/crabs/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/deprecated_todo_tui/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/example_user_input/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/hecto/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/hello_world/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/list-projects/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/project-nav/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/templater/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/todo/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/tui-rs/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/wasm-game-of-life/www/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Programming/Rust/wyd/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Repos/dotfiles/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Repos/forgit/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/Science/CElegansAging/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/git/dotfiles/',0);
INSERT INTO tmp_git_config VALUES('/home/cuffaro/Documents/tmp/latex-snippets/',0);
CREATE TABLE project (
    id          integer primary key autoincrement, 
    path        varchar(255), 
    name        varchar(255), 
    desc        varchar(4000), 
    cat         varchar(255), 
    status      varchar(255),
    is_git      tinyint(1),
    owner       varchar(255),
    repo        varchar(255),
    last_commit varchar(255),
    UNIQUE(path)
);
INSERT INTO project VALUES(7,'/home/cuffaro/Documents/Programming/Rust/wyd/','wyd','application for organizing projects','Rust','Stable',1,'quffaro','wyd','2023-03-22T18:50:43Z');
INSERT INTO project VALUES(18,'/home/cuffaro/Documents/Latex/NoteGoodwinStaton/','NoteGoodwinStaton','N/A AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA','Math','Stable',1,'quffaro','NoteGoodwinStaton','N/A');
INSERT INTO project VALUES(19,'/home/cuffaro/Documents/Latex/PAPER_CURRENT_GammaFunction/','PAPER_CURRENT_GammaFunction',NULL,'Math','Unstable',1,'quffaro','PAPER_CURRENT_GammaFunction','N/A');
INSERT INTO project VALUES(20,'/home/cuffaro/Documents/Latex/mHoTT/','mHoTT',NULL,'Math','Unstable',1,'quffaro','mHoTT','N/A');
INSERT INTO project VALUES(21,'/home/cuffaro/Documents/Latex/Replicators/','Replicators',NULL,'Unknown','Stable',1,'quffaro','Replicators','N/A');
CREATE TABLE todo (
    id          integer primary key autoincrement,
    parent_id   integer,
    project_id  integer,
    todo        varchar(255),
    is_complete tinyint(1) default 0
);
INSERT INTO todo VALUES(1,0,7,'user needs to specify their search folder',1);
INSERT INTO todo VALUES(2,0,7,'user needs to specify their git config',0);
INSERT INTO todo VALUES(3,0,7,'user needs to search for TODO instances in their directory',0);
INSERT INTO todo VALUES(4,0,7,'todos need to start in Insert mode',1);
INSERT INTO todo VALUES(5,0,7,'move ui code to its own file',0);
INSERT INTO todo VALUES(6,0,7,'todo navigation is backwards',1);
INSERT INTO todo VALUES(7,0,7,'add task for project name',1);
INSERT INTO todo VALUES(8,0,7,'pressing h again should close popup',0);
INSERT INTO todo VALUES(9,0,7,'adding by config files should also guess owner/repo',1);
INSERT INTO todo VALUES(10,0,7,'wyd should detect pat.txt for requesting',0);
INSERT INTO todo VALUES(11,0,7,'i want a status for loading',0);
INSERT INTO todo VALUES(12,0,7,'mouse scrolling on task bar',1);
INSERT INTO todo VALUES(13,0,7,'sort by column',0);
INSERT INTO todo VALUES(14,0,7,'display project count',0);
INSERT INTO todo VALUES(15,0,7,'customize item categories',1);
INSERT INTO todo VALUES(16,0,7,'what happens if a path changes?',0);
INSERT INTO todo VALUES(17,0,7,'focus should be on config',0);
INSERT INTO todo VALUES(18,0,7,'pressing x also toggles tasks',1);
INSERT INTO todo VALUES(19,0,7,'divide search popup into a tree for navigating through directory',0);
CREATE TABLE category (
    id   integer primary key autoincrement,
    name varchar(255)
);
INSERT INTO category VALUES(1,'Rust');
INSERT INTO category VALUES(2,'ssdsdij');
INSERT INTO category VALUES(3,'OCaml');
DELETE FROM sqlite_sequence;
INSERT INTO sqlite_sequence VALUES('project',21);
INSERT INTO sqlite_sequence VALUES('todo',19);
INSERT INTO sqlite_sequence VALUES('category',3);
COMMIT;
