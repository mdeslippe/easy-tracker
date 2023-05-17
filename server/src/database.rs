use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::{pool::PoolConnection, MySql, Pool};

/// A database connection factory trait.
#[async_trait]
pub(crate) trait DatabaseConnectionFactory: Interface {
    /// # Description
    ///
    /// Retrieves a connection from the pool.
    ///
    /// The total time this method is allowed to execute is capped by
    /// [`PoolOptions::acquire_timeout`].
    /// If that timeout elapses, this will return [`Error::PoolClosed`].
    ///
    /// ### Note: Cancellation/Timeout May Drop Connections
    /// If `get_connection` is cancelled or times out after it acquires a connection from the idle
    /// queue or opens a new one, it will drop that connection because we don't want to assume it
    /// is safe to return to the pool, and testing it to see if it's safe to release could introduce
    /// subtle bugs if not implemented correctly. To avoid that entirely, we've decided to not
    /// gracefully handle cancellation here.
    ///
    /// This should eliminate any potential `.await` points between acquiring a connection and
    /// returning it.
    ///
    /// # Returns
    ///
    /// This method returns a result:
    /// - If a connection was successfully acquired, the Ok variant with the connection will be returned.
    /// - If an error occurred while attempting to acquire a connection, the Err variant with the error
    /// that occurred will be returned.
    async fn get_connection(&self) -> Result<PoolConnection<MySql>, sqlx::Error>;
}

/// A database connection factory implementation struct.
#[derive(Component)]
#[shaku(interface = DatabaseConnectionFactory)]
pub(crate) struct DatabaseConnectionFactoryImpl {
    /// The connection pool that will be used to acquire connections.
    connection_pool: Pool<MySql>,
}

/// A DatabaseConnectionFactory implementation for the DatabaseConnectionFactoryImpl struct.
#[async_trait]
impl DatabaseConnectionFactory for DatabaseConnectionFactoryImpl {
    async fn get_connection(&self) -> Result<PoolConnection<MySql>, sqlx::Error> {
        return self.connection_pool.acquire().await;
    }
}

// An implementation for the DatabaseConnectionFactoryImpl struct.
impl DatabaseConnectionFactoryImpl {
    /// # Description
    ///
    /// Create database connection factory parameters that can be used to override the default
    /// values injected.
    ///
    /// ### Note: This is necessary as the Shaku macro does not make them accessible outside of
    /// this file.
    ///
    /// # Arguments
    ///
    /// `pool` - The connection pool instance that will be used to override the default.
    ///
    /// # Returns
    ///
    /// The database connection factory parameters created.
    pub(crate) fn create_parameters(pool: Pool<MySql>) -> DatabaseConnectionFactoryImplParameters {
        return DatabaseConnectionFactoryImplParameters {
            connection_pool: pool,
        };
    }
}
