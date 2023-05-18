use sqlx::{pool::PoolConnection, MySql, Transaction};

/// # Description
///
/// An enumeration of all of the possible contexts a query can be executed in.
pub(crate) enum QueryContext<'a> {
    /// A connection context variant. This variant is typically used when a query can be executed
    /// independent from all other queries. If several queries need to be executed atomically
    /// (i.e. if one fails, they all fail), the Transaction variant should be used.
    Connection(PoolConnection<MySql>),
    /// A transaction context variant. This variant is typically used when several queries need to be
    /// executed atomically (i.e. if one fails, they all fail). This has more overhead than
    /// the Connection variant, and should only be used if several queries must be atomic.
    Transaction(Transaction<'a, MySql>),
}
