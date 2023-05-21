use crate::feature::user::model::User;
use sqlx::{pool::PoolConnection, MySql, Transaction};
use std::error::Error;

/// # Description
///
/// An enumeration of all of the possible contexts a query can be executed in.
pub(crate) enum QueryContext<'a> {
    /// # Description
    ///
    /// A connection context variant. This variant is typically used when a query can be executed
    /// independent from all other queries. If several queries need to be executed atomically
    /// (i.e. if one fails, they all fail), the Transaction variant should be used.
    Connection(PoolConnection<MySql>),

    /// # Description
    ///
    /// A transaction context variant. This variant is typically used when several queries need to be
    /// executed atomically (i.e. if one fails, they all fail). This has more overhead than
    /// the Connection variant, and should only be used if several queries must be atomic.
    Transaction(Transaction<'a, MySql>),
}

/// An implementation for the QueryContext enum.
impl<'a> QueryContext<'a> {
    /// # Description
    ///
    /// If the context is a transaction, commit the changes made in this context.
    ///
    /// ### Note: If the context is not a transaction, this will not do anything.
    ///
    /// # Returns
    ///
    /// This functions returns a result:
    /// - If the context is a connection or is a transaction and was successfully committed, the Ok
    /// variant will be returned.
    /// - If the context is a transaction and failed to commit, the Err variant will be returned with
    /// the error that occurred.
    pub(crate) async fn commit_if_transaction(self) -> Result<(), sqlx::error::Error> {
        if let QueryContext::Transaction(transaction) = self {
            return transaction.commit().await;
        } else {
            return Ok(());
        }
    }

    /// # Description
    ///
    /// If the context is a transaction, rollback the changes made in this context.
    ///
    /// ### Note: If the context is not a transaction, this will not do anything.
    ///
    /// This functions returns a result:
    /// - If the context is a connection or is a transaction and was successfully rolled back, the Ok
    /// variant will be returned.
    /// - If the context is a transaction and failed to rollback, the Err variant will be returned with
    /// the error that occurred.
    pub(crate) async fn rollback_if_transaction(self) -> Result<(), sqlx::error::Error> {
        if let QueryContext::Transaction(transaction) = self {
            return transaction.rollback().await;
        } else {
            return Ok(());
        }
    }
}

/// # Description
///
/// An enumeration of all of the possible authentication results.
pub(crate) enum AuthenticationResult {
    /// # Description
    ///
    /// The Ok variant will be returned if authentication was successful.
    Ok(User),

    /// # Description
    ///
    /// The NotAuthenticated variant will be returned if authentication was not successful.
    NotAuthenticated,

    /// # Description
    ///
    /// The Err variant will be returned if an error occurs during the authentication process.
    Err(Box<dyn Error>),
}

/// # Description
///
/// An enumeration of all of the possible results of an insertion.
///
/// # Generic Parameters
///
/// `T` - The datatype that will be returned if the insertion was successful.
///
/// `I` - The datatype that will be returned if data validation errors occur.
///
/// `E` - The datatype that will be returned if unexpected errors occurs.
pub(crate) enum InsertionResult<T, I, E> {
    /// # Description
    ///
    /// The Ok variant will be returned if the insertion was successful.
    Ok(T),

    /// # Description
    ///
    /// The Invalid variant will be returned if validation errors occur.
    Invalid(I),

    /// # Description
    ///
    /// The Err variant will be returned if unexpected errors occur.
    Err(E),
}

/// # Description
///
/// An enumeration of all of the possible results of a query.
///
/// # Generic Parameters
///
/// `T` - The datatype that will be returned if the query was successful.
///
/// `E` - The datatype that will be returned if an unexpected errors occurs.
pub(crate) enum QueryResult<T, E> {
    /// # Description
    ///
    /// The Ok variant will be returned if the query was successful.
    Ok(T),

    /// # Description
    ///
    /// The NotFound variant will be returned if the query did not return any results.
    NotFound,

    /// # Description
    ///
    /// The Err variant will be returned if unexpected errors occur.
    Err(E),
}

/// # Description
///
/// An enumeration of all of the possible results of an update.
///
/// # Generic Parameters
///
/// `T` - The datatype that will be returned if the update was successful.
///
/// `I` - The datatype that will be returned if data validation errors occur.
///
/// `E` - The datatype that will be returned if an unexpected errors occurs.
pub(crate) enum UpdateResult<T, I, E> {
    /// # Description
    ///
    /// The Ok variant will be returned if the update was successful.
    Ok(T),

    /// # Description
    ///
    /// The NotFound variant will be returned if the entity being updated could not be found.
    NotFound,

    /// # Description
    ///
    /// The Invalid variant will be returned if validation errors occur.
    Invalid(I),

    /// # Description
    ///
    /// The Err variant will be returned if unexpected errors occur.
    Err(E),
}

/// # Description
///
/// An enumeration of all of the possible results of a deletion.
///
/// # Generic Parameters
///
/// `E` - The datatype that will be returned if an unexpected errors occurs.
pub(crate) enum DeletionResult<E> {
    /// # Description
    ///
    /// The Ok variant will be returned if the deletion was successful.
    Ok,

    /// # Description
    ///
    /// The NotFound variant will be returned if the entity being deleted could not be found.
    NotFound,

    /// # Description
    ///
    /// The Err variant will be returned if unexpected errors occur.
    Err(E),
}
