
use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::io::{stdin, stdout, Write, Read};
use clap::{Arg, App, SubCommand};
use clipboard::{ClipboardProvider, ClipboardContext};
use tempfile::NamedTempFile;

type ClipResult = Result<Option<String>, Box<dyn Error>>;

fn main() -> Result<(), Box<dyn Error>>
{
    // Parse args
    let args = App::new("clip")
        .version("0.1")
        .author("Griffin O'Neill <gsoneill1003@gmail.com>")
        .about("Echo clipboard contents")
        .subcommand(SubCommand::with_name("diff")
                    .about("Compare clipboard contents")
                    .arg(Arg::with_name("file")
                        .index(1)))
        .get_matches();

    // Get clipboard contents
    let mut context: ClipboardContext = ClipboardProvider::new().unwrap();

    // Determine whether this is an echo or a diff
    if let Some(args) = args.subcommand_matches("diff") 
    {
        // Make sure there's something in the clipboard
        let clipboard_contents = match get_clip_contents(&mut context)
        {
            Ok(None) => wait_for_clipboard(&mut context),
            contents => contents
        };

        // Output current clip contents to a temp file
        let file_1 = match clipboard_contents?
        {
            Some(contents) => write_temp_file(&contents),
            None => panic!("Unexpected empty clipboard")
        }?;
        let file_1_path = file_1.path();

        // Determine whether or not to compare with an existing file
        Ok(match args.value_of("file")
        {
            Some(file) => 
            {
                diff(file_1_path, Path::new(file))?
            },
            None => match wait_for_clipboard(&mut context)?
            {
                Some(contents) =>
                {
                    diff(file_1_path, write_temp_file(&contents)?.path())?
                },
                None => panic!("Unexpected empty clipboard")
            }
        })
    }
    else
    {
        Ok(match get_clip_contents(&mut context)?
        {
            Some(contents) => println!("{}", contents),
            None => ()
        })
    }
}

/// Perform a diff. Uses unix diff
#[cfg(not(windows))]
fn diff(path1: &Path, path2: &Path) -> std::io::Result<()>
{
    Command::new("diff")
        .args(&[path1, path2])
        .status()
        .map(|_| ())
}

/// Perform a diff. Uses Windows fc
#[cfg(windows)]
fn diff(path1: &Path, path2: &Path) -> std::io::Result<()>
{
    Command::new("fc")
        .args(&[path1, path2])
        .status()
        .map(|_|())
}

/// Write a string to a temporary file
fn write_temp_file(string: &str) -> Result<NamedTempFile, std::io::Error>
{                
    let mut temp_file = NamedTempFile::new()?;
    write!(temp_file, "{}", string);
    Ok(temp_file)
}

/// Get the contents of the clipboard
fn get_clip_contents(context: &mut ClipboardContext) -> ClipResult
{
    match context.get_contents()
    {
        Ok(contents) => Ok(Some(contents)),
        Err(err) if (*err).to_string() == "The operation completed successfully. (os error 0)" => Ok(None),
        Err(err) => Err(err)
    }
}

/// Pause so something can be placed on the clipboard
fn wait_for_clipboard(mut context: &mut ClipboardContext) -> ClipResult
{
    let mut clipboard_contents: ClipResult = Ok(None);
    while let Ok(None) = clipboard_contents
    {
        let mut stdout = stdout();
        stdout.write(b"Put something on the clipboard and press enter to continue...").unwrap();
        stdout.flush().unwrap();
        stdin().read(&mut [0]).unwrap();
        clipboard_contents = get_clip_contents(&mut context);
    }
    clipboard_contents
}