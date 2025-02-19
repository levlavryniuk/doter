// 0. Auto detect .config/doter or creat it

// 1. Auto detect .config/doter/index.toml or create it
// 2. Add entries using clap
// add backup folders.
// make local and remote dir

use rusqlite::Connection;

mod commands;
mod config;
mod git;
mod marker_manager;

const MAKERS_TBL: &str = "
create table if not exists markers (
    alias varchar(255) primary key,
    source_location varchar(255),
    copy_location varchar(255)
);
";

const VARS_TBL: &str = "
create table if not exists variables (
    name varchar(255) primary key,
    value varchar(255)
);
";

fn create_tables(conn: &Connection) {
    conn.execute(MAKERS_TBL, []).unwrap();
    conn.execute(VARS_TBL, []).unwrap();
}

fn main() {
    let mut cfg = config::Config::new();

    let conn = Connection::open(&cfg.doter_file_path).unwrap();

    create_tables(&conn);

    cfg.load_vars(&conn);

    let tmp = cfg.clone();

    let mut mgr = marker_manager::MarkerManager::new(&conn, &tmp);

    let command_mgr = commands::CommandManager::new();
    let command = command_mgr.create_command();

    let matches = command.get_matches();
    command_mgr.handle(&matches, &mut mgr, &mut cfg);
}
