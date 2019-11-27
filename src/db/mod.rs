use kvdb_rocksdb::{Database, DatabaseConfig};

// database columns
/// Column for State
pub const COL_STATE: Option<u32> = Some(0);
/// Column for Block headers
pub const COL_HEADERS: Option<u32> = Some(1);
/// Column for Block bodies
pub const COL_BODIES: Option<u32> = Some(2);
/// Column for Extras
pub const COL_EXTRA: Option<u32> = Some(3);
/// Column for MemPool Data
pub const COL_MEMPOOL: Option<u32> = Some(4);
/// Column for Transaction error hints
pub const COL_ERROR_HINT: Option<u32> = Some(5);
/// Number of columns in DB
pub const NUM_COLUMNS: Option<u32> = Some(6);

pub fn open(db_path: &str) -> Result<Database, String> {
    let db_config = DatabaseConfig::with_columns(NUM_COLUMNS);

    let db = Database::open(&db_config, db_path)
        .map_err(|_e| "Low level database error. Some issue with disk?".to_string())?;

    Ok(db)
}

pub fn print_all(db: &Database) {
    for i in 0..6 {
        println!("print_all {}", i);
        let it = db.iter(Some(i)).expect("print_all");
        for (k, v) in it {
            println!("K: {:?} / V: {:?}", k, v);
        }
    }
}

pub const VERSION_KEY_TENDERMINT_BACKUP: &[u8] = b"version_tendermint-backup";
const BACKUP_KEY: &[u8] = b"tendermint-backup";
pub fn print_tendermint_backup(db: &Database) {
    let tendermint_backup_version = db.get(COL_EXTRA, VERSION_KEY_TENDERMINT_BACKUP);
    println!("tendermint backup version {:?}", tendermint_backup_version);
}
