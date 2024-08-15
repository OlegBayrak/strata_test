use std::sync::Arc;

use rockbound::rocksdb;
use tempfile::TempDir;

use alpen_express_db::database::CommonDatabase;

use crate::{l2::db::L2Db, ChainStateDb, ClientStateDb, L1Db, OptimisticDb, SyncEventDb};

pub fn get_rocksdb_tmp_instance() -> anyhow::Result<Arc<OptimisticDb>> {
    let dbname = crate::ROCKSDB_NAME;
    let cfs = crate::STORE_COLUMN_FAMILIES;
    let mut opts = rocksdb::Options::default();
    opts.create_missing_column_families(true);
    opts.create_if_missing(true);

    let temp_dir = TempDir::new().expect("failed to create temp dir");

    let rbdb = rockbound::OptimisticTransactionDB::open(
        temp_dir.into_path(),
        dbname,
        cfs.iter().map(|s| s.to_string()),
        &opts,
    )?;

    let db_ops = Arc::new(OptimisticDb {
        db: rbdb,
        retry_count: 5,
    });

    Ok(db_ops)
}

pub fn get_common_db() -> Arc<CommonDatabase<L1Db, L2Db, SyncEventDb, ClientStateDb, ChainStateDb>>
{
    let rbdb = get_rocksdb_tmp_instance().unwrap();
    let l1_db = Arc::new(L1Db::new(rbdb.clone()));
    let l2_db = Arc::new(L2Db::new(rbdb.clone()));
    let sync_ev_db = Arc::new(SyncEventDb::new(rbdb.clone()));
    let cs_db = Arc::new(ClientStateDb::new(rbdb.clone()));
    let chst_db = Arc::new(ChainStateDb::new(rbdb));

    Arc::new(CommonDatabase::new(
        l1_db, l2_db, sync_ev_db, cs_db, chst_db,
    ))
}
