use sqlx::{pool::PoolConnection, MySql, Transaction};

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
    /// The Ok variant will be returned if the insertion is successful.
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
