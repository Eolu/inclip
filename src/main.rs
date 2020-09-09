use std::env;
use std::error::Error;
use std::io::{stdin, stdout, Write, Read};
use std::iter::once;
use std::path::Path;
use std::process::Command;
use clap::{Arg, App, SubCommand};
use clipboard::{ClipboardProvider, ClipboardContext};
use tempfile::NamedTempFile;

// Platform diff command
#[cfg(not(windows))]
const DIFF_COMMAND: &str = "diff";
#[cfg(windows)]
const DIFF_COMMAND: &str = "fc.exe";

/// Entry-point
fn main() -> Result<(), Box<dyn Error>>
{
    std::process::exit(inclip()?);
}

/// Main program
fn inclip() -> Result<i32, Box<dyn Error>>
{
    // Parse args
    let args = App::new("inclip")
        .version("1.0.0")
        .usage("inclip \n    inclip diff [file] [args...]")
        .author("Griffin O'Neill <gsoneill1003@gmail.com>")
        .about("Echo clipboard contents")
        .subcommand(SubCommand::with_name("diff")
                    .about("Compare clipboard contents")
                    .arg(Arg::with_name("file")
                        .index(1)))
        .get_matches();

    // Get clipboard contents
    let mut context = ClipboardProvider::new()?;

    // Determine whether this is an echo or a diff
    Ok(if let Some(args) = args.subcommand_matches("diff") 
    {
        // Make sure there's something in the clipboard
        let clipboard_contents = match get_clip_contents(&mut context)
        {
            Ok(None) => Some(wait_for_clipboard(&mut context)?),
            contents => contents?
        };

        // Output current clip contents to a temp file
        let file_1 = match clipboard_contents
        {
            Some(contents) => write_temp_file(&contents),
            None => panic!("Unexpected empty clipboard")
        }?;

        // Determine whether or not to compare with an existing file
        match args.value_of("file")
        {
            // File path specified
            Some(arg) if Path::new(arg).is_file() => 
            {
                diff(file_1.path(), Path::new(arg), env::args().skip(3))?
            },
            // No file, expecting more clipboard input
            _ => 
            {
                let contents = wait_for_clipboard(&mut context)?;
                diff(file_1.path(), write_temp_file(&contents)?.path(), env::args().skip(2))?
            }
        }
    }
    else
    {
        if let Some(contents) = get_clip_contents(&mut context)?
        {
            println!("{}", contents)
        }

        0
    })
}

/// Perform a diff
fn diff(path1: &Path, path2: &Path, args: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>>
{
    let args = once(path1.as_os_str().to_os_string())
        .chain(once(path2.as_os_str().to_os_string()))
        .chain(args.map(std::ffi::OsString::from));

    match Command::new(DIFF_COMMAND).args(args).status()?
    {
        exit_status => match exit_status.code()
        {
            Some(code) => Ok(code),
            None => Ok(0)
        }
    }
}

/// Write a string to a temporary file
fn write_temp_file(string: &str) -> Result<NamedTempFile, std::io::Error>
{                
    let mut temp_file = NamedTempFile::new()?;
    write!(temp_file, "{}", string)?;
    Ok(temp_file)
}

/// Get the contents of the clipboard
fn get_clip_contents(context: &mut ClipboardContext) -> Result<Option<String>, Box<dyn Error>>
{
    match context.get_contents()
    {
        Ok(contents) => Ok(Some(contents)),
        Err(err) if (*err).to_string() == "The operation completed successfully. (os error 0)" => Ok(None),
        Err(err) => Err(err)
    }
}

/// Pause so something can be placed on the clipboard
fn wait_for_clipboard(mut context: &mut ClipboardContext) -> Result<String, Box<dyn Error>>
{
    let mut clipboard_contents = Ok(None);
    while let Ok(None) = clipboard_contents
    {
        let mut stdout = stdout();
        stdout.write(b"Put something on the clipboard and press enter to continue.")?;
        stdout.flush()?;
        stdin().read(&mut [0])?;
        clipboard_contents = get_clip_contents(&mut context);
    }
    Ok(clipboard_contents?.expect("Unexpected empty clipboard"))
}