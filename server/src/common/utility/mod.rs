use std::env;

use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// # Description
///
/// Get the file path of the server's configuration file.
///
/// If arguments are passed in when executing the application, that will be used as the file path.
/// If arguments are not passed in when executing the application, this this will attempt to load
/// it from the present working directory.
///
/// Note that the returned path could be malformed or could be a path to a file that does not exist.
///
/// # Returns
///
/// The file path of the server's configuration file.
pub(crate) fn get_config_path() -> String {
    // Get arguments that were passed in when executing the application.
    let args: Vec<String> = env::args().collect();

    // If a configuration file path was specified return that, otherwise return the default path.
    if args.len() > 1 {
        // The first argument will always be the name of the file being executed, so we must skip it.
        return args.iter().skip(1).map(|s| s.chars()).flatten().collect();
    } else {
        return String::from("config.json");
    }
}

/// # Description
///
/// Create a string with randomly generated characters.
///
/// # Arguments
///
/// `size` - The amount of characters to include in the randomly generated string.
///
/// # Returns
///
/// The string that was randomly generated.
pub(crate) fn generate_random_string(size: usize) -> String {
    let mut rng = thread_rng();

    return (&mut rng)
        .sample_iter(Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();
}
